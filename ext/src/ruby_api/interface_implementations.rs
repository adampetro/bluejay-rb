use super::interface_implementation::InterfaceImplementation;
use crate::helpers::TypedFrozenRArray;
use magnus::typed_data::Obj;

pub type InterfaceImplementations = TypedFrozenRArray<Obj<InterfaceImplementation>>;

impl bluejay_core::definition::InterfaceImplementations for InterfaceImplementations {
    type InterfaceImplementation = InterfaceImplementation;
}
