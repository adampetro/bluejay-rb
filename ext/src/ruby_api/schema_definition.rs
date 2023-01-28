use std::collections::{HashMap, hash_map::Entry};
use super::object_type_definition::ObjectTypeDefinition;
use super::input_object_type_definition::InputObjectTypeDefinition;
use super::enum_type_definition::EnumTypeDefinition;
use super::union_type_definition::UnionTypeDefinition;
use super::interface_type_definition::InterfaceTypeDefinition;
use super::custom_scalar_type_definition::CustomScalarTypeDefinition;
use super::input_value_definition::InputValueDefinition;
use super::field_definition::FieldDefinition;
use super::input_fields_definition::InputFieldsDefinition;
use super::arguments_definition::ArgumentsDefinition;
use super::fields_definition::FieldsDefinition;
use super::interface_implementation::InterfaceImplementation;
use super::interface_implementations::InterfaceImplementations;
use super::union_member_type::UnionMemberType;
use super::union_member_types::UnionMemberTypes;
use super::validation_error::ValidationError;
use super::{root, ExecutionResult, BaseInputTypeReference, InputTypeReference, BaseOutputTypeReference, OutputTypeReference};
use crate::helpers::{WrappedStruct, WrappedDefinition};
use crate::execution::{Engine as ExecutionEngine};
use magnus::{function, Error, Module, Object, scan_args::get_kwargs, RHash, RArray, Value, TypedData, DataTypeFunctions, method};
use bluejay_core::validation::executable::Validator;
use bluejay_core::{BuiltinScalarDefinition, IntoEnumIterator, AsIter};

#[derive(Clone, Debug, TypedData)]
#[magnus(class = "Bluejay::SchemaDefinition", mark)]
pub struct SchemaDefinition {
    description: Option<String>,
    query: WrappedDefinition<ObjectTypeDefinition>,
    mutation: Option<WrappedDefinition<ObjectTypeDefinition>>,
    contained_types: HashMap<String, TypeDefinitionReference>,
}

impl SchemaDefinition {
    pub fn new(kw: RHash) -> Result<Self, Error> {
        let args = get_kwargs(kw, &["description", "query", "mutation"], &[])?;
        let (description, query, mutation): (Option<String>, WrappedDefinition<ObjectTypeDefinition>, Option<WrappedDefinition<ObjectTypeDefinition>>) = args.required;
        let _: () = args.optional;
        let _: () = args.splat;
        let contained_types = SchemaTypeVisitor::compute_contained_types(&query, mutation.as_ref());
        Ok(Self { description, query, mutation, contained_types })
    }

    pub fn query(&self) -> WrappedStruct<ObjectTypeDefinition> {
        *self.query.get()
    }

    pub fn r#type(&self, name: &str) -> Option<&TypeDefinitionReference> {
        self.contained_types.get(name)
    }

    fn execute(&self, query: String, operation_name: Option<String>, variable_values: RHash, initial_value: Value) -> Result<ExecutionResult, Error> {
        ExecutionEngine::execute_request(self, query.as_str(), operation_name.as_ref().map(String::as_str), variable_values, initial_value)
    }

    fn validate_query(&self, query: String) -> RArray {
        let (document, _) = bluejay_parser::parse(query.as_str());

        RArray::from_iter(
            Validator::validate(&document, self)
                .map(|error| -> WrappedStruct<ValidationError> {
                    WrappedStruct::wrap(error.into())
                })
        )
    }
}

impl DataTypeFunctions for SchemaDefinition {
    fn mark(&self) {
        self.query.mark();
        if let Some(mutation) = &self.mutation {
            mutation.mark();
        }
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

    fn description(&self) -> Option<&str> {
        self.description.as_ref().map(String::as_str)
    }

    fn query(&self) -> &Self::ObjectTypeDefinition {
        self.query.as_ref()
    }

    fn mutation(&self) -> Option<&Self::ObjectTypeDefinition> {
        self.mutation.as_ref().map(AsRef::as_ref)
    }

    fn get_type(&self, name: &str) -> Option<&Self::TypeDefinitionReference> {
        self.contained_types.get(name)
    }
}

impl TryFrom<&BaseInputTypeReference> for TypeDefinitionReference {
    type Error = ();

    fn try_from(value: &BaseInputTypeReference) -> Result<Self, Self::Error> {
        match value {
            BaseInputTypeReference::BuiltinScalarType(_) => Err(()),
            BaseInputTypeReference::CustomScalarType(cstd) => Ok(Self::CustomScalarType(cstd.clone(), Default::default())),
            BaseInputTypeReference::EnumType(etd) => Ok(Self::EnumType(etd.clone(), Default::default())),
            BaseInputTypeReference::InputObjectType(iotd) => Ok(Self::InputObjectType(iotd.clone(), Default::default())),
        }
    }
}

impl TryInto<BaseInputTypeReference> for &TypeDefinitionReference {
    type Error = ();

