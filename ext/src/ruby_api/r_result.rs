use super::root;
use crate::helpers::WrappedStruct;
use magnus::{
    exception, function, gc, method, rb_sys::AsRawValue, DataTypeFunctions, Error, Module, Object,
    TryConvert, TypedData, Value,
};

#[derive(Clone, Debug, TypedData)]
#[magnus(class = "Bluejay::Result", mark)]
#[repr(transparent)]
pub struct RResult(Result<Value, Value>);

impl RResult {
    fn ok(value: Value) -> Self {
        Self(Ok(value))
    }

    fn err(value: Value) -> Self {
        Self(Err(value))
    }

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

    pub fn eql(&self, other: Value) -> Result<bool, Error> {
        if let Ok(other) = other.try_convert::<&Self>() {
            match (&self.0, &other.0) {
                (Ok(self_ok), Ok(other_ok)) => self_ok.eql(other_ok),
                (Err(self_err), Err(other_err)) => self_err.eql(other_err),
                _ => Ok(false),
            }
        } else {
            Ok(false)
        }
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

impl<T: TryConvert, E: TryConvert> TryFrom<&RResult> for Result<T, E> {
    type Error = Error;

    fn try_from(value: &RResult) -> Result<Self, Self::Error> {
        match value.0 {
            Ok(ok) => {
                let ok: T = ok.try_convert()?;
                Ok(Ok(ok))
            }
            Err(err) => {
                let err: E = err.try_convert()?;
                Ok(Err(err))
            }
        }
    }
}

pub fn init() -> Result<(), Error> {
    let class = root().define_class("Result", Default::default())?;

    class.define_singleton_method("[]", function!(|_: Value, _: Value| RResult::class(), 2))?;
    class.define_singleton_method("ok", function!(RResult::ok, 1))?;
    class.define_singleton_method("err", function!(RResult::err, 1))?;
    class.define_method("ok?", method!(RResult::is_ok, 0))?;
    class.define_method("err?", method!(RResult::is_err, 0))?;
    class.define_method("unwrap", method!(RResult::unwrap, 0))?;
    class.define_method("unwrap_err", method!(RResult::unwrap_err, 0))?;
    class.define_method("inspect", method!(RResult::inspect, 0))?;
    class.define_method("==", method!(RResult::eql, 1))?;

    Ok(())
}
