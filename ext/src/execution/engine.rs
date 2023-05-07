use crate::execution::{CoerceResult, ExecutionError, FieldError, KeyStore};
use crate::ruby_api::{
    BaseInputTypeReference, BaseOutputTypeReference, CoerceInput, ExecutionResult, FieldDefinition,
    InputTypeReference, InputValueDefinition, InterfaceTypeDefinition, ObjectTypeDefinition,
    OutputTypeReference, SchemaDefinition, TypeDefinitionReference, UnionTypeDefinition,
};
use bluejay_core::definition::{
    AbstractOutputTypeReference, OutputTypeReference as CoreOutputTypeReference,
};
use bluejay_core::executable::AbstractOperationDefinition;
use bluejay_core::{AbstractTypeReference, AsIter, Directive as CoreDirective, OperationType};
use bluejay_parser::ast::executable::{ExecutableDocument, Field, OperationDefinition, Selection};
use bluejay_parser::ast::{Directive, VariableArguments, VariableValue};
use magnus::{ArgList, Error, RArray, RHash, Value, QNIL};
use std::collections::{BTreeMap, HashSet};

pub struct Engine<'a> {
    schema: &'a SchemaDefinition,
    document: &'a ExecutableDocument<'a>,
    variables: &'a RHash,
    key_store: KeyStore<'a>,
}

