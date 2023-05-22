use super::root;
use magnus::{
    exception, function, gc, method, DataTypeFunctions, Error, IntoValue, Module, Object,
    TryConvert, TypedData, Value,
};

#[derive(Clone, Debug, TypedData)]
#[magnus(class = "Bluejay::Result", mark)]
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
                "Ok variant does not have an error value",
            )),
            Err(err) => Ok(err),
        }
    }

    fn unwrap(&self) -> Result<Value, Error> {
        match self.0 {
            Ok(ok) => Ok(ok),
            Err(_) => Err(Error::new(
                exception::runtime_error(),
                "Error variant does not have an ok value",
            )),
        }
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
        match &self.0 {
            Ok(ok) => gc::mark(ok),
            Err(err) => gc::mark(err),
        }
    }
}

impl<T: IntoValue, E: IntoValue> From<Result<T, E>> for RResult {
    fn from(r: Result<T, E>) -> Self {
        Self(r.map(IntoValue::into_value).map_err(IntoValue::into_value))
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
    class.define_method("==", method!(RResult::eql, 1))?;

    Ok(())
}
