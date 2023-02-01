use super::root;
use crate::helpers::WrappedStruct;
use magnus::{
    exception, function, gc, method, rb_sys::AsRawValue, DataTypeFunctions, Error, Module, Object,
    TypedData, Value,
};

#[derive(Clone, Debug, TypedData)]
#[magnus(class = "Bluejay::Result", mark)]
pub struct RResult(Result<Value, Value>);

impl RResult {
    fn is_ok(&self) -> bool {
        self.0.is_ok()
    }

    fn is_err(&self) -> bool {
        self.0.is_err()
    }

    fn unwrap_err(&self) -> Result<Value, Error> {
        match self.0 {
            Ok(_) => Err(Error::new(
                exception::runtime_error(),
                "Ok variant does not have an error value".to_owned(),
            )),
            Err(err) => Ok(err),
        }
    }

    fn unwrap(&self) -> Result<Value, Error> {
        match self.0 {
            Ok(ok) => Ok(ok),
            Err(_) => Err(Error::new(
                exception::runtime_error(),
                "Error variant does not have an ok value".to_owned(),
            )),
        }
    }

    fn inspect(rb_self: WrappedStruct<Self>) -> Result<String, Error> {
        let rs_self = rb_self.get();

        Ok(format!(
            "#<Bluejay::Result:0x{:016x} @{}={:?}>",
            rb_self.to_value().as_raw(),
            match rs_self.0 {
                Ok(_) => "ok_value",
                Err(_) => "error_value",
            },
            match rs_self.0 {
                Ok(val) => val,
                Err(val) => val,
            },
        ))
    }
}

impl DataTypeFunctions for RResult {
    fn mark(&self) {
        let value = match &self.0 {
            Ok(ok) => ok,
            Err(err) => err,
        };
        gc::mark(value)
    }
}

impl<T: Into<Value>, E: Into<Value>> From<Result<T, E>> for RResult {
    fn from(r: Result<T, E>) -> Self {
        Self(r.map(Into::into).map_err(Into::into))
    }
}

pub fn init() -> Result<(), Error> {
    let class = root().define_class("Result", Default::default())?;

    // class.define_singleton_method("new", function!(RResult::new, 1))?;
    class.define_singleton_method("[]", function!(|_: Value, _: Value| RResult::class(), 2))?;
    class.define_method("ok?", method!(RResult::is_ok, 0))?;
    class.define_method("err?", method!(RResult::is_err, 0))?;
    class.define_method("unwrap", method!(RResult::unwrap, 0))?;
    class.define_method("unwrap_err", method!(RResult::unwrap_err, 0))?;
    class.define_method("inspect", method!(RResult::inspect, 0))?;

    Ok(())
}
