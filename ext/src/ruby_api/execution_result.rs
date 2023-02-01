use super::root;
use super::ExecutionError;
use crate::helpers::WrappedStruct;
use magnus::{gc, method, DataTypeFunctions, Error, Module, RArray, TypedData, Value};

#[derive(Clone, Debug, TypedData)]
#[magnus(class = "Bluejay::ExecutionResult", mark)]
pub struct ExecutionResult {
    value: Value,
    errors: Vec<WrappedStruct<ExecutionError>>,
}

impl ExecutionResult {
    pub fn new(value: Value, errors: Vec<WrappedStruct<ExecutionError>>) -> Self {
        Self { value, errors }
    }

    fn value(&self) -> Value {
        self.value
    }

    fn errors(&self) -> RArray {
        RArray::from_iter(self.errors.iter().copied())
    }
}

impl DataTypeFunctions for ExecutionResult {
    fn mark(&self) {
        gc::mark(&self.value);
        self.errors.iter().for_each(WrappedStruct::mark);
    }
}

pub fn init() -> Result<(), Error> {
    let class = root().define_class("ExecutionResult", Default::default())?;
    class.define_method("value", method!(ExecutionResult::value, 0))?;
    class.define_method("errors", method!(ExecutionResult::errors, 0))?;

    Ok(())
}
