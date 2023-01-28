use super::{field_definition::FieldDefinition};
use crate::helpers::{WrappedStruct, TypedFrozenRArray};

pub type FieldsDefinition = TypedFrozenRArray<WrappedStruct<FieldDefinition>>;

impl bluejay_core::definition::FieldsDefinition for FieldsDefinition {
    type FieldDefinition = FieldDefinition;
}
