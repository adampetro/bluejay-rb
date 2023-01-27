use super::{union_member_type::UnionMemberType};
use crate::helpers::{WrappedStruct, from_rarray, WrappedStructMap};
use magnus::{RArray, Error};

#[derive(Clone, Debug)]
pub struct UnionMemberTypes(Vec<WrappedStruct<UnionMemberType>>);

impl UnionMemberTypes {
    pub fn new(union_member_types: RArray) -> Result<Self, Error> {
        from_rarray(union_member_types).map(Self)
    }

    pub(crate) fn mark(&self) {
        self.0.iter().for_each(WrappedStruct::mark)
    }
}

impl AsRef<[WrappedStruct<UnionMemberType>]> for UnionMemberTypes {
    fn as_ref(&self) -> &[WrappedStruct<UnionMemberType>] {
        &self.0
    }
}

impl bluejay_core::AsIter for UnionMemberTypes {
    type Item = UnionMemberType;
    type Iterator<'a> = WrappedStructMap<'a, Self::Item>;

    fn iter<'a>(&'a self) -> Self::Iterator<'a> {
        self.0.iter().map(|ws| ws.get())
    }
}

impl bluejay_core::definition::UnionMemberTypes for UnionMemberTypes {
    type UnionMemberType = UnionMemberType;
}
