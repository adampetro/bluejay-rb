use crate::execution::Engine as ExecutionEngine;
use crate::helpers::{Warden, WrappedDefinition};
use crate::ruby_api::{
    base, root, ArgumentsDefinition, BaseInputType, BaseOutputType, CustomScalarTypeDefinition,
    DirectiveDefinition, Directives, EnumTypeDefinition, EnumValueDefinition, EnumValueDefinitions,
    ExecutionResult, FieldDefinition, FieldsDefinition, InputFieldsDefinition,
    InputObjectTypeDefinition, InputType, InputValueDefinition, InterfaceImplementation,
    InterfaceImplementations, InterfaceTypeDefinition, ObjectTypeDefinition, OutputType,
    TypeDefinition, UnionMemberType, UnionMemberTypes, UnionTypeDefinition, ValidationError,
};
use crate::visibility_scoped::{ScopedSchemaDefinition, VisibilityCache};
use bluejay_core::definition::{
    InputType as CoreInputType, OutputType as CoreOutputType,
    SchemaDefinition as CoreSchemaDefinition, TypeDefinition as CoreTypeDefinition,
    TypeDefinitionReference,
};
use bluejay_core::AsIter;
use bluejay_printer::definition::SchemaDefinitionPrinter;
use bluejay_validator::executable::{BuiltinRulesValidator, Cache as ValidationCache};
use magnus::{
    exception, function, gc, memoize, method, scan_args::get_kwargs, scan_args::KwArgs,
    typed_data::Obj, DataTypeFunctions, Error, Module, Object, RArray, RClass, RHash, RModule,
    TypedData, Value,
};
use std::collections::{
    btree_map::{Entry, Values},
    BTreeMap, HashMap,
};

#[derive(Debug, TypedData)]
#[magnus(class = "Bluejay::SchemaDefinition", mark)]
pub struct SchemaDefinition {
    description: Option<String>,
    query: WrappedDefinition<ObjectTypeDefinition>,
    mutation: Option<WrappedDefinition<ObjectTypeDefinition>>,
    directives: Directives,
    contained_types: BTreeMap<String, TypeDefinition>,
    contained_directives: BTreeMap<String, WrappedDefinition<DirectiveDefinition>>,
    interface_implementors: HashMap<String, Vec<WrappedDefinition<ObjectTypeDefinition>>>,
    ruby_class: RClass,
}

impl SchemaDefinition {
    pub fn new(kw: RHash) -> Result<Self, Error> {
        let args: KwArgs<_, (), ()> = get_kwargs(
            kw,
            &[
                "description",
                "query",
                "mutation",
                "directives",
                "ruby_class",
            ],
            &[],
        )?;
        let (description, query, mutation, directives, ruby_class): (
            Option<String>,
            WrappedDefinition<ObjectTypeDefinition>,
            Option<WrappedDefinition<ObjectTypeDefinition>>,
            RArray,
            RClass,
        ) = args.required;
        if !query.wrapper().is_kind_of(Self::query_root_module()) {
            return Err(Error::new(
                exception::type_error(),
                format!(
                    "no implicit conversion of {} into {}",
                    query.wrapper(),
                    Self::query_root_module(),
                ),
            ));
        }
        let directives = directives.try_into()?;
        let (contained_types, contained_directives) =
            SchemaTypeVisitor::compute_contained_definitions(
                &query,
                mutation.as_ref(),
                &directives,
            )?;
        let interface_implementors = Self::interface_implementors(&contained_types);

        Self::validate_default_values(&contained_types)?;

        Ok(Self {
            description,
            query,
            mutation,
            directives,
            contained_types,
            contained_directives,
            interface_implementors,
            ruby_class,
        })
    }

    pub fn query(&self) -> Obj<ObjectTypeDefinition> {
        *self.query.get()
    }

    pub fn mutation(&self) -> Option<Obj<ObjectTypeDefinition>> {
        self.mutation.as_ref().map(WrappedDefinition::get).copied()
    }

