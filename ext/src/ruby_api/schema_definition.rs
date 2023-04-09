use crate::execution::Engine as ExecutionEngine;
use crate::helpers::WrappedDefinition;
use crate::ruby_api::{
    root, BaseInputTypeReference, BaseOutputTypeReference, DirectiveDefinition, Directives,
    ExecutionResult, InputTypeReference, OutputTypeReference,
};
use crate::ruby_api::{
    ArgumentsDefinition, CustomScalarTypeDefinition, EnumTypeDefinition, EnumValueDefinition,
    EnumValueDefinitions, FieldDefinition, FieldsDefinition, InputFieldsDefinition,
    InputObjectTypeDefinition, InputValueDefinition, InterfaceImplementation,
    InterfaceImplementations, InterfaceTypeDefinition, ObjectTypeDefinition, UnionMemberType,
    UnionMemberTypes, UnionTypeDefinition, ValidationError,
};
use bluejay_core::definition::{
    AbstractTypeDefinitionReference, TypeDefinitionReference as CoreTypeDefinitionReference,
    TypeDefinitionReferenceFromAbstract,
};
use bluejay_core::{AsIter, BuiltinScalarDefinition, IntoEnumIterator};
use bluejay_printer::definition::DisplaySchemaDefinition;
use bluejay_validator::executable::RulesValidator;
use magnus::{
    function, method, scan_args::get_kwargs, scan_args::KwArgs, typed_data::Obj, DataTypeFunctions,
    Error, Module, Object, RArray, RHash, TypedData, Value,
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
    contained_types: BTreeMap<String, TypeDefinitionReference>,
    contained_directives: BTreeMap<String, WrappedDefinition<DirectiveDefinition>>,
    interface_implementors: HashMap<String, Vec<WrappedDefinition<ObjectTypeDefinition>>>,
}

impl SchemaDefinition {
    pub fn new(kw: RHash) -> Result<Self, Error> {
        let args: KwArgs<_, (), ()> =
            get_kwargs(kw, &["description", "query", "mutation", "directives"], &[])?;
        let (description, query, mutation, directives): (
            Option<String>,
            WrappedDefinition<ObjectTypeDefinition>,
            Option<WrappedDefinition<ObjectTypeDefinition>>,
            RArray,
        ) = args.required;
        let directives = directives.try_into()?;
        let (contained_types, contained_directives) =
            SchemaTypeVisitor::compute_contained_definitions(
                &query,
                mutation.as_ref(),
                &directives,
            );
        let interface_implementors = Self::interface_implementors(&contained_types);
        Ok(Self {
            description,
            query,
            mutation,
            directives,
            contained_types,
            contained_directives,
            interface_implementors,
        })
    }

    pub fn query(&self) -> Obj<ObjectTypeDefinition> {
        *self.query.get()
    }

