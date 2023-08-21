use super::root;
use super::ExecutionError;
use crate::helpers::TypedFrozenRArray;
use magnus::{
    gc, method, typed_data::Obj, DataTypeFunctions, Error, Module, RArray, TypedData, Value,
};

#[derive(Clone, Debug, TypedData)]
#[magnus(class = "Bluejay::ExecutionResult", mark)]
pub struct ExecutionResult {
    value: Value,
    errors: TypedFrozenRArray<Obj<ExecutionError>>,
}

impl ExecutionResult {
    pub fn new(value: Value, errors: impl Iterator<Item = ExecutionError>) -> Self {
        let errors = TypedFrozenRArray::from_iter(errors.into_iter().map(Into::into));
        Self { value, errors }
    }

    fn value(&self) -> Value {
        self.value
    }

    fn errors(&self) -> RArray {
        self.errors.into()
    }
}

impl DataTypeFunctions for ExecutionResult {
    fn mark(&self) {
        gc::mark(&self.value);
        gc::mark(self.errors);
    }
}

pub fn init() -> Result<(), Error> {
    let class = root().define_class("ExecutionResult", Default::default())?;
    class.define_method("value", method!(ExecutionResult::value, 0))?;
    class.define_method("errors", method!(ExecutionResult::errors, 0))?;

    Ok(())
}