    pub fn r#type(&self, name: &str) -> Option<&TypeDefinition> {
        self.contained_types.get(name)
    }

    pub fn directive(&self, name: &str) -> Option<Obj<DirectiveDefinition>> {
        self.contained_directives.get(name).map(|wd| *wd.get())
    }

    pub fn ruby_class(&self) -> RClass {
        self.ruby_class
    }

    fn execute(
        &self,
        query: String,
        operation_name: Option<String>,
        variable_values: RHash,
        initial_value: Value,
        context: Value,
    ) -> Result<ExecutionResult, Error> {
        ExecutionEngine::execute_request(
            self,
            query.as_str(),
            operation_name.as_deref(),
            variable_values,
            initial_value,
            context,
        )
    }

    fn validate_query(&self, query: String, context: Value) -> Result<RArray, Error> {
        if let Ok(document) =
            bluejay_parser::ast::executable::ExecutableDocument::parse(query.as_str())
        {
            let warden = Warden::new(context);
            let cache = VisibilityCache::new(warden);
            let scoped_schema_definition = ScopedSchemaDefinition::new(self, &cache);

            let r_array = RArray::from_iter(
                BuiltinRulesValidator::validate(
                    &document,
                    &scoped_schema_definition,
                    &ValidationCache::new(&document, &scoped_schema_definition),
                )
                .map(|error| -> Obj<ValidationError> { Obj::wrap(error.into()) }),
            );
            cache.warden().to_result().map(|_| r_array)
        } else {
            Ok(RArray::new())
        }
    }

    fn to_definition(&self, context: Value) -> Result<String, Error> {
        let warden = Warden::new(context);
        let cache = VisibilityCache::new(warden);
        let scoped_schema_definition = ScopedSchemaDefinition::new(self, &cache);

        let s = SchemaDefinitionPrinter::to_string(&scoped_schema_definition);
        cache.warden().to_result().map(|_| s)
    }

    fn interface_implementors(
        type_definitions: &BTreeMap<String, TypeDefinition>,
    ) -> HashMap<String, Vec<WrappedDefinition<ObjectTypeDefinition>>> {
        type_definitions.values().fold(
            HashMap::new(),
            |mut interface_implementors, type_definition| {
                if let TypeDefinition::Object(otd) = type_definition {
                    otd.as_ref().interface_implementations().iter().for_each(
                        |interface_implementation| {
                            let itd = interface_implementation.interface();
                            interface_implementors
                                .entry(itd.as_ref().name().to_owned())
                                .or_default()
                                .push(otd.clone());
                        },
                    );
                }

                interface_implementors
            },
        )
    }

    fn validate_default_values(
        type_definitions: &BTreeMap<String, TypeDefinition>,
    ) -> Result<(), Error> {
        // TODO: something better than a warden with nil context
        let cache = VisibilityCache::new(Warden::new(*magnus::QNIL));
        type_definitions
            .values()
            .try_for_each(|type_definition| match type_definition {
                TypeDefinition::InputObject(iotd) => iotd
                    .as_ref()
                    .input_fields_definition()
                    .iter()
                    .try_for_each(|ivd| ivd.validate_default_value(&cache)),
                TypeDefinition::Object(otd) => {
                    otd.as_ref().fields_definition().iter().try_for_each(|fd| {
                        fd.argument_definitions()
                            .iter()
                            .try_for_each(|ivd| ivd.validate_default_value(&cache))
                    })
                }
                TypeDefinition::Interface(itd) => {
                    itd.as_ref().fields_definition().iter().try_for_each(|fd| {
                        fd.argument_definitions()
                            .iter()
                            .try_for_each(|ivd| ivd.validate_default_value(&cache))
                    })
                }
                _ => Ok(()),
            })?;
        cache.warden().to_result()
    }

    fn query_root_module() -> RModule {
        *memoize!(RModule: base().define_module("QueryRoot").unwrap())
    }
}

