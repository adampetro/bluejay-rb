use super::{input_value_definition::InputValueDefinition};
use crate::helpers::{WrappedStruct, TypedFrozenRArray};

pub type InputFieldsDefinition = TypedFrozenRArray<WrappedStruct<InputValueDefinition>>;

impl bluejay_core::definition::InputFieldsDefinition for InputFieldsDefinition {
    type InputValueDefinition = InputValueDefinition;
}
