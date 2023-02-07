use std::ops::Deref;

use crate::helpers::WrappedDefinition;
use crate::ruby_api::{
    value::{mark_inner, Value, ValueInner},
    InputObjectTypeDefinition,
};
use bluejay_core::AsIter;
use bluejay_core::ObjectValue;
use magnus::{gc, Error, RObject, TryConvert, Value as RValue};

#[derive(Debug)]
pub struct InputObject {
    obj: RObject,
    definition: WrappedDefinition<InputObjectTypeDefinition>,
    fields: Vec<(String, ValueInner)>,
}

impl TryConvert for InputObject {
    fn try_convert(val: RValue) -> Result<Self, Error> {
        let definition: WrappedDefinition<InputObjectTypeDefinition> =
            WrappedDefinition::try_convert(*val.class())?;
        let obj: RObject = val.try_convert()?;
        obj.freeze();
        let fields_result: Result<Vec<(String, ValueInner)>, Error> = definition
            .as_ref()
            .input_fields_definition()
            .iter()
            .map(|ivd| -> Result<(String, ValueInner), Error> {
                let value: RValue = obj.funcall(ivd.name(), ())?;
                let value: Value = value.try_convert()?;
                let name = ivd.name().to_string();
                Ok((name, value.into()))
            })
            .collect();
        let fields = fields_result?;
        Ok(Self {
            obj,
            definition,
            fields,
        })
    }
}

impl ObjectValue<ValueInner> for InputObject {
    type Key = String;

    fn fields(&self) -> &[(Self::Key, ValueInner)] {
        &self.fields
    }
}

impl InputObject {
    pub(crate) fn mark(&self) {
        gc::mark(self.obj);
        self.definition.mark();
        self.fields.iter().for_each(|(_, v)| mark_inner(v));
    }
}

impl Deref for InputObject {
    type Target = RValue;

    fn deref(&self) -> &Self::Target {
        &self.obj
    }
}