    fn try_into(self) -> Result<BaseInputTypeReference, Self::Error> {
        match self {
            TypeDefinitionReference::BuiltinScalarType(bstd) => Ok(BaseInputTypeReference::BuiltinScalarType(*bstd)),
            TypeDefinitionReference::CustomScalarType(cstd, _) => Ok(BaseInputTypeReference::CustomScalarType(cstd.clone())),
            TypeDefinitionReference::EnumType(etd, _) => Ok(BaseInputTypeReference::EnumType(etd.clone())),
            TypeDefinitionReference::InputObjectType(iotd, _) => Ok(BaseInputTypeReference::InputObjectType(iotd.clone())),
            TypeDefinitionReference::InterfaceType(_, _) | TypeDefinitionReference::ObjectType(_, _) | TypeDefinitionReference::UnionType(_, _) => Err(()),
        }
    }
}

impl TryFrom<&BaseOutputTypeReference> for TypeDefinitionReference {
    type Error = ();

    fn try_from(value: &BaseOutputTypeReference) -> Result<Self, Self::Error> {
        match value {
            BaseOutputTypeReference::BuiltinScalarType(_) => Err(()),
            BaseOutputTypeReference::CustomScalarType(cstd) => Ok(Self::CustomScalarType(cstd.clone(), Default::default())),
            BaseOutputTypeReference::EnumType(etd) => Ok(Self::EnumType(etd.clone(), Default::default())),
            BaseOutputTypeReference::ObjectType(otd) => Ok(Self::ObjectType(otd.clone(), Default::default())),
            BaseOutputTypeReference::InterfaceType(itd) => Ok(Self::InterfaceType(itd.clone(), Default::default())),
            BaseOutputTypeReference::UnionType(utd) => Ok(Self::UnionType(utd.clone(), Default::default())),
        }
    }
}

struct SchemaTypeVisitor {
    types: HashMap<String, TypeDefinitionReference>,
}

impl Into<HashMap<String, TypeDefinitionReference>> for SchemaTypeVisitor {
    fn into(self) -> HashMap<String, TypeDefinitionReference> {
        self.types
    }
}

impl SchemaTypeVisitor {
    pub fn compute_contained_types(query: &WrappedDefinition<ObjectTypeDefinition>, mutation: Option<&WrappedDefinition<ObjectTypeDefinition>>) -> HashMap<String, TypeDefinitionReference> {
        let mut type_visitor = Self::new();
        type_visitor.visit_type(TypeDefinitionReference::ObjectType(query.clone(), Default::default()));
        if let Some(mutation) = mutation {
            type_visitor.visit_type(TypeDefinitionReference::ObjectType(mutation.clone(), Default::default()));
        }
        type_visitor.visit_builtin_scalar_definitions();
        type_visitor.into()
    }

    fn new() -> Self {
        Self { types: HashMap::new() }
    }

    fn visit_object_type_definition(&mut self, otd: &ObjectTypeDefinition) {
        self.visit_field_definitions(otd.fields_definition())
    }

    fn visit_union_type_definition(&mut self, utd: &UnionTypeDefinition) {
        for union_member in utd.member_types().iter() {
            let t = TypeDefinitionReference::ObjectType(union_member.r#type(), Default::default());
            self.visit_type(t);
        }
    }

    fn visit_interface_type_definition(&mut self, itd: &InterfaceTypeDefinition) {
        self.visit_field_definitions(itd.fields_definition())
    }

    fn visit_input_object_type_definition(&mut self, iotd: &InputObjectTypeDefinition) {
        self.visit_input_value_definitions(iotd.input_fields_definition())
    }

    fn visit_type(&mut self, t: TypeDefinitionReference) {
        let name = t.name().to_owned();
        match self.types.entry(name) {
            Entry::Occupied(_) => {},
            Entry::Vacant(entry) => {
                entry.insert(t.clone());
                match t {
                    TypeDefinitionReference::BuiltinScalarType(_) | TypeDefinitionReference::CustomScalarType(_, _) | TypeDefinitionReference::EnumType(_, _) => {},
                    TypeDefinitionReference::ObjectType(otd, _) => self.visit_object_type_definition(otd.as_ref()),
                    TypeDefinitionReference::UnionType(utd, _) => self.visit_union_type_definition(utd.as_ref()),
                    TypeDefinitionReference::InterfaceType(itd, _) => self.visit_interface_type_definition(itd.as_ref()),
                    TypeDefinitionReference::InputObjectType(iotd, _) => self.visit_input_object_type_definition(iotd.as_ref()),
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
        }
    }

    fn visit_input_value_definitions(&mut self, input_fields_definition: &InputFieldsDefinition) {
        for input_value_definition in input_fields_definition.iter() {
            let base_type = input_value_definition.r#type().as_ref().base();
            let t: Result<TypeDefinitionReference, ()> = base_type.try_into();
            if let Ok(t) = t {
                self.visit_type(t);
            }
        }
    }

    fn visit_builtin_scalar_definitions(&mut self) {
        BuiltinScalarDefinition::iter().for_each(|bisd| {
            self.visit_type(TypeDefinitionReference::BuiltinScalarType(bisd));
        })
    }
}

pub fn init() -> Result<(), Error> {
    let class = root().define_class("SchemaDefinition", Default::default())?;

    class.define_singleton_method("new", function!(SchemaDefinition::new, 1))?;
    class.define_method("execute", method!(SchemaDefinition::execute, 4))?;
    class.define_method("validate_query", method!(SchemaDefinition::validate_query, 1))?;

    Ok(())
}
