use super::{input_value_definition::InputValueDefinition};
use crate::helpers::{WrappedStruct, from_rarray, WrappedStructMap};
use magnus::{RArray, Error, gc};

#[derive(Clone, Debug)]
pub struct InputFieldsDefinition {
    input_field_definitions: Vec<WrappedStruct<InputValueDefinition>>,
    rarray: RArray,
}

impl InputFieldsDefinition {
    pub fn new(rarray: RArray) -> Result<Self, Error> {
        rarray.freeze();
        from_rarray(rarray).map(|input_field_definitions| Self { input_field_definitions, rarray })
    }

    pub(crate) fn mark(&self) {
        gc::mark(self.rarray);
        self.input_field_definitions.iter().for_each(WrappedStruct::mark)
    }
}

impl AsRef<[WrappedStruct<InputValueDefinition>]> for InputFieldsDefinition {
    fn as_ref(&self) -> &[WrappedStruct<InputValueDefinition>] {
        &self.input_field_definitions
    }
}

impl AsRef<RArray> for InputFieldsDefinition {
    fn as_ref(&self) -> &RArray {
        &self.rarray
    }
}

impl bluejay_core::AsIter for InputFieldsDefinition {
    type Item = InputValueDefinition;
    type Iterator<'a> = WrappedStructMap<'a, Self::Item>;

    fn iter<'a>(&'a self) -> Self::Iterator<'a> {
        self.input_field_definitions.iter().map(|ws| ws.get())
    }
}

impl bluejay_core::definition::InputFieldsDefinition for InputFieldsDefinition {
    type InputValueDefinition = InputValueDefinition;
}
