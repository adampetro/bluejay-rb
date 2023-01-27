use super::{enum_value_definition::EnumValueDefinition};
use crate::helpers::{WrappedStruct, from_rarray, WrappedStructMap};
use magnus::{RArray, Error};

#[derive(Clone, Debug)]
pub struct EnumValueDefinitions(Vec<WrappedStruct<EnumValueDefinition>>);

impl EnumValueDefinitions {
    pub fn new(argument_definitions: RArray) -> Result<Self, Error> {
        from_rarray(argument_definitions).map(Self)
    }

    pub(crate) fn mark(&self) {
        self.0.iter().for_each(WrappedStruct::mark)
    }

    pub(crate) fn empty() -> Self {
        Self(vec![])
    }
}

impl AsRef<[WrappedStruct<EnumValueDefinition>]> for EnumValueDefinitions {
    fn as_ref(&self) -> &[WrappedStruct<EnumValueDefinition>] {
        &self.0
    }
}

impl bluejay_core::AsIter for EnumValueDefinitions {
    type Item = EnumValueDefinition;
    type Iterator<'a> = WrappedStructMap<'a, Self::Item>;

    fn iter<'a>(&'a self) -> Self::Iterator<'a> {
        self.0.iter().map(|ws| ws.get())
    }
}

impl bluejay_core::definition::EnumValueDefinitions for EnumValueDefinitions {
    type EnumValueDefinition = EnumValueDefinition;
}
