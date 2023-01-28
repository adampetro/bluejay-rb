use super::{enum_value_definition::EnumValueDefinition};
use crate::helpers::{WrappedStruct, TypedFrozenRArray};

pub type EnumValueDefinitions = TypedFrozenRArray<WrappedStruct<EnumValueDefinition>>;

impl bluejay_core::definition::EnumValueDefinitions for EnumValueDefinitions {
    type EnumValueDefinition = EnumValueDefinition;
}
