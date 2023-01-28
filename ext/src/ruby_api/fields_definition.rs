use super::{field_definition::FieldDefinition};
use crate::helpers::ObjVec;

pub type FieldsDefinition = ObjVec<FieldDefinition>;

impl bluejay_core::definition::FieldsDefinition for FieldsDefinition {
    type FieldDefinition = FieldDefinition;
}
