use super::input_value_definition::InputValueDefinition;
use crate::helpers::TypedFrozenRArray;
use magnus::typed_data::Obj;

pub type InputFieldsDefinition = TypedFrozenRArray<Obj<InputValueDefinition>>;

impl bluejay_core::definition::InputFieldsDefinition for InputFieldsDefinition {
    type InputValueDefinition = InputValueDefinition;
}
