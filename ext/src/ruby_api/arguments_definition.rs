use super::{input_value_definition::InputValueDefinition};
use crate::helpers::{WrappedStruct, TypedFrozenRArray};

pub type ArgumentsDefinition = TypedFrozenRArray<WrappedStruct<InputValueDefinition>>;

impl bluejay_core::definition::ArgumentsDefinition for ArgumentsDefinition {
    type ArgumentDefinition = InputValueDefinition;
}
