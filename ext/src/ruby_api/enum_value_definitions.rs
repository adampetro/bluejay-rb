use super::{enum_value_definition::EnumValueDefinition};
use crate::helpers::ObjVec;

pub type EnumValueDefinitions = ObjVec<EnumValueDefinition>;

impl bluejay_core::definition::EnumValueDefinitions for EnumValueDefinitions {
    type EnumValueDefinition = EnumValueDefinition;
}
