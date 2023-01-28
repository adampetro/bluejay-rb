use magnus::{RArray, TryConvert, Error, gc, TypedData};
use crate::helpers::WrappedStruct;
use bluejay_core::AsIter;

#[derive(Debug)]
pub struct ObjVec<T: TypedData> {
    data: Vec<WrappedStruct<T>>,
    rarray: RArray,
}

fn from_rarray<T: TryConvert>(arr: RArray) -> Result<Vec<T>, Error> {
    arr.each().map(|value| {
        value.and_then(|value| value.try_convert())
    }).collect()
}

impl<T: TypedData> ObjVec<T> {
    pub fn new(rarray: RArray) -> Result<Self, Error> {
        rarray.freeze();
        let data = from_rarray(rarray)?;
        Ok(Self { data, rarray })
    }

    pub fn empty() -> Self {
        let rarray = RArray::new();
        rarray.freeze();
        Self { data: Vec::new(), rarray }
    }

    pub fn mark(&self) {
        gc::mark(self.rarray);
    }
}

impl<T: TypedData> AsRef<RArray> for ObjVec<T> {
    fn as_ref(&self) -> &RArray {
        &self.rarray
    }
}

impl<T: TypedData> AsIter for ObjVec<T> {
    type Item = T;
    type Iterator<'a> = std::iter::Map<std::slice::Iter<'a, WrappedStruct<T>>, fn(&'a WrappedStruct<T>) -> &'a T> where T: 'a;

    fn iter<'a>(&'a self) -> Self::Iterator<'a> {
        self.data.iter().map(WrappedStruct::get)
    }
}
