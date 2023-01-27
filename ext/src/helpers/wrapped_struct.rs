use magnus::{error::Error, exception, gc, value::Value, RTypedData, TryConvert, TypedData, RArray};
use std::{marker::PhantomData, ops::Deref};

/// A small wrapper for `RTypedData` that keeps track of the concrete struct
/// type, and the underlying [`Value`] for GC purposes.
#[derive(Debug)]
#[repr(transparent)]
pub struct WrappedStruct<T: TypedData> {
    inner: RTypedData,
    phantom: PhantomData<T>,
}

impl<T: TypedData> Copy for WrappedStruct<T>  {}

impl<T: TypedData> Clone for WrappedStruct<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: TypedData> WrappedStruct<T> {
    /// Gets the underlying struct.
    pub fn get(&self) -> &T {
        self.inner.get().unwrap()
    }

    /// Get the Ruby [`Value`] for this struct.
    pub fn to_value(&self) -> Value {
        self.inner.into()
    }

    /// Marks the Ruby [`Value`] for GC.
    pub fn mark(&self) {
        gc::mark(&self.inner.into());
    }

    pub fn wrap(data: T) -> Self {
        let inner = RTypedData::wrap(data);
        Self {
            inner,
            phantom: PhantomData,
        }
    }
}

impl<T: TypedData> From<WrappedStruct<T>> for Value {
    fn from(wrapped_struct: WrappedStruct<T>) -> Self {
        wrapped_struct.to_value()
    }
}

impl<T: TypedData> Deref for WrappedStruct<T> {
    type Target = RTypedData;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T: TypedData> From<T> for WrappedStruct<T> {
    fn from(t: T) -> Self {
        Self {
            inner: RTypedData::wrap(t).into(),
            phantom: PhantomData,
        }
    }
}

impl<T> TryConvert for WrappedStruct<T>
where
    T: TypedData,
{
    fn try_convert(val: Value) -> Result<Self, Error> {
        let inner = RTypedData::from_value(val).ok_or_else(|| {
            Error::new(
                exception::type_error(),
                format!(
                    "no implicit conversion of {} into {}",
                    unsafe { val.classname() },
                    T::class()
                ),
            )
        })?;
        inner.get::<T>()?;

        Ok(Self {
            inner,
            phantom: PhantomData,
        })
    }
}

impl<T: TypedData> AsRef<T> for WrappedStruct<T> {
    fn as_ref(&self) -> &T {
        self.get()
    }
}

pub type WrappedStructMap<'a, T> = std::iter::Map<std::slice::Iter<'a, WrappedStruct<T>>, fn(&'a WrappedStruct<T>) -> &'a T>;
