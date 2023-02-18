use super::input_value_definition::InputValueDefinition;
use crate::helpers::TypedFrozenRArray;
use magnus::typed_data::Obj;

pub type ArgumentsDefinition = TypedFrozenRArray<Obj<InputValueDefinition>>;

impl bluejay_core::definition::ArgumentsDefinition for ArgumentsDefinition {
    type ArgumentDefinition = InputValueDefinition;
}
