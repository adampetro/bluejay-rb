use super::{input_value_definition::InputValueDefinition};
use crate::helpers::ObjVec;

pub type ArgumentsDefinition = ObjVec<InputValueDefinition>;

impl bluejay_core::definition::ArgumentsDefinition for ArgumentsDefinition {
    type ArgumentDefinition = InputValueDefinition;
}
