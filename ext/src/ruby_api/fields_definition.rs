use super::field_definition::FieldDefinition;
use crate::helpers::TypedFrozenRArray;
use magnus::typed_data::Obj;

pub type FieldsDefinition = TypedFrozenRArray<Obj<FieldDefinition>>;

impl bluejay_core::definition::FieldsDefinition for FieldsDefinition {
    type FieldDefinition = FieldDefinition;
}
