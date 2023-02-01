use crate::execution::{CoerceResult, ExecutionError, FieldError, KeyStore};
use crate::helpers::WrappedStruct;
use crate::ruby_api::{
    BaseInputTypeReference, BaseOutputTypeReference, CoerceInput,
    ExecutionError as RubyExecutionError, ExecutionResult, FieldDefinition, InputTypeReference,
    InterfaceTypeDefinition, ObjectTypeDefinition, OutputTypeReference, SchemaDefinition,
    TypeDefinitionReference, UnionTypeDefinition,
};
use bluejay_core::{
    definition::OutputTypeReference as CoreOutputTypeReference, AsIter, BooleanValue, FloatValue,
    IntegerValue, ObjectValue, OperationType, Value as CoreValue, Variable as CoreVariable,
};
use bluejay_parser::ast::executable::{
    ExecutableDocument, Field, OperationDefinition, Selection, VariableDefinition,
};
use bluejay_parser::ast::{VariableArgument, VariableValue};
use magnus::{Error, RArray, RHash, RString, Value, QNIL};
use std::collections::{BTreeMap, HashSet};

pub struct Engine<'a> {
    schema: &'a SchemaDefinition,
    document: &'a ExecutableDocument<'a>,
    variable_values: &'a RHash, // pointer to ensure it stays on the stack somewhere
    key_store: KeyStore<'a>,
}

impl<'a> Engine<'a> {
    pub fn execute_request(
        schema: &SchemaDefinition,
        query: &str,
        operation_name: Option<&str>,
        variable_values: RHash,
        initial_value: Value,
    ) -> Result<ExecutionResult, Error> {
        let (document, parse_errors) = bluejay_parser::ast::parse(query);

        if !parse_errors.is_empty() {
            return Ok(Self::execution_result(
                Default::default(),
                parse_errors
                    .into_iter()
                    .map(ExecutionError::ParseError)
                    .collect(),
            ));
        }

        let operation_definition = match Self::get_operation(&document, operation_name) {
            Ok(od) => od,
            Err(error) => {
                return Ok(Self::execution_result(Default::default(), vec![error]));
            }
        };

        let coerced_variable_values =
            match Self::get_variable_values(&schema, &operation_definition, variable_values) {
                Ok(cvv) => cvv,
                Err(errors) => {
                    return Ok(Self::execution_result(Default::default(), errors));
                }
            };

        let instance = Engine {
            schema: &schema,
            document: &document,
            variable_values: &coerced_variable_values,
            key_store: KeyStore::new(),
        };

        match operation_definition.operation_type() {
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
                .find(|od| matches!(od.name(), Some(n) if n == operation_name))
                .ok_or_else(|| ExecutionError::NoOperationWithName {
                    name: operation_name,
                })
        } else {
            if document.operation_definitions().len() == 1 {
                Ok(&document.operation_definitions()[0])
            } else {
                Err(ExecutionError::CannotUseAnonymousOperation)
            }
        }
    }

