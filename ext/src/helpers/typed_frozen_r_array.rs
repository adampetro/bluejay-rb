use std::{marker::PhantomData, ops::Deref};
use magnus::{RArray, TryConvert, Error, TypedData, Value};
use crate::helpers::WrappedStruct;
use bluejay_core::AsIter;

#[derive(Debug)]
#[repr(transparent)]
pub struct TypedFrozenRArray<T: TryConvert> {
    data: RArray,
    t: PhantomData<T>,
}

impl<T: TryConvert> Clone for TypedFrozenRArray<T> {
    fn clone(&self) -> Self {
        Self { data: self.data, t: Default::default() }
    }
}

impl<T: TryConvert> Copy for TypedFrozenRArray<T> {}

impl<T: TryConvert> TypedFrozenRArray<T> {
    pub fn new(data: RArray) -> Result<Self, Error> {
        data.freeze();
        unsafe { data.as_slice() }.iter().try_for_each(|el| el.try_convert().map(|_: T| ()))?;
        Ok(Self { data, t: Default::default() })
    }

    pub fn empty() -> Self {
        let data = RArray::new();
        data.freeze();
        Self { data, t: Default::default() }
    }
}

impl<T: TryConvert> Deref for TypedFrozenRArray<T> {
    type Target = Value;

    fn deref(&self) -> &Self::Target {
        &*self.data
    }
}

impl<T: TryConvert> Into<RArray> for TypedFrozenRArray<T> {
    fn into(self) -> RArray {
        self.data
    }
}

impl<T: TypedData> AsIter for TypedFrozenRArray<WrappedStruct<T>> {
    type Item = T;
    type Iterator<'a> = std::iter::Map<std::slice::Iter<'a, Value>, fn(&'a Value) -> &'a T> where T: 'a;

    fn iter<'a>(&'a self) -> Self::Iterator<'a> {
        unsafe { self.data.as_slice() }.iter().map(|val| val.try_convert().unwrap())
    }
}
