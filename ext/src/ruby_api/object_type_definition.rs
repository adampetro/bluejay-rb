use bluejay_core::AsIter;
use magnus::{function, Error, Module, Object, scan_args::get_kwargs, RHash, memoize, TypedData, RArray, DataTypeFunctions, RClass, method};
use super::{root, field_definition::FieldDefinition, interface_implementations::InterfaceImplementations, interface_implementation::InterfaceImplementation, interface_type_definition::InterfaceTypeDefinition, fields_definition::FieldsDefinition};
use crate::helpers::{WrappedStruct, from_rarray, HasDefinitionWrapper};

#[derive(Clone, Debug, TypedData)]
#[magnus(class = "Bluejay::ObjectTypeDefinition", mark)]
pub struct ObjectTypeDefinition {
    name: String,
    description: Option<String>,
    fields_definition: FieldsDefinition,
    interface_implementations: InterfaceImplementations,
}

impl ObjectTypeDefinition {
    fn new(kw: RHash) -> Result<Self, Error> {
        let args = get_kwargs(kw, &["name", "field_definitions", "interface_implementations", "description"], &[])?;
        let (name, field_definitions, interface_implementations, description): (String, RArray, RArray, Option<String>) = args.required;
        let _: () = args.optional;
        let _: () = args.splat;
        let fields_definition = FieldsDefinition::new(Some(field_definitions))?;
        let interface_implementations = InterfaceImplementations::new(interface_implementations)?;
        Ok(Self { name, description, fields_definition, interface_implementations })
    }
}

impl DataTypeFunctions for ObjectTypeDefinition {
    fn mark(&self) {
        self.fields_definition.mark();
        self.interface_implementations.mark();
    }
}

impl HasDefinitionWrapper for ObjectTypeDefinition {
    fn wrapping_class() -> RClass {
        *memoize!(RClass: root().define_class("ObjectType", Default::default()).unwrap())
    }
}

impl ObjectTypeDefinition {
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_ref().map(String::as_str)
    }

    pub fn field_definitions(&self) -> &[WrappedStruct<FieldDefinition>] {
        self.fields_definition.as_ref()
    }

    pub fn interface_implementations(&self) -> &[WrappedStruct<InterfaceImplementation>] {
        self.interface_implementations.as_ref()
    }

    pub fn implements_interface(&self, interface: &InterfaceTypeDefinition) -> bool {
        self.interface_implementations
            .iter()
            .any(|ii| ii.interface().get().name() == interface.name())
    }

    pub fn field_definition(&self, name: &str) -> Option<&FieldDefinition> {
        self.fields_definition
            .iter()
            .find(|fd| fd.name() == name)
    }
}

impl bluejay_core::definition::ObjectTypeDefinition for ObjectTypeDefinition {
    type FieldsDefinition = FieldsDefinition;
    type InterfaceImplementations = InterfaceImplementations;

    fn description(&self) -> Option<&str> {
        self.description.as_ref().map(String::as_str)
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn fields_definition(&self) -> &Self::FieldsDefinition {
        &self.fields_definition
    }

    fn interface_impelementations(&self) -> &Self::InterfaceImplementations {
        &self.interface_implementations
    }
}

pub fn init() -> Result<(), Error> {
    let class = root().define_class("ObjectTypeDefinition", Default::default())?;

    class.define_singleton_method("new", function!(ObjectTypeDefinition::new, 1))?;
    class.define_method("name", method!(ObjectTypeDefinition::name, 0))?;

    Ok(())
}
