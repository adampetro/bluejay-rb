use magnus::{function, Error, Module, Object, scan_args::get_kwargs, RHash, Value, memoize, TypedData, RArray, DataTypeFunctions, RClass, gc};
use super::{root, field_definition::FieldDefinition, interface_implementations::InterfaceImplementations, interface_implementation::InterfaceImplementation, fields_definition::FieldsDefinition};
use crate::helpers::{WrappedStruct, HasDefinitionWrapper};
use bluejay_core::AsIter;

#[derive(Clone, Debug, TypedData)]
#[magnus(class = "Bluejay::InterfaceTypeDefinition", mark)]
pub struct InterfaceTypeDefinition {
    name: String,
    description: Option<String>,
    fields_definition: FieldsDefinition,
    interface_implementations: InterfaceImplementations,
}

impl InterfaceTypeDefinition {
    fn new(kw: RHash) -> Result<Self, Error> {
        let args = get_kwargs(kw, &["name", "field_definitions", "interface_implementations", "description"], &[])?;
        let (name, field_definitions, interface_implementations, description): (String, RArray, RArray, Option<String>) = args.required;
        let _: () = args.optional;
        let _: () = args.splat;
        let fields_definition = FieldsDefinition::new(Some(field_definitions))?;
        let interface_implementations = InterfaceImplementations::new(interface_implementations)?;
        Ok(Self { name, description, fields_definition, interface_implementations })
    }

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
}

impl DataTypeFunctions for InterfaceTypeDefinition {
    fn mark(&self) {
        self.fields_definition.mark();
        self.interface_implementations.mark();
    }
}

impl HasDefinitionWrapper for InterfaceTypeDefinition {
    fn wrapping_class() -> RClass {
        *memoize!(RClass: root().define_class("InterfaceType", Default::default()).unwrap())
    }
}

impl bluejay_core::definition::InterfaceTypeDefinition for InterfaceTypeDefinition {
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
    let class = root().define_class("InterfaceTypeDefinition", Default::default())?;

    class.define_singleton_method("new", function!(InterfaceTypeDefinition::new, 1))?;

    Ok(())
}
