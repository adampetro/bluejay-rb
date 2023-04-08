use super::root;
use magnus::{
    function, method,
    rb_sys::AsRawValue,
    typed_data::{self, Obj},
    Error, Module, Object,
};
use std::borrow::Cow;

#[derive(Clone, Debug, PartialEq, Eq)]
#[magnus::wrap(class = "Bluejay::ExecutionError")]
pub struct ExecutionError {
    message: Cow<'static, str>,
}

impl ExecutionError {
    pub fn new(message: impl Into<Cow<'static, str>>) -> Self {
        Self {
            message: message.into(),
        }
    }

    pub fn message(&self) -> &str {
        self.message.as_ref()
    }

    fn inspect(rb_self: Obj<Self>) -> Result<String, Error> {
        let rs_self = rb_self.get();

        Ok(format!(
            "#<Bluejay::ExecutionError:0x{:016x} @message={:?}>",
            rb_self.as_raw(),
            rs_self.message,
        ))
    }
}

impl From<String> for ExecutionError {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

pub fn init() -> Result<(), Error> {
    let class = root().define_class("ExecutionError", Default::default())?;

    class.define_singleton_method("new", function!(<ExecutionError as From<String>>::from, 1))?;
    class.define_method("message", method!(ExecutionError::message, 0))?;
    class.define_method(
        "==",
        method!(<ExecutionError as typed_data::IsEql>::is_eql, 1),
    )?;
    class.define_method("inspect", method!(ExecutionError::inspect, 0))?;

    Ok(())
}
