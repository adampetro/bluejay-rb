use crate::execution::Engine as ExecutionEngine;
use crate::helpers::WrappedDefinition;
use crate::ruby_api::{
    base, root, ArgumentsDefinition, BaseInputType, BaseOutputType, CustomScalarTypeDefinition,
    DirectiveDefinition, Directives, EnumTypeDefinition, EnumValueDefinition, EnumValueDefinitions,
    ExecutionResult, FieldDefinition, FieldsDefinition, InputFieldsDefinition,
    InputObjectTypeDefinition, InputType, InputValueDefinition, InterfaceImplementation,
    InterfaceImplementations, InterfaceTypeDefinition, ObjectTypeDefinition, OutputType, Scalar,
    UnionMemberType, UnionMemberTypes, UnionTypeDefinition, ValidationError,
};
use crate::visibility_scoped::{ScopedSchemaDefinition, VisibilityCache};
use bluejay_core::definition::{
    InputType as CoreInputType, OutputType as CoreOutputType,
    SchemaDefinition as CoreSchemaDefinition, TypeDefinition as CoreTypeDefinition,
    TypeDefinitionReference,
};
use bluejay_core::{AsIter, BuiltinScalarDefinition};
use bluejay_printer::definition::DisplaySchemaDefinition;
use bluejay_validator::executable::{BuiltinRulesValidator, Cache as ValidationCache};
use bluejay_visibility::NullWarden;
use magnus::IntoValue;
use magnus::{
    exception, function, gc, memoize, method, scan_args::get_kwargs, scan_args::KwArgs,
    typed_data::Obj, DataTypeFunctions, Error, Module, Object, RArray, RClass, RHash, RModule,
    Ruby, TypedData, Value,
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
        if !query.class().is_inherited(Self::query_root_module()) {
            return Err(Error::new(
                exception::type_error(),
                format!(
                    "no implicit conversion of {} into {}",
                    query.class(),
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
            );
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
    ) -> Result<ExecutionResult, Error> {
        ExecutionEngine::execute_request(
            self,
            query.as_str(),
            operation_name.as_deref(),
            variable_values,
            initial_value,
        )
    }

    fn validate_query(&self, query: String) -> RArray {
        if let Ok(document) =
            bluejay_parser::ast::executable::ExecutableDocument::parse(query.as_str())
        {
            let warden = NullWarden::default();
            let cache = VisibilityCache::new(warden);
            let scoped_schema_definition = ScopedSchemaDefinition::new(self, &cache);

            RArray::from_iter(
                BuiltinRulesValidator::validate(
                    &document,
                    &scoped_schema_definition,
                    &ValidationCache::new(&document, &scoped_schema_definition),
                )
                .map(|error| -> Obj<ValidationError> { Obj::wrap(error.into()) }),
            )
        } else {
            RArray::new()
        }
    }

    fn to_definition(&self) -> String {
        let cache = VisibilityCache::new(NullWarden::default());
        let scoped_schema_definition = ScopedSchemaDefinition::new(self, &cache);

        DisplaySchemaDefinition::to_string(&scoped_schema_definition)
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
                                .entry(itd.get().name().to_owned())
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
        let cache = VisibilityCache::new(bluejay_visibility::NullWarden::default());
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
            })
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

#[derive(Debug, Clone)]
pub enum TypeDefinition {
    BuiltinScalar(BuiltinScalarDefinition),
    CustomScalar(WrappedDefinition<CustomScalarTypeDefinition>),
    Object(WrappedDefinition<ObjectTypeDefinition>),
    InputObject(WrappedDefinition<InputObjectTypeDefinition>),
    Enum(WrappedDefinition<EnumTypeDefinition>),
    Union(WrappedDefinition<UnionTypeDefinition>),
    Interface(WrappedDefinition<InterfaceTypeDefinition>),
}

impl TypeDefinition {
    fn mark(&self) {
        match self {
            Self::BuiltinScalar(_) => {}
            Self::CustomScalar(cstd) => cstd.mark(),
            Self::Object(otd) => otd.mark(),
            Self::InputObject(iotd) => iotd.mark(),
            Self::Enum(etd) => etd.mark(),
            Self::Union(utd) => utd.mark(),
            Self::Interface(itd) => itd.mark(),
        }
    }
}

impl CoreTypeDefinition for TypeDefinition {
    type CustomScalarTypeDefinition = CustomScalarTypeDefinition;
    type ObjectTypeDefinition = ObjectTypeDefinition;
    type InputObjectTypeDefinition = InputObjectTypeDefinition;
    type EnumTypeDefinition = EnumTypeDefinition;
    type UnionTypeDefinition = UnionTypeDefinition;
    type InterfaceTypeDefinition = InterfaceTypeDefinition;

    fn as_ref(&self) -> TypeDefinitionReference<'_, Self> {
        match self {
            Self::BuiltinScalar(bstd) => TypeDefinitionReference::BuiltinScalar(*bstd),
            Self::CustomScalar(cstd) => TypeDefinitionReference::CustomScalar(cstd.as_ref()),
            Self::Object(otd) => TypeDefinitionReference::Object(otd.as_ref()),
            Self::InputObject(iotd) => TypeDefinitionReference::InputObject(iotd.as_ref()),
            Self::Enum(etd) => TypeDefinitionReference::Enum(etd.as_ref()),
            Self::Union(utd) => TypeDefinitionReference::Union(utd.as_ref()),
            Self::Interface(itd) => TypeDefinitionReference::Interface(itd.as_ref()),
        }
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

impl From<&BaseInputType> for TypeDefinition {
    fn from(value: &BaseInputType) -> Self {
        match value {
            BaseInputType::BuiltinScalar(bstd) => Self::BuiltinScalar(*bstd),
            BaseInputType::CustomScalar(cstd) => Self::CustomScalar(cstd.clone()),
            BaseInputType::Enum(etd) => Self::Enum(etd.clone()),
            BaseInputType::InputObject(iotd) => Self::InputObject(iotd.clone()),
        }
    }
}

impl TryInto<BaseInputType> for &TypeDefinition {
    type Error = ();

    fn try_into(self) -> Result<BaseInputType, Self::Error> {
        match self {
            TypeDefinition::BuiltinScalar(bstd) => Ok(BaseInputType::BuiltinScalar(*bstd)),
            TypeDefinition::CustomScalar(cstd) => Ok(BaseInputType::CustomScalar(cstd.clone())),
            TypeDefinition::Enum(etd) => Ok(BaseInputType::Enum(etd.clone())),
            TypeDefinition::InputObject(iotd) => Ok(BaseInputType::InputObject(iotd.clone())),
            TypeDefinition::Interface(_) | TypeDefinition::Object(_) | TypeDefinition::Union(_) => {
                Err(())
            }
        }
    }
}

impl From<&BaseOutputType> for TypeDefinition {
    fn from(value: &BaseOutputType) -> Self {
        match value {
            BaseOutputType::BuiltinScalar(bstd) => Self::BuiltinScalar(*bstd),
            BaseOutputType::CustomScalar(cstd) => Self::CustomScalar(cstd.clone()),
            BaseOutputType::Enum(etd) => Self::Enum(etd.clone()),
            BaseOutputType::Object(otd) => Self::Object(otd.clone()),
            BaseOutputType::Interface(itd) => Self::Interface(itd.clone()),
            BaseOutputType::Union(utd) => Self::Union(utd.clone()),
        }
    }
}

impl IntoValue for TypeDefinition {
    fn into_value_with(self, handle: &Ruby) -> Value {
        match self {
            Self::BuiltinScalar(bstd) => Scalar::from(bstd).into_value_with(handle),
            Self::CustomScalar(cstd) => cstd.into_value_with(handle),
            Self::Enum(etd) => etd.into_value_with(handle),
            Self::InputObject(iotd) => iotd.into_value_with(handle),
            Self::Interface(itd) => itd.into_value_with(handle),
            Self::Object(otd) => otd.into_value_with(handle),
            Self::Union(utd) => utd.into_value_with(handle),
        }
    }
}

impl IntoValue for &TypeDefinition {
    fn into_value_with(self, handle: &Ruby) -> Value {
        self.clone().into_value_with(handle)
    }
}

struct SchemaTypeVisitor {
    types: BTreeMap<String, TypeDefinition>,
    directives: BTreeMap<String, WrappedDefinition<DirectiveDefinition>>,
}

impl From<SchemaTypeVisitor>
    for (
        BTreeMap<String, TypeDefinition>,
        BTreeMap<String, WrappedDefinition<DirectiveDefinition>>,
    )
{
    fn from(
        val: SchemaTypeVisitor,
    ) -> (
        BTreeMap<String, TypeDefinition>,
        BTreeMap<String, WrappedDefinition<DirectiveDefinition>>,
    ) {
        (val.types, val.directives)
    }
}

impl SchemaTypeVisitor {
    pub fn compute_contained_definitions(
        query: &WrappedDefinition<ObjectTypeDefinition>,
        mutation: Option<&WrappedDefinition<ObjectTypeDefinition>>,
        schema_directives: &Directives,
    ) -> (
        BTreeMap<String, TypeDefinition>,
        BTreeMap<String, WrappedDefinition<DirectiveDefinition>>,
    ) {
        let mut type_visitor = Self::new();
        type_visitor.visit_type(TypeDefinition::Object(query.clone()));
        if let Some(mutation) = mutation {
            type_visitor.visit_type(TypeDefinition::Object(mutation.clone()));
        }
        type_visitor.visit_directives(schema_directives);
        type_visitor.visit_builtin_directive_definitions();
        type_visitor.into()
    }

    fn new() -> Self {
        Self {
            types: BTreeMap::new(),
            directives: BTreeMap::new(),
        }
    }

    fn visit_object_type_definition(&mut self, otd: &ObjectTypeDefinition) {
        self.visit_field_definitions(otd.fields_definition());
        self.visit_directives(otd.directives());
    }

    fn visit_union_type_definition(&mut self, utd: &UnionTypeDefinition) {
        for union_member in utd.member_types().iter() {
            let t = TypeDefinition::Object(union_member.r#type());
            self.visit_type(t);
        }
        self.visit_directives(utd.directives());
    }

    fn visit_interface_type_definition(&mut self, itd: &InterfaceTypeDefinition) {
        self.visit_field_definitions(itd.fields_definition());
        self.visit_directives(itd.directives());
    }

    fn visit_input_object_type_definition(&mut self, iotd: &InputObjectTypeDefinition) {
        self.visit_input_value_definitions(iotd.input_fields_definition());
        self.visit_directives(iotd.directives());
    }

    fn visit_custom_scalar_type_definition(&mut self, cstd: &CustomScalarTypeDefinition) {
        self.visit_directives(cstd.directives());
    }

    fn visit_enum_type_definition(&mut self, etd: &EnumTypeDefinition) {
        etd.enum_value_definitions().iter().for_each(|evd| {
            self.visit_directives(evd.directives());
        });
        self.visit_directives(etd.directives());
    }

    fn visit_type(&mut self, t: TypeDefinition) {
        let name = t.as_ref().name().to_owned();
        match self.types.entry(name) {
            Entry::Occupied(_) => {}
            Entry::Vacant(entry) => {
                entry.insert(t.clone());
                match t {
                    TypeDefinition::BuiltinScalar(_) => {}
                    TypeDefinition::CustomScalar(cstd) => {
                        self.visit_custom_scalar_type_definition(cstd.as_ref());
                    }
                    TypeDefinition::Enum(etd) => {
                        self.visit_enum_type_definition(etd.as_ref());
                    }
                    TypeDefinition::Object(otd) => {
                        self.visit_object_type_definition(otd.as_ref());
                    }
                    TypeDefinition::Union(utd) => {
                        self.visit_union_type_definition(utd.as_ref());
                    }
                    TypeDefinition::Interface(itd) => {
                        self.visit_interface_type_definition(itd.as_ref());
                    }
                    TypeDefinition::InputObject(iotd) => {
                        self.visit_input_object_type_definition(iotd.as_ref());
                    }
                }
            }
        }
    }

    fn visit_field_definitions(&mut self, fields_definition: &FieldsDefinition) {
        for field_definition in fields_definition.iter() {
            self.visit_input_value_definitions(field_definition.argument_definitions());
            let base_type = field_definition.r#type().as_ref().base();
            self.visit_type(base_type.into());
            self.visit_directives(field_definition.directives());
        }
    }

    fn visit_input_value_definitions(&mut self, input_fields_definition: &InputFieldsDefinition) {
        for input_value_definition in input_fields_definition.iter() {
            let base_type = input_value_definition.r#type().as_ref().base();
            self.visit_type(base_type.into());
            self.visit_directives(input_value_definition.directives());
        }
    }

    fn visit_directives(&mut self, directives: &Directives) {
        directives.iter().for_each(|directive| {
            let definition = directive.definition();
            self.directives
                .entry(definition.as_ref().name().to_string())
                .or_insert_with(|| definition.clone());
        });
    }

    fn visit_builtin_directive_definitions(&mut self) {
        DirectiveDefinition::builtin_directive_definitions()
            .iter()
            .for_each(|definition| {
                self.directives
                    .entry(definition.as_ref().name().to_string())
                    .or_insert_with(|| definition.clone());
            })
    }
}

pub fn init() -> Result<(), Error> {
    let class = root().define_class("SchemaDefinition", Default::default())?;

    class.define_singleton_method("new", function!(SchemaDefinition::new, 1))?;
    class.define_method("execute", method!(SchemaDefinition::execute, 4))?;
    class.define_method(
        "validate_query",
        method!(SchemaDefinition::validate_query, 1),
    )?;
    class.define_method("to_definition", method!(SchemaDefinition::to_definition, 0))?;
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
