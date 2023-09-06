use crate::ruby_api::{root, ExecutionError};
use magnus::{
    function, method,
    rb_sys::AsRawValue,
    typed_data::{self, Obj},
    Error, Module, Object, RArray,
};

#[derive(Clone, Debug, PartialEq, Eq)]
#[magnus::wrap(class = "Bluejay::CoercionError", mark)]
pub struct CoercionError {
    message: String,
    path: Vec<String>,
}

impl CoercionError {
    pub fn new(message: String, path: Vec<String>) -> Self {
        Self { message, path }
    }

    pub fn message(&self) -> &str {
        self.message.as_str()
    }

    pub fn path(&self) -> RArray {
        RArray::from_iter(self.path.iter().map(|s| s.as_str()))
    }

    fn inspect(rb_self: Obj<Self>) -> Result<String, Error> {
        let rs_self = rb_self.get();

        Ok(format!(
            "#<Bluejay::CoercionError:0x{:016x} @message={:?} @path={:?}>",
            rb_self.as_raw(),
            rs_self.message,
            rs_self.path,
        ))
    }
}

impl From<CoercionError> for ExecutionError {
    fn from(val: CoercionError) -> Self {
        ExecutionError::new(val.message, None, None)
    }
}

pub fn init() -> Result<(), Error> {
    let class = root().define_class("CoercionError", Default::default())?;

    class.define_singleton_method("new", function!(CoercionError::new, 2))?;
    class.define_method("message", method!(CoercionError::message, 0))?;
    class.define_method("path", method!(CoercionError::path, 0))?;
    class.define_method(
        "==",
        method!(<CoercionError as typed_data::IsEql>::is_eql, 1),
    )?;
    class.define_method("inspect", method!(CoercionError::inspect, 0))?;

    Ok(())
}
