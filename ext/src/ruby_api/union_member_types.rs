use super::{union_member_type::UnionMemberType};
use crate::helpers::ObjVec;

pub type UnionMemberTypes = ObjVec<UnionMemberType>;

impl bluejay_core::definition::UnionMemberTypes for UnionMemberTypes {
    type UnionMemberType = UnionMemberType;
}
