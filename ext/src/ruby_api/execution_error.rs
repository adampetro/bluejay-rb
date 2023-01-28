use super::root;
use crate::helpers::WrappedStruct;
use magnus::{function, Error, Module, Object, Value, method, rb_sys::AsRawValue};

#[derive(Clone, Debug, PartialEq)]
#[magnus::wrap(class = "Bluejay::ExecutionError", mark)]
pub struct ExecutionError {
    message: String,
}

impl ExecutionError {
    pub fn new(message: String) -> Self {
        Self { message }
    }

    pub fn message(&self) -> &str {
        self.message.as_str()
    }

    pub fn eql(&self, other: Value) -> bool {
        if let Ok(other) = other.try_convert::<&Self>() {
            self == other
        } else {
            false
        }
    }

    fn inspect(rb_self: WrappedStruct<Self>) -> Result<String, Error> {
        let rs_self = rb_self.get();

        Ok(format!(
            "#<Bluejay::ExecutionError:0x{:016x} @message={:?}>",
            rb_self.to_value().as_raw(),
            rs_self.message,
        ))
    }
}

pub fn init() -> Result<(), Error> {
    let class = root().define_class("ExecutionError", Default::default())?;

    class.define_singleton_method("new", function!(ExecutionError::new, 1))?;
    class.define_method("message", method!(ExecutionError::message, 0))?;
    class.define_method("==", method!(ExecutionError::eql, 1))?;
    class.define_method("inspect", method!(ExecutionError::inspect, 0))?;

    Ok(())
}
