use super::{field_definition::FieldDefinition};
use crate::helpers::{WrappedStruct, from_rarray, WrappedStructMap};
use magnus::{RArray, Error};

#[derive(Clone, Debug)]
pub struct FieldsDefinition(Vec<WrappedStruct<FieldDefinition>>);

impl FieldsDefinition {
    pub fn new(field_definitions: Option<RArray>) -> Result<Self, Error> {
        match field_definitions {
            Some(field_definitions) => {
                field_definitions.push(FieldDefinition::typename()).unwrap();
                from_rarray(field_definitions).map(Self)
            },
            None => Ok(Self(Default::default()))
        }
    }

    pub(crate) fn mark(&self) {
        self.0.iter().for_each(WrappedStruct::mark)
    }
}

impl AsRef<[WrappedStruct<FieldDefinition>]> for FieldsDefinition {
    fn as_ref(&self) -> &[WrappedStruct<FieldDefinition>] {
        &self.0
    }
}

impl bluejay_core::AsIter for FieldsDefinition {
    type Item = FieldDefinition;
    type Iterator<'a> = WrappedStructMap<'a, Self::Item>;

    fn iter<'a>(&'a self) -> Self::Iterator<'a> {
        self.0.iter().map(|ws| ws.get())
    }
}

impl bluejay_core::definition::FieldsDefinition for FieldsDefinition {
    type FieldDefinition = FieldDefinition;
}
