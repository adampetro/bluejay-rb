use bluejay_core::AsIter;
use magnus::{typed_data::Obj, Error, IntoValue, RArray, Ruby, TryConvert, TypedData, Value};
use std::{marker::PhantomData, ops::Deref};

#[derive(Debug)]
#[repr(transparent)]
pub struct TypedFrozenRArray<T: TryConvert> {
    data: RArray,
    t: PhantomData<T>,
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
    pub fn iter_objects(&self) -> impl Iterator<Item = Obj<T>> + '_ {
        unsafe { self.data.as_slice() }
            .iter()
            .map(|val| val.try_convert().unwrap())
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

impl<T: TypedData> AsIter for TypedFrozenRArray<Obj<T>> {
    type Item = T;
    type Iterator<'a> = std::iter::Map<std::slice::Iter<'a, Value>, fn(&'a Value) -> &'a T> where T: 'a;

    fn iter(&self) -> Self::Iterator<'_> {
        unsafe { self.data.as_slice() }
            .iter()
            .map(|val| val.try_convert().unwrap())
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
