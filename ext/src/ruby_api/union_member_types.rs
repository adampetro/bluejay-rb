use super::union_member_type::UnionMemberType;
use crate::helpers::TypedFrozenRArray;
use magnus::typed_data::Obj;

pub type UnionMemberTypes = TypedFrozenRArray<Obj<UnionMemberType>>;

impl bluejay_core::definition::UnionMemberTypes for UnionMemberTypes {
    type UnionMemberType = UnionMemberType;
}
