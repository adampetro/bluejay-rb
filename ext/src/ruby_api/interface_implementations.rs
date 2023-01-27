use super::{interface_implementation::InterfaceImplementation, interface_type_definition::InterfaceTypeDefinition};
use crate::helpers::{WrappedStruct, from_rarray, WrappedStructMap};
use magnus::{RArray, Error};

#[derive(Clone, Debug)]
pub struct InterfaceImplementations(Vec<WrappedStruct<InterfaceImplementation>>);

impl InterfaceImplementations {
    pub fn new(interface_implementations: RArray) -> Result<Self, Error> {
        from_rarray(interface_implementations).map(Self)
    }

    pub(crate) fn mark(&self) {
        self.0.iter().for_each(WrappedStruct::mark)
    }
}

impl AsRef<[WrappedStruct<InterfaceImplementation>]> for InterfaceImplementations {
    fn as_ref(&self) -> &[WrappedStruct<InterfaceImplementation>] {
        &self.0
    }
}

impl bluejay_core::AsIter for InterfaceImplementations {
    type Item = InterfaceImplementation;
    type Iterator<'a> = WrappedStructMap<'a, Self::Item>;

    fn iter<'a>(&'a self) -> Self::Iterator<'a> {
        self.0.iter().map(|ws| ws.get())
    }
}

impl bluejay_core::definition::InterfaceImplementations for InterfaceImplementations {
    type InterfaceImplementation = InterfaceImplementation;
}
