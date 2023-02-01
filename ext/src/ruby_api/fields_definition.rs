use super::field_definition::FieldDefinition;
use crate::helpers::{TypedFrozenRArray, WrappedStruct};

pub type FieldsDefinition = TypedFrozenRArray<WrappedStruct<FieldDefinition>>;

impl bluejay_core::definition::FieldsDefinition for FieldsDefinition {
    type FieldDefinition = FieldDefinition;
}