    pub fn r#type(&self, name: &str) -> Option<&TypeDefinitionReference> {
        self.contained_types.get(name)
    }

    pub fn directive(&self, name: &str) -> Option<Obj<DirectiveDefinition>> {
        self.contained_directives.get(name).map(|wd| *wd.get())
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
            RArray::from_iter(
                RulesValidator::validate(&document, self)
                    .map(|error| -> Obj<ValidationError> { Obj::wrap(error.into()) }),
            )
        } else {
            RArray::new()
        }
    }

    fn to_definition(&self) -> String {
        DisplaySchemaDefinition::to_string(self)
    }

    fn interface_implementors(
        type_definitions: &BTreeMap<String, TypeDefinitionReference>,
    ) -> HashMap<String, Vec<WrappedDefinition<ObjectTypeDefinition>>> {
        type_definitions.values().fold(
            HashMap::new(),
            |mut interface_implementors, type_definition| {
                if let TypeDefinitionReference::Object(otd) = type_definition {
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
}

impl DataTypeFunctions for SchemaDefinition {
    fn mark(&self) {
        self.query.mark();
        if let Some(mutation) = &self.mutation {
            mutation.mark();
        }
        self.directives.mark();
    }
}

#[derive(Debug, Clone)]
pub enum TypeDefinitionReference {
    BuiltinScalar(BuiltinScalarDefinition),
    CustomScalar(WrappedDefinition<CustomScalarTypeDefinition>),
    Object(WrappedDefinition<ObjectTypeDefinition>),
    InputObject(WrappedDefinition<InputObjectTypeDefinition>),
    Enum(WrappedDefinition<EnumTypeDefinition>),
    Union(WrappedDefinition<UnionTypeDefinition>),
    Interface(WrappedDefinition<InterfaceTypeDefinition>),
}

impl AbstractTypeDefinitionReference for TypeDefinitionReference {
    type CustomScalarTypeDefinition = CustomScalarTypeDefinition;
    type ObjectTypeDefinition = ObjectTypeDefinition;
    type InputObjectTypeDefinition = InputObjectTypeDefinition;
    type EnumTypeDefinition = EnumTypeDefinition;
    type UnionTypeDefinition = UnionTypeDefinition;
    type InterfaceTypeDefinition = InterfaceTypeDefinition;

    fn get(&self) -> TypeDefinitionReferenceFromAbstract<'_, Self> {
        match self {
            Self::BuiltinScalar(bstd) => CoreTypeDefinitionReference::BuiltinScalarType(*bstd),
            Self::CustomScalar(cstd) => {
                CoreTypeDefinitionReference::CustomScalarType(cstd.as_ref())
            }
            Self::Object(otd) => CoreTypeDefinitionReference::ObjectType(otd.as_ref()),
            Self::InputObject(iotd) => CoreTypeDefinitionReference::InputObjectType(iotd.as_ref()),
            Self::Enum(etd) => CoreTypeDefinitionReference::EnumType(etd.as_ref()),
            Self::Union(utd) => CoreTypeDefinitionReference::UnionType(utd.as_ref()),
            Self::Interface(itd) => CoreTypeDefinitionReference::InterfaceType(itd.as_ref()),
        }
    }
}

impl bluejay_core::definition::SchemaDefinition for SchemaDefinition {
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
    type BaseInputTypeReference = BaseInputTypeReference;
    type InputTypeReference = InputTypeReference;
    type BaseOutputTypeReference = BaseOutputTypeReference;
    type OutputTypeReference = OutputTypeReference;
    type CustomScalarTypeDefinition = CustomScalarTypeDefinition;
    type ObjectTypeDefinition = ObjectTypeDefinition;
    type InterfaceTypeDefinition = InterfaceTypeDefinition;
    type UnionTypeDefinition = UnionTypeDefinition;
    type InputObjectTypeDefinition = InputObjectTypeDefinition;
    type EnumTypeDefinition = EnumTypeDefinition;
    type TypeDefinitionReference = TypeDefinitionReference;
    type DirectiveDefinition = DirectiveDefinition;
    type Directives = Directives;
    type TypeDefinitionReferences<'a> = std::iter::Map<
        Values<'a, String, TypeDefinitionReference>,
        fn(
            &'a TypeDefinitionReference,
        ) -> TypeDefinitionReferenceFromAbstract<'a, TypeDefinitionReference>,
    >;
    type DirectiveDefinitions<'a> = std::iter::Map<
        Values<'a, String, WrappedDefinition<DirectiveDefinition>>,
        fn(&WrappedDefinition<DirectiveDefinition>) -> &DirectiveDefinition,
    >;
    type IterfaceImplementors<'a> = std::iter::Flatten<
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
        Some(&self.directives)
    }

    fn get_directive_definition(&self, name: &str) -> Option<&Self::DirectiveDefinition> {
        self.contained_directives.get(name).map(AsRef::as_ref)
    }

    fn get_type_definition(
        &self,
        name: &str,
    ) -> Option<TypeDefinitionReferenceFromAbstract<'_, TypeDefinitionReference>> {
        self.contained_types
            .get(name)
            .map(AbstractTypeDefinitionReference::get)
    }

    fn type_definitions(&self) -> Self::TypeDefinitionReferences<'_> {
        self.contained_types
            .values()
            .map(AbstractTypeDefinitionReference::get)
    }

    fn directive_definitions(&self) -> Self::DirectiveDefinitions<'_> {
        self.contained_directives.values().map(AsRef::as_ref)
    }

    fn get_interface_implementors(
        &self,
        itd: &Self::InterfaceTypeDefinition,
    ) -> Self::IterfaceImplementors<'_> {
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

impl TryFrom<&BaseInputTypeReference> for TypeDefinitionReference {
    type Error = ();

    fn try_from(value: &BaseInputTypeReference) -> Result<Self, Self::Error> {
        match value {
            BaseInputTypeReference::BuiltinScalar(_) => Err(()),
            BaseInputTypeReference::CustomScalar(cstd) => Ok(Self::CustomScalar(cstd.clone())),
            BaseInputTypeReference::Enum(etd) => Ok(Self::Enum(etd.clone())),
            BaseInputTypeReference::InputObject(iotd) => Ok(Self::InputObject(iotd.clone())),
        }
    }
}

impl TryInto<BaseInputTypeReference> for &TypeDefinitionReference {
    type Error = ();