type MergedSelectionSets<'a, 'b> = std::iter::FlatMap<
    std::slice::Iter<'b, &'a Field<'a>>,
    &'a [Selection<'a>],
    fn(&&'a Field) -> &'a [Selection<'a>],
>;

impl<'a> Engine<'a> {
    pub fn execute_request(
        schema: &SchemaDefinition,
        query: &str,
        operation_name: Option<&str>,
        variable_values: RHash,
        initial_value: Value,
    ) -> Result<ExecutionResult, Error> {
        let document = match ExecutableDocument::parse(query) {
            Ok(document) => document,
            Err(parse_errors) => {
                return Ok(Self::execution_result(
                    Default::default(),
                    parse_errors
                        .into_iter()
                        .map(ExecutionError::ParseError)
                        .collect(),
                ));
            }
        };

        let operation_definition = match Self::get_operation(&document, operation_name) {
            Ok(od) => od,
            Err(error) => {
                return Ok(Self::execution_result(Default::default(), vec![error]));
            }
        };

        let variables =
            match Self::get_variable_values(schema, operation_definition, variable_values) {
                Ok(cvv) => cvv,
                Err(errors) => {
                    return Ok(Self::execution_result(Default::default(), errors));
                }
            };

        let instance = Engine {
            schema,
            document: &document,
            variables: &variables,
            key_store: KeyStore::new(),
        };

        match operation_definition.as_ref().operation_type() {
            OperationType::Query => {
                let query_root = initial_value.funcall("query", ())?;
                Ok(instance.execute_query(operation_definition, query_root))
            }
            OperationType::Mutation => {
                unimplemented!("executing mutations has not been implemented yet")
            }
            OperationType::Subscription => unreachable!(),
        }
    }

    fn get_operation<'b>(
        document: &'b ExecutableDocument,
        operation_name: Option<&'b str>,
    ) -> Result<&'b OperationDefinition<'b>, ExecutionError<'b>> {
        if let Some(operation_name) = operation_name {
            document
                .operation_definitions()
                .iter()
                .find(|od| matches!(od.as_ref().name(), Some(n) if n == operation_name))
                .ok_or(ExecutionError::NoOperationWithName {
                    name: operation_name,
                })
        } else if document.operation_definitions().len() == 1 {
            Ok(&document.operation_definitions()[0])
        } else {
            Err(ExecutionError::CannotUseAnonymousOperation)
        }
    }

    fn get_variable_values<'b>(
        schema: &'b SchemaDefinition,
        operation: &'b OperationDefinition<'b>,
        variable_values: RHash,
    ) -> Result<RHash, Vec<ExecutionError<'b>>> {
        let coerced_variables = RHash::new();
        let mut errors: Vec<ExecutionError<'b>> = Vec::new();

        if let Some(variable_definitions) = operation.as_ref().variable_definitions() {
            for variable_definition in variable_definitions.iter() {
                let variable_name = variable_definition.variable().name();
                let variable_named_type_reference = variable_definition.r#type();
                let variable_base_type = schema
                    .r#type(variable_named_type_reference.as_ref().name())
                    .unwrap();
                let base_input_type_reference: BaseInputTypeReference =
                    variable_base_type.try_into().unwrap();
                let variable_type = InputTypeReference::from_parser_type_reference(
                    variable_named_type_reference,
                    base_input_type_reference,
                );
                let default_value = variable_definition.default_value();
                let value = variable_values.get(variable_name);
                let has_value = value.is_some();
                let path = vec![variable_name.to_owned()];
                match default_value {
                    Some(default_value) if !has_value => {
                        match variable_type.coerce_parser_value(default_value, &path, &()) {
                            Ok(Ok(coerced_value)) => {
                                coerced_variables
                                    .aset(variable_name, coerced_value)
                                    .unwrap();
                            }
                            Ok(Err(coercion_errors)) => errors.extend(
                                coercion_errors
                                    .into_iter()
                                    .map(ExecutionError::CoercionError),
                            ),
                            Err(error) => errors.push(ExecutionError::ApplicationError(error)),
                        }
                    }
                    _ => {
                        if variable_type.is_required() && !has_value {
                            errors.push(ExecutionError::RequiredVariableMissingValue {
                                name: variable_name,
                            });
                        } else {
                            let value = value.unwrap_or_default();
                            match variable_type.coerce_ruby_const_value(value, &path) {
                                Ok(Ok(coerced_value)) => {
                                    coerced_variables
                                        .aset(variable_name, coerced_value)
                                        .unwrap();
                                }
                                Ok(Err(coercion_errors)) => {
                                    errors.extend(
                                        coercion_errors
                                            .into_iter()
                                            .map(ExecutionError::CoercionError),
                                    );
                                }
                                Err(error) => errors.push(ExecutionError::ApplicationError(error)),
                            }
                        }
                    }
                }
            }
        }

        if errors.is_empty() {
            Ok(coerced_variables)
        } else {
            Err(errors)
        }
    }

    fn execution_result(value: Value, errors: Vec<ExecutionError>) -> ExecutionResult {
        ExecutionResult::new(value, errors)
    }

    fn execute_query(
        &'a self,
        query: &'a OperationDefinition,
        initial_value: Value,
    ) -> ExecutionResult {
        let query_type = self.schema.query();
        let query_type = query_type.get();
        let selection_set = query.as_ref().selection_set();

        let (value, errors) =
            self.execute_selection_set(selection_set.as_ref().iter(), query_type, initial_value);

        Self::execution_result(value, errors)
    }

    fn execute_selection_set(
        &'a self,
        selection_set: impl Iterator<Item = &'a Selection<'a>>,
        object_type: &ObjectTypeDefinition,
        object_value: Value,
    ) -> (Value, Vec<ExecutionError<'a>>) {
        let mut visited_fragments = HashSet::new();
        let grouped_field_set =
            self.collect_fields(object_type, selection_set, &mut visited_fragments);

        let result_map = RHash::new();
        let mut errors = Vec::new();
        let mut has_null_for_required = false;

        for (response_key, fields) in grouped_field_set {
            let field_name = fields.first().unwrap().name().as_ref();
            let field_definition = object_type.field_definition(field_name).unwrap();
            let (response_value, mut errs) =
                self.execute_field(object_type, object_value, field_definition, &fields);
            if field_definition.r#type().as_ref().is_required() && response_value.is_nil() {
                has_null_for_required = true;
            }
            let key = if response_key == field_name {
                field_definition.name_r_string()
            } else {
                self.key_store.get(response_key)
            };
            result_map.aset(key, response_value).unwrap();
            errors.append(&mut errs);
        }

        if has_null_for_required {
            (*QNIL, errors)
        } else {
            (*result_map, errors)
        }
    }

    fn collect_fields(
        &'a self,
        object_type: &ObjectTypeDefinition,
        selection_set: impl Iterator<Item = &'a Selection<'a>>,
        visited_fragments: &mut HashSet<&'a str>,
    ) -> BTreeMap<&'a str, Vec<&'a Field>> {
        let mut grouped_fields: BTreeMap<&'a str, Vec<&'a Field>> = BTreeMap::new();

        for selection in selection_set {
            let should_skip = selection.directives().iter().any(|directive| {
                if directive.name().as_ref() == "skip" {
                    self.coerce_directive(directive)
                        .map(|coerced_directive| -> bool {
                            coerced_directive.funcall("if_arg", ()).unwrap()
                        })
                        .unwrap_or(false)
                } else {
                    false
                }
            });

            let should_include = selection.directives().iter().all(|directive| {
                if directive.name().as_ref() == "include" {
                    self.coerce_directive(directive)
                        .map(|coerced_directive| -> bool {
                            coerced_directive.funcall("if_arg", ()).unwrap()
                        })
                        .unwrap_or(true)
                } else {
                    true
                }
            });

            if should_skip || !should_include {
                continue;
            }

            match selection {
                Selection::Field(field) => {
                    let response_key = field.response_key();
                    let entry_for_response_key = grouped_fields.entry(response_key).or_default();
                    entry_for_response_key.push(field);
                }
                Selection::FragmentSpread(fragment_spread) => {
                    let fragment_spread_name = fragment_spread.name().as_ref();
                    if visited_fragments.insert(fragment_spread_name) {
                        let fragment = self
                            .document
                            .fragment_definitions()
                            .iter()
                            .find(|fd| fd.name().as_ref() == fragment_spread_name);

                        let fragment = match fragment {
                            Some(f) => f,
                            None => {
                                continue;
                            }
                        };

                        let fragment_type_name = fragment.type_condition().named_type().as_ref();

                        if !self.does_fragment_type_apply(object_type, fragment_type_name) {
                            continue;
                        }

                        let fragment_selection_set = fragment.selection_set();

                        let fragment_grouped_field_set = self.collect_fields(
                            object_type,
                            fragment_selection_set.as_ref().iter(),
                            visited_fragments,
                        );

                        for (response_key, fragment_group) in &fragment_grouped_field_set {
                            let group_for_response_key =
                                grouped_fields.entry(response_key).or_default();
                            group_for_response_key.extend_from_slice(fragment_group);
                        }
                    }
                }
                Selection::InlineFragment(inline_fragment) => {
                    let fragment_type = inline_fragment.type_condition();

                    if matches!(fragment_type, Some(fragment_type) if !self.does_fragment_type_apply(object_type, fragment_type.named_type().as_ref()))
                    {
                        continue;
                    }

                    let fragment_selection_set = inline_fragment.selection_set();

                    let fragment_grouped_field_set = self.collect_fields(
                        object_type,
                        fragment_selection_set.as_ref().iter(),
                        visited_fragments,
                    );

                    for (response_key, fragment_group) in &fragment_grouped_field_set {
                        let group_for_response_key =
                            grouped_fields.entry(response_key).or_default();
                        group_for_response_key.extend_from_slice(fragment_group);
                    }
                }
            }
        }

        grouped_fields
    }

    fn does_fragment_type_apply(
        &'a self,
        object_type: &ObjectTypeDefinition,
        fragment_type_name: &str,
    ) -> bool {
        let fragment_type = self.schema.r#type(fragment_type_name).unwrap();

        match fragment_type {
            TypeDefinitionReference::Object(otd) => {
                // TODO: see if there's any edge case where name equality does not work
                otd.as_ref().name() == object_type.name()
            }
            TypeDefinitionReference::Interface(itd) => {
                object_type.implements_interface(itd.as_ref())
            }
            TypeDefinitionReference::Union(utd) => utd.as_ref().contains_type(object_type),
            TypeDefinitionReference::BuiltinScalar(_)
            | TypeDefinitionReference::CustomScalar(_)
            | TypeDefinitionReference::Enum(_)
            | TypeDefinitionReference::InputObject(_) => unreachable!(),
        }
    }

    fn execute_field(
        &'a self,
        object_type: &ObjectTypeDefinition,
        object_value: Value,
        field_definition: &FieldDefinition,
        fields: &[&'a Field],
    ) -> (Value, Vec<ExecutionError<'a>>) {
        let field = fields.first().unwrap();

        if field_definition.argument_definitions().is_empty() {
            self.resolve_field_value(object_type, object_value, field_definition, ())
                .map_err(|err| vec![err])
        } else {
            self.coerce_argument_values(field_definition, field)
                .and_then(|argument_values| {
                    self.resolve_field_value(object_type, object_value, field_definition, unsafe {
                        argument_values.as_slice()
                    })
                    .map_err(|err| vec![err])
                })
        }
        .map(|resolved_value| {
            self.complete_value(field_definition.r#type(), fields, resolved_value)
        })
        .unwrap_or_else(|errors| (*QNIL, errors))
    }

    fn coerce_argument_values(
        &'a self,
        field_definition: &FieldDefinition,
        field: &Field,
    ) -> Result<RArray, Vec<ExecutionError<'a>>> {
        let coerced_args = RArray::with_capacity(field_definition.argument_definitions().len());
        let mut errors: Vec<ExecutionError<'a>> = Vec::new();
        let argument_definitions = field_definition.argument_definitions();
        for argument_definition in argument_definitions.iter() {
            match self.coerce_argument_value(argument_definition, field.arguments()) {
                Ok(value) => coerced_args.push(value).unwrap(),
                Err(errs) => errors.extend(errs.into_iter()),
            }
        }

        if errors.is_empty() {
            Ok(coerced_args)
        } else {
            Err(errors)
        }
    }

    fn coerce_argument_value(
        &'a self,
        argument_definition: &InputValueDefinition,
        arguments: Option<&VariableArguments>,
    ) -> Result<Value, Vec<ExecutionError<'a>>> {
        let argument_name = argument_definition.name();
        let argument_type = argument_definition.r#type();
        let default_value = argument_definition.default_value();
        let argument_value: Option<&VariableValue> = arguments.and_then(|arguments| {
            arguments
                .iter()
                .find(|argument| argument.name().as_ref() == argument_name)
                .map(|argument| argument.value())
        });
        let has_value = argument_value.is_some();
        match default_value {
            Some(default_value) if !has_value => Ok(default_value.to_value()),
            _ => {
                if argument_type.is_required() && !has_value {
                    // TODO: field error
                    // shouldn't this never happen if query is validated and variables coerced to match definition in query?
                    todo!()
                } else if let Some(argument_value) = argument_value {
                    // TODO: see if it is possible to distinguish between null and no value being passed
                    match argument_type.coerce_parser_value(
                        argument_value,
                        &[argument_name.to_owned()],
                        self.variables,
                    ) {
                        Ok(Ok(coerced_value)) => Ok(coerced_value),
                        Ok(Err(coercion_errors)) => Err(coercion_errors
                            .into_iter()
                            .map(ExecutionError::CoercionError)
                            .collect()),
                        Err(error) => Err(vec![ExecutionError::ApplicationError(error)]),
                    }
                } else {
                    Ok(*QNIL)
                }
            }
        }
    }

    fn resolve_field_value(
        &'a self,
        _object_type: &ObjectTypeDefinition,
        object_value: Value,
        field_definition: &FieldDefinition,
        argument_values: impl ArgList,
    ) -> Result<Value, ExecutionError<'a>> {
        // TODO: use object_type somehow?
        object_value
            .funcall(
                field_definition.ruby_resolver_method_name(),
                argument_values,
            )
            .map_err(ExecutionError::ApplicationError)
    }

    fn complete_value(
        &'a self,
        field_type: &OutputTypeReference,
        fields: &[&'a Field],
        result: Value,
    ) -> (Value, Vec<ExecutionError<'a>>) {
        if field_type.as_ref().is_required() && result.is_nil() {
            return (
                *QNIL,
                vec![ExecutionError::FieldError(
                    FieldError::ReturnedNullForNonNullType,
                )],
            );
        } else if result.is_nil() {
            return (result, vec![]);
        }

        match field_type.as_ref() {
            CoreOutputTypeReference::Base(inner, _) => match inner {
                BaseOutputTypeReference::BuiltinScalar(bstd) => match bstd.coerce_result(result) {
                    Ok(value) => (value, vec![]),
                    Err(error) => (*QNIL, vec![ExecutionError::FieldError(error)]),
                },
                BaseOutputTypeReference::CustomScalar(cstd) => {
                    match cstd.as_ref().coerce_result(result) {
                        Ok(value) => (value, vec![]),
                        Err(error) => (*QNIL, vec![ExecutionError::FieldError(error)]),
                    }
                }
                BaseOutputTypeReference::Enum(etd) => match etd.as_ref().coerce_result(result) {
                    Ok(value) => (value, vec![]),
                    Err(error) => (*QNIL, vec![ExecutionError::FieldError(error)]),
                },
                BaseOutputTypeReference::Object(otd) => {
                    let sub_selection_set = Self::merge_selection_sets(fields);
                    self.execute_selection_set(sub_selection_set, otd.as_ref(), result)
                }
                BaseOutputTypeReference::Interface(itd) => {
                    let object_type = self.resolve_interface_type(itd.as_ref(), result);
                    let sub_selection_set = Self::merge_selection_sets(fields);
                    self.execute_selection_set(sub_selection_set, object_type, result)
                }
                BaseOutputTypeReference::Union(utd) => {
                    let object_type = self.resolve_union_type(utd.as_ref(), result);
                    let sub_selection_set = Self::merge_selection_sets(fields);
                    self.execute_selection_set(sub_selection_set, object_type, result)
                }
            },
            CoreOutputTypeReference::List(inner, _) => {
                if let Some(arr) = RArray::from_value(result) {
                    let completed = RArray::with_capacity(arr.len());
                    let mut errors: Vec<ExecutionError<'a>> = Vec::new();
                    let mut has_null = false;
                    for item in unsafe { arr.as_slice() } {
                        let (value, mut errs) = self.complete_value(inner, fields, *item);
                        completed.push(value).unwrap(); // TODO: make sure unwrapping is ok here
                        errors.append(&mut errs);
                        if value.is_nil() {
                            has_null = true;
                        }
                    }
                    if inner.as_ref().is_required() && has_null {
                        (*QNIL, errors)
                    } else {
                        (*completed, errors)
                    }
                } else {
                    (
                        *QNIL,
                        vec![ExecutionError::FieldError(
                            FieldError::ReturnedNonListForListType,
                        )],
                    )
                }
            }
        }
    }

    fn merge_selection_sets<'b>(fields: &'b [&'a Field]) -> MergedSelectionSets<'a, 'b> {
        fields
            .iter()
            .flat_map(|field| field.selection_set().map(AsRef::as_ref).unwrap_or_default())
    }

    fn resolve_interface_type(
        &'a self,
        interface_type: &InterfaceTypeDefinition,
        object_value: Value,
    ) -> &'a ObjectTypeDefinition {
        // TODO: change to return Result<_, FieldError>
        let typename: String = object_value
            .funcall(
                FieldDefinition::typename()
                    .get()
                    .ruby_resolver_method_name(),
                (),
            )
            .unwrap();
        let object_type = match self.schema.r#type(typename.as_str()) {
            Some(TypeDefinitionReference::Object(otd)) => otd.as_ref(),
            _ => panic!(),
        };
        if object_type.implements_interface(interface_type) {
            object_type
        } else {
            panic!()
        }
    }

    fn resolve_union_type(
        &'a self,
        union_type: &UnionTypeDefinition,
        object_value: Value,
    ) -> &'a ObjectTypeDefinition {
        // TODO: change to return Result<_, FieldError>
        let typename: String = object_value
            .funcall(
                FieldDefinition::typename()
                    .get()
                    .ruby_resolver_method_name(),
                (),
            )
            .unwrap();
        let object_type = match self.schema.r#type(typename.as_str()) {
            Some(TypeDefinitionReference::Object(otd)) => otd.as_ref(),
            _ => panic!(),
        };
        if union_type.contains_type(object_type) {
            object_type
        } else {
            panic!()
        }
    }

    fn coerce_directive(
        &'a self,
        directive: &'a Directive<'a, false>,
    ) -> Result<Value, Vec<ExecutionError<'a>>> {
        let directive_definition_obj = self.schema.directive(directive.name().as_ref()).unwrap();
        let directive_definition = directive_definition_obj.get();

        let directive = if directive_definition.arguments_definition().is_empty() {
            directive_definition.ruby_class().new_instance(()).unwrap()
        } else {
            let coerced_args =
                RArray::with_capacity(directive_definition.arguments_definition().len());
            let mut errors = Vec::new();
            for argument_definition in directive_definition.arguments_definition().iter() {
                match self.coerce_argument_value(argument_definition, directive.arguments()) {
                    Ok(value) => coerced_args.push(value).unwrap(),
                    Err(errs) => errors.extend(errs.into_iter()),
                }
            }

            if errors.is_empty() {
                directive_definition
                    .ruby_class()
                    .new_instance(unsafe { coerced_args.as_slice() })
                    .unwrap()
            } else {
                return Err(errors);
            }
        };

        Ok(directive)
    }
}
