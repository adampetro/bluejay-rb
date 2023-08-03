use crate::helpers::{rhash_with_capacity, TypedFrozenRArray};

use super::root;
use magnus::{
    function, gc, method,
    rb_sys::AsRawValue,
    scan_args::scan_args,
    typed_data::{self, Obj},
    DataTypeFunctions, Error, Integer, Module, Object, RArray, RHash, TypedData, Value,
};
use std::borrow::Cow;

#[derive(Clone, Debug, PartialEq, Eq)]
#[magnus::wrap(class = "Bluejay::ExecutionError::ErrorLocation")]
pub struct ErrorLocation {
    line: usize,
    column: usize,
}

impl ErrorLocation {
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }

    fn rb_new(line_v: Integer, col_v: Integer) -> Result<Self, Error> {
        let line = line_v.try_convert::<usize>().unwrap();
        let column = col_v.try_convert::<usize>().unwrap();
        Ok(Self::new(line, column))
    }

    fn to_h(&self) -> Result<RHash, Error> {
        let ruby_h = rhash_with_capacity(2);
        ruby_h.aset("line", self.line)?;
        ruby_h.aset("column", self.column)?;
        Ok(ruby_h)
    }

    fn inspect(rb_self: Obj<Self>) -> Result<String, Error> {
        let rs_self = rb_self.get();

        Ok(format!(
            "#<Bluejay::ExecutionError::ErrorLocation:0x{:016x} @line={:?} @column={:?}>",
            rb_self.as_raw(),
            rs_self.line,
            rs_self.column,
        ))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, TypedData)]
#[magnus(class = "Bluejay::ExecutionError", mark)]
pub struct ExecutionError {
    message: Cow<'static, str>,
    path: Option<Vec<String>>,
    locations: Option<TypedFrozenRArray<Obj<ErrorLocation>>>,
}

impl DataTypeFunctions for ExecutionError {
    fn mark(&self) {
        if let Some(locations) = self.locations {
            gc::mark(locations);
        }
    }
}

impl ExecutionError {
    pub fn new(
        message: impl Into<Cow<'static, str>>,
        path: Option<Vec<String>>,
        locations: Option<TypedFrozenRArray<Obj<ErrorLocation>>>,
    ) -> Self {
        Self {
            message: message.into(),
            path,
            locations,
        }
    }

    fn rb_new(args: &[Value]) -> Result<Self, Error> {
        let args = scan_args::<
            (String,),
            (
                Option<Vec<String>>,
                Option<TypedFrozenRArray<Obj<ErrorLocation>>>,
            ),
            (),
            (),
            (),
            (),
        >(args)?;
        let (message,) = args.required;
        let (path, locations) = args.optional;
        Ok(Self::new(message, path, locations))
    }

    pub fn message(&self) -> &str {
        self.message.as_ref()
    }

    pub fn path(&self) -> Option<Vec<String>> {
        // TODO: avoid clone here
        self.path.clone()
    }

    fn to_h(&self) -> Result<RHash, Error> {
        let ruby_h = rhash_with_capacity(2);
        ruby_h.aset("path", self.path())?;
        ruby_h.aset("message", self.message())?;
        if let Some(locations) = &self.locations {
            let location_hashes = locations
                .iter()
                .map(|loc| loc.to_h())
                .collect::<Result<RArray, Error>>()?;
            ruby_h.aset("locations", location_hashes)?;
        }
        Ok(ruby_h)
    }

    fn inspect(rb_self: Obj<Self>) -> Result<String, Error> {
        let rs_self = rb_self.get();

        Ok(format!(
            "#<Bluejay::ExecutionError:0x{:016x} @message={:?} @path={:?} @locations={:?}>",
            rb_self.as_raw(),
            rs_self.message,
            rs_self.path,
            rs_self.locations,
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

    let loc_class = class.define_class("ErrorLocation", Default::default())?;
    loc_class.define_singleton_method("new", function!(ErrorLocation::rb_new, 2))?;
    loc_class.define_method("inspect", method!(ErrorLocation::inspect, 0))?;
    loc_class.define_method(
        "==",
        method!(<ErrorLocation as typed_data::IsEql>::is_eql, 1),
    )?;

    Ok(())
}