    fn get_variable_values<'b>(
        schema: &'b SchemaDefinition,
        operation: &'b OperationDefinition,
        variable_values: RHash,
    ) -> Result<RHash, Vec<ExecutionError<'b>>> {
        let coerced_values = RHash::new();
        let variable_definitions: &[VariableDefinition] = operation
            .variable_definitions()
            .map(AsRef::as_ref)
            .unwrap_or_default();
        let mut errors: Vec<ExecutionError<'b>> = Vec::new();

        for variable_definition in variable_definitions {
            let variable_name = variable_definition.variable().name();
            let variable_named_type_reference = variable_definition.r#type();
            let variable_base_type = schema.r#type(variable_named_type_reference.name()).unwrap();
            let base_input_type_reference: BaseInputTypeReference =
                variable_base_type.try_into().unwrap();
            let variable_type = InputTypeReference::from_parser_type_reference(
                variable_named_type_reference,
                base_input_type_reference,
            );
            let default_value = variable_definition.default_value();
            let value = variable_values.get(variable_name);
            let has_value = value.is_some();
            if !has_value && default_value.is_some() {
                coerced_values
                    .aset(
                        variable_name,
                        Self::value_from_core_const_value(default_value.unwrap()),
                    )
                    .unwrap();
            } else if variable_type.is_required() && !has_value {
                errors.push(ExecutionError::RequiredVariableMissingValue {
                    name: variable_name,
                });
            } else {
                let path = vec![variable_name.to_owned()];
                let value = value.unwrap_or_default();
                match variable_type.coerce_input(value, &path) {
                    Ok(Ok(coerced_value)) => {
                        coerced_values.aset(variable_name, coerced_value).unwrap();
                    }
                    Ok(Err(coercion_errors)) => {
                        errors.extend(
                            coercion_errors
                                .into_iter()
                                .map(|ce| ExecutionError::CoercionError(ce)),
                        );
                    }
                    Err(error) => errors.push(ExecutionError::ApplicationError(error)),
                }
            }
        }

        if errors.is_empty() {
            Ok(coerced_values)
        } else {
            Err(errors)
        }
    }

    fn value_from_core_const_value(value: &impl bluejay_core::AbstractConstValue) -> Value {
        match value.as_ref() {
            CoreValue::Boolean(b) => b.to_bool().into(),
            CoreValue::Enum(e) => e.as_ref().into(),
            CoreValue::Float(f) => f.to_f64().into(),
            CoreValue::Integer(i) => i.to_i32().into(),
            CoreValue::Null(_) => *QNIL,
            CoreValue::String(s) => s.as_ref().into(),
            CoreValue::Variable(_) => unreachable!(),
            CoreValue::List(l) => {
                *RArray::from_iter(l.as_ref().iter().map(Self::value_from_core_const_value))
            }
            CoreValue::Object(o) => *RHash::from_iter(
                o.fields()
                    .iter()
                    .map(|(k, v)| (k.as_ref(), Self::value_from_core_const_value(v))),
            ),
        }
    }

    fn value_from_core_variable_value(
        value: &impl bluejay_core::AbstractVariableValue,
        variable_values: RHash,
    ) -> Value {
        match value.as_ref() {
            CoreValue::Boolean(b) => b.to_bool().into(),
            CoreValue::Enum(e) => e.as_ref().into(),
            CoreValue::Float(f) => f.to_f64().into(),
            CoreValue::Integer(i) => i.to_i32().into(),
            CoreValue::Null(_) => *QNIL,
            CoreValue::String(s) => s.as_ref().into(),
            CoreValue::Variable(v) => variable_values.get(v.name()).unwrap_or(*QNIL),
            CoreValue::List(l) => *RArray::from_iter(
                l.as_ref()
                    .iter()
                    .map(|v| Self::value_from_core_variable_value(v, variable_values)),
            ),
            CoreValue::Object(o) => *RHash::from_iter(o.fields().iter().map(|(k, v)| {
                (
                    k.as_ref(),
                    Self::value_from_core_variable_value(v, variable_values),
                )
            })),
        }
    }

    fn execution_result(value: Value, errors: Vec<ExecutionError>) -> ExecutionResult {
        let errors: Vec<WrappedStruct<RubyExecutionError>> = errors
            .into_iter()
            .map(|error| WrappedStruct::wrap(error.into()))
            .collect();
        ExecutionResult::new(value, errors)
    }

    fn execute_query(
        &'a self,
        query: &'a OperationDefinition,
        initial_value: Value,
    ) -> ExecutionResult {
        let query_type = self.schema.query();
        let query_type = query_type.get();
        let selection_set = query.selection_set();

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
            let field_name = fields.first().unwrap().name();
            let field_definition = object_type.field_definition(field_name).unwrap();
            let (response_value, mut errs) =
                self.execute_field(object_type, object_value, field_definition, &fields);
            if field_definition.r#type().as_ref().is_required() && response_value.is_nil() {
                has_null_for_required = true;
            }
            let key: RString = self.key_store.get(response_key);
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
            // TODO: skip directive check
            // TODO: include directive check
            match selection {
                Selection::Field(field) => {
                    let response_key = field.response_key();
                    let entry_for_response_key = grouped_fields.entry(response_key).or_default();
                    entry_for_response_key.push(field);
                }
                Selection::FragmentSpread(fragment_spread) => {
                    let fragment_spread_name = fragment_spread.name();
                    if visited_fragments.insert(fragment_spread_name) {
                        let fragment = self
                            .document
                            .fragment_definitions()
                            .iter()
                            .find(|fd| fd.name() == fragment_spread_name);

                        let fragment = match fragment {
                            Some(f) => f,
                            None => {
                                continue;
                            }
                        };

                        let fragment_type = fragment.type_condition();

                        if !self.does_fragment_type_apply(object_type, fragment_type) {
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
                            group_for_response_key.extend_from_slice(&fragment_group);
                        }
                    }
                }
                Selection::InlineFragment(inline_fragment) => {
                    let fragment_type = inline_fragment.type_condition();

                    if matches!(fragment_type, Some(fragment_type) if !self.does_fragment_type_apply(object_type, fragment_type))
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
                        group_for_response_key.extend_from_slice(&fragment_group);
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
            TypeDefinitionReference::ObjectType(otd, _) => {
                // TODO: see if there's any edge case where name equality does not work
                otd.as_ref().name() == object_type.name()
            }
            TypeDefinitionReference::InterfaceType(itd, _) => {
                object_type.implements_interface(itd.as_ref())
            }
            TypeDefinitionReference::UnionType(utd, _) => utd.as_ref().contains_type(object_type),
            TypeDefinitionReference::BuiltinScalarType(_)
            | TypeDefinitionReference::CustomScalarType(_, _)
            | TypeDefinitionReference::EnumType(_, _)
            | TypeDefinitionReference::InputObjectType(_, _) => unreachable!(),
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
        let argument_values = match self.coerce_argument_values(field_definition, field) {
            Ok(argument_values) => argument_values,
            Err(errors) => {
                return (*QNIL, errors);
            }
        };

        let resolved_value = match self.resolve_field_value(
            object_type,
            object_value,
            field_definition,
            argument_values,
        ) {
            Ok(resolved_value) => resolved_value,
            Err(error) => {
                return (*QNIL, vec![error]);
            }
        };

        self.complete_value(field_definition.r#type(), fields, resolved_value)
    }

    fn coerce_argument_values(
        &'a self,
        field_definition: &FieldDefinition,
        field: &Field,
    ) -> Result<Vec<Value>, Vec<ExecutionError<'a>>> {
        let mut coerced_args: Vec<Value> = Vec::new();
        let mut errors: Vec<ExecutionError<'a>> = Vec::new();
        let arguments: &[VariableArgument] =
            field.arguments().map(AsRef::as_ref).unwrap_or_default();
        let argument_definitions = field_definition.argument_definitions();
        for argument_definition in argument_definitions.iter() {
            let argument_name = argument_definition.name();
            let argument_type = argument_definition.r#type();
            let default_value = argument_definition.default_value();
            let argument_value: Option<Value> = arguments
                .iter()
                .find(|argument| argument.name() == argument_name)
                .and_then(|argument| {
                    let value = argument.value();
                    match value {
                        VariableValue::Variable(variable) => {
                            let variable_name = variable.name();
                            self.variable_values.get(variable_name)
                        }
                        _ => Some(Self::value_from_core_variable_value(
                            value,
                            *self.variable_values,
                        )),
                    }
                });
            let has_value = argument_value.is_some();
            if !has_value && default_value.is_some() {
                match argument_type
                    .coerce_input(default_value.unwrap(), &[argument_name.to_owned()])
                {
                    Ok(Ok(coerced_value)) => {
                        coerced_args.push(coerced_value);
                    }
                    // TODO: would be kind of bad if default value didn't coerce
                    Ok(Err(coercion_errors)) => {
                        errors.extend(
                            coercion_errors
                                .into_iter()
                                .map(|ce| ExecutionError::CoercionError(ce)),
                        );
                    }
                    Err(error) => {
                        errors.push(ExecutionError::ApplicationError(error));
                    }
                }
            } else if argument_type.is_required()
                && (!has_value || matches!(argument_value, Some(v) if v.is_nil()))
            {
                // TODO: field error
                // shouldn't this never happen if query is validated and variables coerced to match definition in query?
            } else {
                // TODO: see if it is possible to distinguish between null and no value being passed
                match argument_type.coerce_input(
                    argument_value.unwrap_or_default(),
                    &[argument_name.to_owned()],
                ) {
                    Ok(Ok(coerced_value)) => {
                        coerced_args.push(coerced_value);
                    }
                    Ok(Err(coercion_errors)) => {
                        errors.extend(
                            coercion_errors
                                .into_iter()
                                .map(|ce| ExecutionError::CoercionError(ce)),
                        );
                    }
                    Err(error) => {
                        errors.push(ExecutionError::ApplicationError(error));
                    }
                }
            }
        }

        if errors.is_empty() {
            Ok(coerced_args)
        } else {
            Err(errors)
        }
    }

    fn resolve_field_value(
        &'a self,
        _object_type: &ObjectTypeDefinition,
        object_value: Value,
        field_definition: &FieldDefinition,
        argument_values: Vec<Value>,
    ) -> Result<Value, ExecutionError<'a>> {
        // TODO: use object_type somehow?
        object_value
            .funcall(
                field_definition.ruby_resolver_method_name(),
                argument_values.as_slice(),
            )
            .map_err(|error| ExecutionError::ApplicationError(error))
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
            CoreOutputTypeReference::Base(inner, _) => {
                match inner {
                    BaseOutputTypeReference::BuiltinScalarType(bstd) => {
                        match bstd.coerce_result(result) {
                            Ok(value) => (value, vec![]),
                            Err(error) => (*QNIL, vec![ExecutionError::FieldError(error)]),
                        }
                    }
                    BaseOutputTypeReference::CustomScalarType(_) => (result, vec![]), // TODO: see if any checks are needed here
                    BaseOutputTypeReference::EnumType(etd) => {
                        match etd.as_ref().coerce_result(result) {
                            Ok(value) => (value, vec![]),
                            Err(error) => (*QNIL, vec![ExecutionError::FieldError(error)]),
                        }
                    }
                    BaseOutputTypeReference::ObjectType(otd) => {
                        let sub_selection_set = Self::merge_selection_sets(fields);
                        self.execute_selection_set(sub_selection_set, otd.as_ref(), result)
                    }
                    BaseOutputTypeReference::InterfaceType(itd) => {
                        let object_type = self.resolve_interface_type(itd.as_ref(), result);
                        let sub_selection_set = Self::merge_selection_sets(fields);
                        self.execute_selection_set(sub_selection_set, object_type, result)
                    }
                    BaseOutputTypeReference::UnionType(utd) => {
                        let object_type = self.resolve_union_type(utd.as_ref(), result);
                        let sub_selection_set = Self::merge_selection_sets(fields);
                        self.execute_selection_set(sub_selection_set, object_type, result)
                    }
                }
            }
            CoreOutputTypeReference::List(inner, _) => {
                if let Some(arr) = RArray::from_value(result) {
                    let inner = inner.get();
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

    fn merge_selection_sets<'b>(
        fields: &'b [&'a Field],
    ) -> std::iter::FlatMap<
        std::slice::Iter<'b, &'a Field<'a>>,
        &'a [Selection<'a>],
        fn(&&'a Field) -> &'a [Selection<'a>],
    > {
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
            Some(TypeDefinitionReference::ObjectType(otd, _)) => otd.as_ref(),
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
            Some(TypeDefinitionReference::ObjectType(otd, _)) => otd.as_ref(),
            _ => panic!(),
        };
        if union_type.contains_type(object_type) {
            object_type
        } else {
            panic!()
        }
    }
}
