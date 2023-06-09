use super::root;
use super::ExecutionError;
use crate::helpers::TypedFrozenRArray;
use magnus::{
    gc, method, typed_data::Obj, DataTypeFunctions, Error, Module, RArray, TypedData, Value,
};
use std::time::Duration;

#[derive(Clone, Debug, TypedData)]
#[magnus(class = "Bluejay::ExecutionResult", mark)]
pub struct ExecutionResult {
    value: Value,
    errors: TypedFrozenRArray<Obj<ExecutionError>>,
    ruby_duration: Duration,
}

impl ExecutionResult {
    pub fn new(
        value: Value,
        errors: impl IntoIterator<Item = impl Into<ExecutionError>>,
        ruby_duration: Duration,
    ) -> Self {
        let errors = TypedFrozenRArray::from_iter(errors.into_iter().map(Into::into));
        Self {
            value,
            errors,
            ruby_duration,
        }
    }

    fn value(&self) -> Value {
        self.value
    }

    fn errors(&self) -> RArray {
        self.errors.into()
    }

    fn ruby_duration_us(&self) -> u64 {
        self.ruby_duration
            .as_micros()
            .try_into()
            .expect("Error converting u128 to u64")
    }

    fn ruby_duration_ns(&self) -> u64 {
        self.ruby_duration
            .as_nanos()
            .try_into()
            .expect("Error converting u128 to u64")
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
    class.define_method(
        "ruby_duration_us",
        method!(ExecutionResult::ruby_duration_us, 0),
    )?;
    class.define_method(
        "ruby_duration_ns",
        method!(ExecutionResult::ruby_duration_ns, 0),
    )?;

    Ok(())
}
