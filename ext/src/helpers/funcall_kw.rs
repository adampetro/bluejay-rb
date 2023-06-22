use magnus::{
    rb_sys::{protect, AsRawId, AsRawValue, FromRawValue},
    value::IntoId,
    Class, Error, RClass, RHash, TryConvert, Value,
};
use rb_sys::{rb_class_new_instance_kw, rb_funcallv_kw, RB_PASS_KEYWORDS, VALUE};
use std::os::raw::c_int;

pub trait FuncallKw {
    fn funcall_kw<M: IntoId, T: TryConvert>(self, method: M, kwargs: RHash) -> Result<T, Error>;
}

impl FuncallKw for Value {
    fn funcall_kw<M: IntoId, T: TryConvert>(self, method: M, kwargs: RHash) -> Result<T, Error> {
        let args = [kwargs];
        let slice = args.as_slice();
        unsafe {
            let id = method.into_id_unchecked();
            protect(|| {
                rb_funcallv_kw(
                    self.as_raw(),
                    id.as_raw(),
                    slice.len() as c_int,
                    slice.as_ptr() as *const VALUE,
                    RB_PASS_KEYWORDS as c_int,
                )
            })
            .and_then(|v| Value::from_raw(v).try_convert())
        }
    }
}

pub trait NewInstanceKw {
    type Instance;

    fn new_instance_kw(self, kwargs: RHash) -> Result<Self::Instance, Error>;
}

impl NewInstanceKw for RClass {
    type Instance = <Self as Class>::Instance;

    fn new_instance_kw(self, kwargs: RHash) -> Result<Self::Instance, Error> {
        let args = [kwargs];
        let slice = args.as_slice();
        unsafe {
            protect(|| {
                rb_class_new_instance_kw(
                    slice.len() as c_int,
                    slice.as_ptr() as *const VALUE,
                    self.as_raw(),
                    RB_PASS_KEYWORDS as c_int,
                )
            })
            .and_then(|v| Value::from_raw(v).try_convert())
        }
    }
}
