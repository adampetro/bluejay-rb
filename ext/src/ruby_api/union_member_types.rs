use super::union_member_type::UnionMemberType;
use crate::helpers::{TypedFrozenRArray, WrappedStruct};

pub type UnionMemberTypes = TypedFrozenRArray<WrappedStruct<UnionMemberType>>;

impl bluejay_core::definition::UnionMemberTypes for UnionMemberTypes {
    type UnionMemberType = UnionMemberType;
}
