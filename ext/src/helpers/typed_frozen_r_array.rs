use crate::helpers::RArrayIter;
use bluejay_core::AsIter;
use magnus::{typed_data::Obj, Error, IntoValue, RArray, Ruby, TryConvert, TypedData, Value};
use std::{fmt::Debug, marker::PhantomData, ops::Deref};

#[repr(transparent)]
pub struct TypedFrozenRArray<T: TryConvert> {
    data: RArray,
    t: PhantomData<T>,
}

impl<T: TryConvert> Debug for TypedFrozenRArray<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.data, f)
    }
}

impl<T: TryConvert> Clone for TypedFrozenRArray<T> {
    fn clone(&self) -> Self {
        Self {
            data: self.data,
            t: Default::default(),
        }
    }
}

impl<T: TryConvert> Copy for TypedFrozenRArray<T> {}

impl<T: TryConvert> PartialEq for TypedFrozenRArray<T> {
    fn eq(&self, other: &Self) -> bool {
        matches!(self.data.equal(other.data), Ok(true))
    }
}

impl<T: TryConvert> Eq for TypedFrozenRArray<T> {}

impl<T: TryConvert> TypedFrozenRArray<T> {
    pub fn new(data: RArray) -> Result<Self, Error> {
        data.freeze();
        unsafe { data.as_slice() }
            .iter()
            .try_for_each(|el| el.try_convert().map(|_: T| ()))?;
        Ok(Self {
            data,
            t: Default::default(),
        })
    }

    pub fn empty() -> Self {
        let data = RArray::new();
        data.freeze();
        Self {
            data,
            t: Default::default(),
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

impl<T: TypedData> TypedFrozenRArray<Obj<T>> {
    pub fn iter_objects(&self) -> RArrayIter<Obj<T>> {
        RArrayIter::from(&self.data)
    }

    pub fn iter(&self) -> RArrayIter<&T> {
        RArrayIter::from(&self.data)
    }
}

impl<T: TryConvert> Deref for TypedFrozenRArray<T> {
    type Target = Value;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T: TryConvert> From<TypedFrozenRArray<T>> for RArray {
    fn from(val: TypedFrozenRArray<T>) -> Self {
        val.data
    }
}

impl<T: TryConvert> TryConvert for TypedFrozenRArray<T> {
    fn try_convert(val: Value) -> Result<Self, Error> {
        RArray::try_convert(val).and_then(Self::new)
    }
}

impl<T: TryConvert> Default for TypedFrozenRArray<T> {
    fn default() -> Self {
        Self::empty()
    }
}

impl<T: TypedData> AsIter for TypedFrozenRArray<Obj<T>> {
    type Item = T;
    type Iterator<'a> = RArrayIter<'a, &'a T> where T: 'a;

    fn iter(&self) -> Self::Iterator<'_> {
        RArrayIter::from(&self.data)
    }
}

impl<T: TypedData> FromIterator<T> for TypedFrozenRArray<Obj<T>> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let data = RArray::from_iter(iter.into_iter().map(Obj::wrap));
        data.freeze();
        Self {
            data,
            t: Default::default(),
        }
    }
}

impl<T: TryConvert> IntoValue for TypedFrozenRArray<T> {
    fn into_value_with(self, handle: &Ruby) -> Value {
        self.data.into_value_with(handle)
    }
}