impl DataTypeFunctions for SchemaDefinition {
    fn mark(&self) {
        self.query.mark();
        if let Some(mutation) = &self.mutation {
            mutation.mark();
        }
        self.directives.mark();
        gc::mark(self.ruby_class);
        self.contained_types.values().for_each(TypeDefinition::mark);
        self.contained_directives
            .values()
            .for_each(WrappedDefinition::mark);
        self.interface_implementors
            .values()
            .flatten()
            .for_each(WrappedDefinition::mark);
    }
}

impl CoreSchemaDefinition for SchemaDefinition {
    type InputValueDefinition = InputValueDefinition;
    type InputFieldsDefinition = InputFieldsDefinition;
    type ArgumentsDefinition = ArgumentsDefinition;
    type EnumValueDefinition = EnumValueDefinition;
    type EnumValueDefinitions = EnumValueDefinitions;
    type FieldDefinition = FieldDefinition;
    type FieldsDefinition = FieldsDefinition;
    type InterfaceImplementation = InterfaceImplementation;
    type InterfaceImplementations = InterfaceImplementations;
    type UnionMemberType = UnionMemberType;
    type UnionMemberTypes = UnionMemberTypes;
    type BaseInputType = BaseInputType;
    type InputType = InputType;
    type BaseOutputType = BaseOutputType;
    type OutputType = OutputType;
    type CustomScalarTypeDefinition = CustomScalarTypeDefinition;
    type ObjectTypeDefinition = ObjectTypeDefinition;
    type InterfaceTypeDefinition = InterfaceTypeDefinition;
    type UnionTypeDefinition = UnionTypeDefinition;
    type InputObjectTypeDefinition = InputObjectTypeDefinition;
    type EnumTypeDefinition = EnumTypeDefinition;
    type TypeDefinition = TypeDefinition;
    type DirectiveDefinition = DirectiveDefinition;
    type Directives = Directives;
    type TypeDefinitions<'a> = std::iter::Map<
        Values<'a, String, TypeDefinition>,
        fn(&'a TypeDefinition) -> TypeDefinitionReference<'a, TypeDefinition>,
    >;
    type DirectiveDefinitions<'a> = std::iter::Map<
        Values<'a, String, WrappedDefinition<DirectiveDefinition>>,
        fn(&WrappedDefinition<DirectiveDefinition>) -> &DirectiveDefinition,
    >;
    type InterfaceImplementors<'a> = std::iter::Flatten<
        std::option::IntoIter<
            std::iter::Map<
                std::slice::Iter<'a, WrappedDefinition<ObjectTypeDefinition>>,
                fn(&WrappedDefinition<ObjectTypeDefinition>) -> &ObjectTypeDefinition,
            >,
        >,
    >;

    fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    fn query(&self) -> &Self::ObjectTypeDefinition {
        self.query.as_ref()
    }

    fn mutation(&self) -> Option<&Self::ObjectTypeDefinition> {
        self.mutation.as_ref().map(AsRef::as_ref)
    }

    fn subscription(&self) -> Option<&Self::ObjectTypeDefinition> {
        None
    }

    fn schema_directives(&self) -> Option<&Self::Directives> {
        self.directives.to_option()
    }

    fn get_directive_definition(&self, name: &str) -> Option<&Self::DirectiveDefinition> {
        self.contained_directives.get(name).map(AsRef::as_ref)
    }

    fn get_type_definition(
        &self,
        name: &str,
    ) -> Option<TypeDefinitionReference<'_, TypeDefinition>> {
        self.contained_types
            .get(name)
            .map(CoreTypeDefinition::as_ref)
    }

    fn type_definitions(&self) -> Self::TypeDefinitions<'_> {
        self.contained_types
            .values()
            .map(CoreTypeDefinition::as_ref)
    }

    fn directive_definitions(&self) -> Self::DirectiveDefinitions<'_> {
        self.contained_directives.values().map(AsRef::as_ref)
    }

    fn get_interface_implementors(
        &self,
        itd: &Self::InterfaceTypeDefinition,
    ) -> Self::InterfaceImplementors<'_> {
        type MapFn<'a> = std::iter::Map<
            std::slice::Iter<'a, WrappedDefinition<ObjectTypeDefinition>>,
            fn(&WrappedDefinition<ObjectTypeDefinition>) -> &ObjectTypeDefinition,
        >;

        self.interface_implementors
            .get(itd.name())
            .map(|interface_implementors| -> MapFn<'_> {
                interface_implementors.iter().map(AsRef::as_ref)
            })
            .into_iter()
            .flatten()
    }
}

