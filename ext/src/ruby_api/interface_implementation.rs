use super::{root, interface_type_definition::InterfaceTypeDefinition};
use magnus::{RClass, function, Error, Module, Object, TypedData, DataTypeFunctions, method};
use crate::helpers::{WrappedDefinition, WrappedStruct};

#[derive(Clone, Debug, TypedData)]
#[magnus(class = "Bluejay::InterfaceImplementation", mark)]
pub struct InterfaceImplementation {
    interface: WrappedDefinition<InterfaceTypeDefinition>,
}

impl InterfaceImplementation {
    pub fn new(interface: RClass) -> Result<Self, Error> {
        WrappedDefinition::new(interface).map(|interface| Self { interface })
    }

    pub fn interface(&self) -> WrappedStruct<InterfaceTypeDefinition> {
        *self.interface.get()
    }
}

impl DataTypeFunctions for InterfaceImplementation {
    fn mark(&self) {
        self.interface.mark();
    }
}

impl bluejay_core::definition::InterfaceImplementation for InterfaceImplementation {
    type InterfaceTypeDefinition = InterfaceTypeDefinition;

    fn interface(&self) -> &Self::InterfaceTypeDefinition {
        self.interface.as_ref()
    }
}

pub fn init() -> Result<(), Error> {
    let class = root().define_class("InterfaceImplementation", Default::default())?;

    class.define_singleton_method("new", function!(InterfaceImplementation::new, 1))?;
    class.define_method("interface", method!(|ii: &InterfaceImplementation| ii.interface.class(), 0))?;

    Ok(())
}
