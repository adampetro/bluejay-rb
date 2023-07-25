use crate::helpers::rhash_with_capacity;

use super::root;
use magnus::{
    function, method,
    rb_sys::AsRawValue,
    scan_args::scan_args,
    typed_data::{self, Obj},
    Error, Module, Object, Value, RHash,
};
use std::borrow::Cow;

#[derive(Clone, Debug, PartialEq, Eq)]
#[magnus::wrap(class = "Bluejay::ExecutionError")]
pub struct ExecutionError {
    message: Cow<'static, str>,
    path: Option<Vec<String>>,
}

impl ExecutionError {
    pub fn new(message: impl Into<Cow<'static, str>>, path: Option<Vec<String>>) -> Self {
        Self {
            message: message.into(),
            path,
        }
    }

    fn rb_new(args: &[Value]) -> Result<Self, Error> {
        let args = scan_args::<(String,), (Option<Vec<String>>,), (), (), (), ()>(args)?;
        let (message,) = args.required;
        let (path,) = args.optional;
        Ok(Self::new(message, path))
    }

    pub fn message(&self) -> &str {
        self.message.as_ref()
    }

    pub fn path(&self) -> Option<Vec<String>> {
        // TODO: avoid clone here
        self.path.clone()
    }

    pub fn to_h(&self) -> RHash {
        let ruby_h = rhash_with_capacity(2);
        let _ = ruby_h.aset("message", self.message());
        _ = ruby_h.aset("path", self.path());
        ruby_h
    }

    fn inspect(rb_self: Obj<Self>) -> Result<String, Error> {
        let rs_self = rb_self.get();

        Ok(format!(
            "#<Bluejay::ExecutionError:0x{:016x} @message={:?} @path={:?}>",
            rb_self.as_raw(),
            rs_self.message,
            rs_self.path,
        ))
    }
}

pub fn init() -> Result<(), Error> {
    let class = root().define_class("ExecutionError", Default::default())?;

    class.define_singleton_method("new", function!(ExecutionError::rb_new, -1))?;
    class.define_method("message", method!(ExecutionError::message, 0))?;
    class.define_method("path", method!(ExecutionError::path, 0))?;
    class.define_method(
        "==",
        method!(<ExecutionError as typed_data::IsEql>::is_eql, 1),
    )?;
    class.define_method("inspect", method!(ExecutionError::inspect, 0))?;
    class.define_method("to_h", method!(ExecutionError::to_h, 0))?;

    Ok(())
}
