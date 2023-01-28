use super::{input_value_definition::InputValueDefinition};
use crate::helpers::ObjVec;

pub type InputFieldsDefinition = ObjVec<InputValueDefinition>;

impl bluejay_core::definition::InputFieldsDefinition for InputFieldsDefinition {
    type InputValueDefinition = InputValueDefinition;
}
