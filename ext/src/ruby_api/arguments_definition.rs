use super::{input_value_definition::InputValueDefinition};
use crate::helpers::{WrappedStruct, from_rarray, WrappedStructMap};
use magnus::{RArray, Error};

#[derive(Clone, Debug)]
pub struct ArgumentsDefinition(Vec<WrappedStruct<InputValueDefinition>>);

impl ArgumentsDefinition {
    pub fn new(argument_definitions: Option<RArray>) -> Result<Self, Error> {
        match argument_definitions {
            Some(argument_definitions) => from_rarray(argument_definitions).map(Self),
            None => Ok(Self(Default::default()))
        }
    }

    pub(crate) fn mark(&self) {
        self.0.iter().for_each(WrappedStruct::mark)
    }

    pub(crate) fn empty() -> Self {
        Self(vec![])
    }
}

impl AsRef<[WrappedStruct<InputValueDefinition>]> for ArgumentsDefinition {
    fn as_ref(&self) -> &[WrappedStruct<InputValueDefinition>] {
        &self.0
    }
}

impl bluejay_core::AsIter for ArgumentsDefinition {
    type Item = InputValueDefinition;
    type Iterator<'a> = WrappedStructMap<'a, Self::Item>;

    fn iter<'a>(&'a self) -> Self::Iterator<'a> {
        self.0.iter().map(|ws| ws.get())
    }
}

impl bluejay_core::definition::ArgumentsDefinition for ArgumentsDefinition {
    type ArgumentDefinition = InputValueDefinition;
}
