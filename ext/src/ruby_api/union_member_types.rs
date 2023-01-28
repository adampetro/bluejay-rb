use super::{union_member_type::UnionMemberType};
use crate::helpers::{WrappedStruct, TypedFrozenRArray};

pub type UnionMemberTypes = TypedFrozenRArray<WrappedStruct<UnionMemberType>>;

impl bluejay_core::definition::UnionMemberTypes for UnionMemberTypes {
    type UnionMemberType = UnionMemberType;
}
