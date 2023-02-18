use super::enum_value_definition::EnumValueDefinition;
use crate::helpers::TypedFrozenRArray;
use magnus::typed_data::Obj;

pub type EnumValueDefinitions = TypedFrozenRArray<Obj<EnumValueDefinition>>;

impl bluejay_core::definition::EnumValueDefinitions for EnumValueDefinitions {
    type EnumValueDefinition = EnumValueDefinition;
}
