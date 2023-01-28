use super::{interface_implementation::InterfaceImplementation};
use crate::helpers::ObjVec;

pub type InterfaceImplementations = ObjVec<InterfaceImplementation>;

impl bluejay_core::definition::InterfaceImplementations for InterfaceImplementations {
    type InterfaceImplementation = InterfaceImplementation;
}