    fn try_into(self) -> Result<BaseInputTypeReference, Self::Error> {
        match self {
            TypeDefinitionReference::BuiltinScalar(bstd) => {
                Ok(BaseInputTypeReference::BuiltinScalar(*bstd))
            }
            TypeDefinitionReference::CustomScalar(cstd) => {
                Ok(BaseInputTypeReference::CustomScalar(cstd.clone()))
            }
            TypeDefinitionReference::Enum(etd) => Ok(BaseInputTypeReference::Enum(etd.clone())),
            TypeDefinitionReference::InputObject(iotd) => {
                Ok(BaseInputTypeReference::InputObject(iotd.clone()))
            }
            TypeDefinitionReference::Interface(_)
            | TypeDefinitionReference::Object(_)
            | TypeDefinitionReference::Union(_) => Err(()),
        }
    }
}

impl TryFrom<&BaseOutputTypeReference> for TypeDefinitionReference {
    type Error = ();

    fn try_from(value: &BaseOutputTypeReference) -> Result<Self, Self::Error> {
        match value {
            BaseOutputTypeReference::BuiltinScalar(_) => Err(()),
            BaseOutputTypeReference::CustomScalar(cstd) => Ok(Self::CustomScalar(cstd.clone())),
            BaseOutputTypeReference::Enum(etd) => Ok(Self::Enum(etd.clone())),
            BaseOutputTypeReference::Object(otd) => Ok(Self::Object(otd.clone())),
            BaseOutputTypeReference::Interface(itd) => Ok(Self::Interface(itd.clone())),
            BaseOutputTypeReference::Union(utd) => Ok(Self::Union(utd.clone())),
        }
    }
}

struct SchemaTypeVisitor {
    types: BTreeMap<String, TypeDefinitionReference>,
    directives: BTreeMap<String, WrappedDefinition<DirectiveDefinition>>,
}

impl From<SchemaTypeVisitor>
    for (
        BTreeMap<String, TypeDefinitionReference>,
        BTreeMap<String, WrappedDefinition<DirectiveDefinition>>,
    )
{
    fn from(
        val: SchemaTypeVisitor,
    ) -> (
        BTreeMap<String, TypeDefinitionReference>,
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
        BTreeMap<String, TypeDefinitionReference>,
        BTreeMap<String, WrappedDefinition<DirectiveDefinition>>,
    ) {
        let mut type_visitor = Self::new();
        type_visitor.visit_type(TypeDefinitionReference::Object(query.clone()));
        if let Some(mutation) = mutation {
            type_visitor.visit_type(TypeDefinitionReference::Object(mutation.clone()));
        }
        type_visitor.visit_directives(schema_directives);
        type_visitor.visit_builtin_scalar_definitions();
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
            let t = TypeDefinitionReference::Object(union_member.r#type());
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

    fn visit_type(&mut self, t: TypeDefinitionReference) {
        let name = t.get().name().to_owned();
        match self.types.entry(name) {
            Entry::Occupied(_) => {}
            Entry::Vacant(entry) => {
                entry.insert(t.clone());
                match t {
                    TypeDefinitionReference::BuiltinScalar(_) => {}
                    TypeDefinitionReference::CustomScalar(cstd) => {
                        self.visit_custom_scalar_type_definition(cstd.as_ref());
                    }
                    TypeDefinitionReference::Enum(etd) => {
                        self.visit_enum_type_definition(etd.as_ref());
                    }
                    TypeDefinitionReference::Object(otd) => {
                        self.visit_object_type_definition(otd.as_ref());
                    }
                    TypeDefinitionReference::Union(utd) => {
                        self.visit_union_type_definition(utd.as_ref());
                    }
                    TypeDefinitionReference::Interface(itd) => {
                        self.visit_interface_type_definition(itd.as_ref());
                    }
                    TypeDefinitionReference::InputObject(iotd) => {
                        self.visit_input_object_type_definition(iotd.as_ref());
                    }
                }
            }
        }
    }

    fn visit_field_definitions(&mut self, fields_definition: &FieldsDefinition) {
        for field_definition in fields_definition.iter() {
            self.visit_input_value_definitions(field_definition.argument_definitions());
            let base_type = field_definition.r#type().base();
            let t: Result<TypeDefinitionReference, ()> = base_type.try_into();
            if let Ok(t) = t {
                self.visit_type(t);
            }
            self.visit_directives(field_definition.directives());
        }
    }

    fn visit_input_value_definitions(&mut self, input_fields_definition: &InputFieldsDefinition) {
        for input_value_definition in input_fields_definition.iter() {
            let base_type = input_value_definition.r#type().base();
            let t: Result<TypeDefinitionReference, ()> = base_type.try_into();
            if let Ok(t) = t {
                self.visit_type(t);
            }
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

    fn visit_builtin_scalar_definitions(&mut self) {
        BuiltinScalarDefinition::iter().for_each(|bisd| {
            self.visit_type(TypeDefinitionReference::BuiltinScalar(bisd));
        })
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

    Ok(())
}
