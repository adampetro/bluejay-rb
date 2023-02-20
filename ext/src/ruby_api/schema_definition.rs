use super::arguments_definition::ArgumentsDefinition;
use super::custom_scalar_type_definition::CustomScalarTypeDefinition;
use super::enum_type_definition::EnumTypeDefinition;
use super::field_definition::FieldDefinition;
use super::fields_definition::FieldsDefinition;
use super::input_fields_definition::InputFieldsDefinition;
use super::input_object_type_definition::InputObjectTypeDefinition;
use super::input_value_definition::InputValueDefinition;
use super::interface_implementation::InterfaceImplementation;
use super::interface_implementations::InterfaceImplementations;
use super::interface_type_definition::InterfaceTypeDefinition;
use super::object_type_definition::ObjectTypeDefinition;
use super::union_member_type::UnionMemberType;
use super::union_member_types::UnionMemberTypes;
use super::union_type_definition::UnionTypeDefinition;
use super::validation_error::ValidationError;
use crate::execution::Engine as ExecutionEngine;
use crate::helpers::WrappedDefinition;
use crate::ruby_api::{
    root, BaseInputTypeReference, BaseOutputTypeReference, DirectiveDefinition, Directives,
    ExecutionResult, InputTypeReference, OutputTypeReference,
};
use bluejay_core::definition::{
    BaseInputTypeReference as CoreBaseInputTypeReference,
    BaseOutputTypeReference as CoreBaseOutputTypeReference,
};
use bluejay_core::validation::executable::RulesValidator;
use bluejay_core::{AsIter, BuiltinScalarDefinition, IntoEnumIterator};
use magnus::{
    function, method, scan_args::get_kwargs, scan_args::KwArgs, typed_data::Obj, DataTypeFunctions,
    Error, Module, Object, RArray, RHash, TypedData, Value,
};
use std::collections::{hash_map::Entry, HashMap};