struct SchemaTypeVisitor {
    types: BTreeMap<String, TypeDefinition>,
    directives: BTreeMap<String, WrappedDefinition<DirectiveDefinition>>,
}

type ContainedDefinitionResult = Result<
    (
        BTreeMap<String, TypeDefinition>,
        BTreeMap<String, WrappedDefinition<DirectiveDefinition>>,
    ),
    Error,
>;

impl SchemaTypeVisitor {
    pub fn compute_contained_definitions(
        query: &WrappedDefinition<ObjectTypeDefinition>,
        mutation: Option<&WrappedDefinition<ObjectTypeDefinition>>,
        schema_directives: &Directives,
    ) -> ContainedDefinitionResult {
        let mut type_visitor = Self::new();
        type_visitor.visit_type(TypeDefinition::Object(query.clone()))?;
        if let Some(mutation) = mutation {
            type_visitor.visit_type(TypeDefinition::Object(mutation.clone()))?;
        }
        type_visitor.visit_directives(schema_directives)?;
        type_visitor.visit_builtin_directive_definitions()?;
        let Self { types, directives } = type_visitor;
        Ok((types, directives))
    }

    fn new() -> Self {
        Self {
            types: BTreeMap::new(),
            directives: BTreeMap::new(),
        }
    }

    fn visit_object_type_definition(&mut self, otd: &ObjectTypeDefinition) -> Result<(), Error> {
        self.visit_field_definitions(otd.fields_definition())?;
        otd.interface_implementations()
            .iter()
            .try_for_each(|ii| self.visit_type(TypeDefinition::Interface(ii.interface())))?;
        self.visit_directives(otd.directives())
    }

