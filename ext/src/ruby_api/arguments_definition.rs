use super::input_value_definition::InputValueDefinition;
use crate::helpers::{TypedFrozenRArray, WrappedStruct};

pub type ArgumentsDefinition = TypedFrozenRArray<WrappedStruct<InputValueDefinition>>;

impl bluejay_core::definition::ArgumentsDefinition for ArgumentsDefinition {
    type ArgumentDefinition = InputValueDefinition;
}
