use super::{interface_implementation::InterfaceImplementation};
use crate::helpers::{WrappedStruct, TypedFrozenRArray};

pub type InterfaceImplementations = TypedFrozenRArray<WrappedStruct<InterfaceImplementation>>;

impl bluejay_core::definition::InterfaceImplementations for InterfaceImplementations {
    type InterfaceImplementation = InterfaceImplementation;
}