    fn visit_union_type_definition(&mut self, utd: &UnionTypeDefinition) -> Result<(), Error> {
        utd.member_types().iter().try_for_each(|union_member| {
            let t = TypeDefinition::Object(union_member.r#type());
            self.visit_type(t)
        })?;
        self.visit_directives(utd.directives())
    }

    fn visit_interface_type_definition(
        &mut self,
        itd: &InterfaceTypeDefinition,
    ) -> Result<(), Error> {
        self.visit_field_definitions(itd.fields_definition())?;
        itd.interface_implementations()
            .iter()
            .try_for_each(|ii| self.visit_type(TypeDefinition::Interface(ii.interface())))?;
        self.visit_directives(itd.directives())
    }

    fn visit_input_object_type_definition(
        &mut self,
        iotd: &InputObjectTypeDefinition,
    ) -> Result<(), Error> {
        self.visit_input_value_definitions(iotd.input_fields_definition())?;
        self.visit_directives(iotd.directives())
    }

    fn visit_custom_scalar_type_definition(
        &mut self,
        cstd: &CustomScalarTypeDefinition,
    ) -> Result<(), Error> {
        self.visit_directives(cstd.directives())
    }

    fn visit_enum_type_definition(&mut self, etd: &EnumTypeDefinition) -> Result<(), Error> {
        etd.enum_value_definitions()
            .iter()
            .try_for_each(|evd| self.visit_directives(evd.directives()))?;
        self.visit_directives(etd.directives())
    }

    fn visit_type(&mut self, t: TypeDefinition) -> Result<(), Error> {
        t.try_init_wrapped_definition()?;
        let name = t.as_ref().name().to_owned();
        match self.types.entry(name) {
            Entry::Occupied(entry) => {
                if entry.get() != &t {
                    let message = format!(
                        "GraphQL type name `{}` is used in multiple classes: {} and {}",
                        entry.key(),
                        entry.get().classname(),
                        t.classname(),
                    );
                    Err(Error::new(
                        crate::ruby_api::non_unique_definition_name_error(),
                        message,
                    ))
                } else {
                    Ok(())
                }
            }
            Entry::Vacant(entry) => {
                entry.insert(t.clone());
                match t {
                    TypeDefinition::BuiltinScalar(_) => Ok(()),
                    TypeDefinition::CustomScalar(cstd) => {
                        self.visit_custom_scalar_type_definition(cstd.as_ref())
                    }
                    TypeDefinition::Enum(etd) => self.visit_enum_type_definition(etd.as_ref()),
                    TypeDefinition::Object(otd) => self.visit_object_type_definition(otd.as_ref()),
                    TypeDefinition::Union(utd) => self.visit_union_type_definition(utd.as_ref()),
                    TypeDefinition::Interface(itd) => {
                        self.visit_interface_type_definition(itd.as_ref())
                    }
                    TypeDefinition::InputObject(iotd) => {
                        self.visit_input_object_type_definition(iotd.as_ref())
                    }
                }
            }
        }
    }

    fn visit_field_definitions(
        &mut self,
        fields_definition: &FieldsDefinition,
    ) -> Result<(), Error> {
        fields_definition.iter().try_for_each(|field_definition| {
            self.visit_input_value_definitions(field_definition.argument_definitions())?;
            let base_type = field_definition.r#type().as_ref().base();
            self.visit_type(base_type.into())?;
            self.visit_directives(field_definition.directives())
        })
    }

    fn visit_input_value_definitions(
        &mut self,
        input_fields_definition: &InputFieldsDefinition,
    ) -> Result<(), Error> {
        input_fields_definition
            .iter()
            .try_for_each(|input_value_definition| {
                let base_type = input_value_definition.r#type().as_ref().base();
                self.visit_type(base_type.into())?;
                self.visit_directives(input_value_definition.directives())
            })
    }

    fn visit_directives(&mut self, directives: &Directives) -> Result<(), Error> {
        directives.iter().try_for_each(|directive| {
            let definition = directive.definition();
            definition.try_init()?;
            self.directives
                .entry(definition.as_ref().name().to_string())
                .or_insert_with(|| definition.clone());
            Ok(())
        })
    }

    fn visit_builtin_directive_definitions(&mut self) -> Result<(), Error> {
        DirectiveDefinition::builtin_directive_definitions()
            .iter()
            .try_for_each(|definition| {
                definition.try_init()?;
                self.directives
                    .entry(definition.as_ref().name().to_string())
                    .or_insert_with(|| definition.clone());
                Ok(())
            })
    }
}

pub fn init() -> Result<(), Error> {
    let class = root().define_class("SchemaDefinition", Default::default())?;

    class.define_singleton_method("new", function!(SchemaDefinition::new, 1))?;
    class.define_method("execute", method!(SchemaDefinition::execute, 5))?;
    class.define_method(
        "validate_query",
        method!(SchemaDefinition::validate_query, 2),
    )?;
    class.define_method("to_definition", method!(SchemaDefinition::to_definition, 1))?;
    class.define_method(
        "type",
        method!(|sd: &SchemaDefinition, name: String| sd.r#type(&name), 1),
    )?;
    class.define_method(
        "description",
        method!(<SchemaDefinition as CoreSchemaDefinition>::description, 0),
    )?;
    class.define_method("query_type", method!(SchemaDefinition::query, 0))?;
    class.define_method("mutation_type", method!(SchemaDefinition::mutation, 0))?;
    class.define_method("subscription_type", method!(|_: &SchemaDefinition| (), 0))?;
    class.define_method(
        "types",
        method!(
            |sd: &SchemaDefinition| RArray::from_iter(sd.contained_types.values()),
            0
        ),
    )?;
    class.define_method(
        "directives",
        method!(
            |sd: &SchemaDefinition| RArray::from_iter(sd.contained_directives.values()),
            0
        ),
    )?;
    class.define_method(
        "resolve_typename",
        method!(|_: &SchemaDefinition| "__Schema", 0),
    )?;

    Ok(())
}