#[derive(Debug, TypedData)]
#[magnus(class = "Bluejay::SchemaDefinition", mark)]
pub struct SchemaDefinition {
    description: Option<String>,
    query: WrappedDefinition<ObjectTypeDefinition>,
    mutation: Option<WrappedDefinition<ObjectTypeDefinition>>,
    directives: Directives,
    contained_types: HashMap<String, TypeDefinitionReference>,
    contained_directives: HashMap<String, WrappedDefinition<DirectiveDefinition>>,
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
        Ok(Self {
            description,
            query,
            mutation,
            directives,
            contained_types,
            contained_directives,
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
        let (document, _) =
            bluejay_parser::ast::executable::ExecutableDocument::parse(query.as_str());

        RArray::from_iter(
            RulesValidator::validate(&document, self)
                .map(|error| -> Obj<ValidationError> { Obj::wrap(error.into()) }),
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

pub type TypeDefinitionReference = bluejay_core::definition::TypeDefinitionReference<
    CustomScalarTypeDefinition,
    WrappedDefinition<CustomScalarTypeDefinition>,
    ObjectTypeDefinition,
    WrappedDefinition<ObjectTypeDefinition>,
    InputObjectTypeDefinition,
    WrappedDefinition<InputObjectTypeDefinition>,
    EnumTypeDefinition,
    WrappedDefinition<EnumTypeDefinition>,
    UnionTypeDefinition,
    WrappedDefinition<UnionTypeDefinition>,
    InterfaceTypeDefinition,
    WrappedDefinition<InterfaceTypeDefinition>,
>;

impl<'a> bluejay_core::definition::SchemaDefinition<'a> for SchemaDefinition {
    type InputValueDefinition = InputValueDefinition;
    type InputFieldsDefinition = InputFieldsDefinition;
    type ArgumentsDefinition = ArgumentsDefinition;
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

    fn get_directive(&self, name: &str) -> Option<&Self::DirectiveDefinition> {
        self.contained_directives.get(name).map(AsRef::as_ref)
    }

    fn get_type(&self, name: &str) -> Option<&Self::TypeDefinitionReference> {
        self.contained_types.get(name)
    }
}

impl TryFrom<&BaseInputTypeReference> for TypeDefinitionReference {
    type Error = ();

    fn try_from(value: &BaseInputTypeReference) -> Result<Self, Self::Error> {
        match value.as_ref() {
            CoreBaseInputTypeReference::BuiltinScalarType(_) => Err(()),
            CoreBaseInputTypeReference::CustomScalarType(cstd, _) => {
                Ok(Self::CustomScalarType(cstd.clone(), Default::default()))
            }
            CoreBaseInputTypeReference::EnumType(etd, _) => {
                Ok(Self::EnumType(etd.clone(), Default::default()))
            }
            CoreBaseInputTypeReference::InputObjectType(iotd, _) => {
                Ok(Self::InputObjectType(iotd.clone(), Default::default()))
            }
        }
    }
}

impl TryInto<BaseInputTypeReference> for &TypeDefinitionReference {
    type Error = ();

    fn try_into(self) -> Result<BaseInputTypeReference, Self::Error> {
        match self {
            TypeDefinitionReference::BuiltinScalarType(bstd) => {
                Ok(CoreBaseInputTypeReference::BuiltinScalarType(*bstd).into())
            }
            TypeDefinitionReference::CustomScalarType(cstd, _) => Ok(
                CoreBaseInputTypeReference::CustomScalarType(cstd.clone(), Default::default())
                    .into(),
            ),
            TypeDefinitionReference::EnumType(etd, _) => {
                Ok(CoreBaseInputTypeReference::EnumType(etd.clone(), Default::default()).into())
            }
            TypeDefinitionReference::InputObjectType(iotd, _) => Ok(
                CoreBaseInputTypeReference::InputObjectType(iotd.clone(), Default::default())
                    .into(),
            ),
            TypeDefinitionReference::InterfaceType(_, _)
            | TypeDefinitionReference::ObjectType(_, _)
            | TypeDefinitionReference::UnionType(_, _) => Err(()),
        }
    }
}

impl TryFrom<&BaseOutputTypeReference> for TypeDefinitionReference {
    type Error = ();

    fn try_from(value: &BaseOutputTypeReference) -> Result<Self, Self::Error> {
        match value.as_ref() {
            CoreBaseOutputTypeReference::BuiltinScalarType(_) => Err(()),
            CoreBaseOutputTypeReference::CustomScalarType(cstd, _) => {
                Ok(Self::CustomScalarType(cstd.clone(), Default::default()))
            }
            CoreBaseOutputTypeReference::EnumType(etd, _) => {
                Ok(Self::EnumType(etd.clone(), Default::default()))
            }
            CoreBaseOutputTypeReference::ObjectType(otd, _) => {
                Ok(Self::ObjectType(otd.clone(), Default::default()))
            }
            CoreBaseOutputTypeReference::InterfaceType(itd, _) => {
                Ok(Self::InterfaceType(itd.clone(), Default::default()))
            }
            CoreBaseOutputTypeReference::UnionType(utd, _) => {
                Ok(Self::UnionType(utd.clone(), Default::default()))
            }
        }
    }
}

struct SchemaTypeVisitor {
    types: HashMap<String, TypeDefinitionReference>,
    directives: HashMap<String, WrappedDefinition<DirectiveDefinition>>,
}

impl From<SchemaTypeVisitor>
    for (
        HashMap<String, TypeDefinitionReference>,
        HashMap<String, WrappedDefinition<DirectiveDefinition>>,
    )
{
    fn from(
        val: SchemaTypeVisitor,
    ) -> (
        HashMap<String, TypeDefinitionReference>,
        HashMap<String, WrappedDefinition<DirectiveDefinition>>,
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
        HashMap<String, TypeDefinitionReference>,
        HashMap<String, WrappedDefinition<DirectiveDefinition>>,
    ) {
        let mut type_visitor = Self::new();
        type_visitor.visit_type(TypeDefinitionReference::ObjectType(
            query.clone(),
            Default::default(),
        ));
        if let Some(mutation) = mutation {
            type_visitor.visit_type(TypeDefinitionReference::ObjectType(
                mutation.clone(),
                Default::default(),
            ));
        }
        type_visitor.visit_directives(schema_directives);
        type_visitor.visit_builtin_scalar_definitions();
        type_visitor.visit_builtin_directive_definitions();
        type_visitor.into()
    }

    fn new() -> Self {
        Self {
            types: HashMap::new(),
            directives: HashMap::new(),
        }
    }

    fn visit_object_type_definition(&mut self, otd: &ObjectTypeDefinition) {
        self.visit_field_definitions(otd.fields_definition());
        self.visit_directives(otd.directives());
    }

    fn visit_union_type_definition(&mut self, utd: &UnionTypeDefinition) {
        for union_member in utd.member_types().iter() {
            let t = TypeDefinitionReference::ObjectType(union_member.r#type(), Default::default());
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
        let name = t.name().to_owned();
        match self.types.entry(name) {
            Entry::Occupied(_) => {}
            Entry::Vacant(entry) => {
                entry.insert(t.clone());
                match t {
                    TypeDefinitionReference::BuiltinScalarType(_) => {}
                    TypeDefinitionReference::CustomScalarType(cstd, _) => {
                        self.visit_custom_scalar_type_definition(cstd.as_ref());
                    }
                    TypeDefinitionReference::EnumType(etd, _) => {
                        self.visit_enum_type_definition(etd.as_ref());
                    }
                    TypeDefinitionReference::ObjectType(otd, _) => {
                        self.visit_object_type_definition(otd.as_ref());
                    }
                    TypeDefinitionReference::UnionType(utd, _) => {
                        self.visit_union_type_definition(utd.as_ref());
                    }
                    TypeDefinitionReference::InterfaceType(itd, _) => {
                        self.visit_interface_type_definition(itd.as_ref());
                    }
                    TypeDefinitionReference::InputObjectType(iotd, _) => {
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
        directives.as_ref().iter().for_each(|directive| {
            let definition = directive.definition();
            self.directives
                .entry(definition.as_ref().name().to_string())
                .or_insert_with(|| definition.clone());
        });
    }

    fn visit_builtin_scalar_definitions(&mut self) {
        BuiltinScalarDefinition::iter().for_each(|bisd| {
            self.visit_type(TypeDefinitionReference::BuiltinScalarType(bisd));
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

    Ok(())
}
