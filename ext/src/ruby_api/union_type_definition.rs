use magnus::{function, Error, Module, Object, scan_args::get_kwargs, RHash, Value, memoize, TypedData, RArray, DataTypeFunctions, RClass, gc};
use super::{root, union_member_type::UnionMemberType, object_type_definition::ObjectTypeDefinition, union_member_types::UnionMemberTypes};
use crate::helpers::{WrappedStruct, HasDefinitionWrapper};
use bluejay_core::AsIter;

#[derive(Clone, Debug, TypedData)]
#[magnus(class = "Bluejay::UnionTypeDefinition", mark)]
pub struct UnionTypeDefinition {
    name: String,
    description: Option<String>,
    member_types: UnionMemberTypes,
}

impl UnionTypeDefinition {
    fn new(kw: RHash) -> Result<Self, Error> {
        let args = get_kwargs(kw, &["name", "member_types", "description"], &[])?;
        let (name, member_types, description): (String, RArray, Option<String>) = args.required;
        let _: () = args.optional;
        let _: () = args.splat;
        let member_types = UnionMemberTypes::new(member_types)?;
        Ok(Self { name, description, member_types })
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_ref().map(String::as_str)
    }

    pub fn member_types(&self) -> &[WrappedStruct<UnionMemberType>] {
        self.member_types.as_ref()
    }

    pub fn contains_type(&self, t: &ObjectTypeDefinition) -> bool {
        self.member_types
            .iter()
            .any(|mt| mt.r#type().as_ref().name() == t.name())
    }
}

impl DataTypeFunctions for UnionTypeDefinition {
    fn mark(&self) {
        self.member_types.mark();
    }
}

impl HasDefinitionWrapper for UnionTypeDefinition {
    fn wrapping_class() -> RClass {
        *memoize!(RClass: root().define_class("UnionType", Default::default()).unwrap())
    }
}

impl bluejay_core::definition::UnionTypeDefinition for UnionTypeDefinition {
    type UnionMemberTypes = UnionMemberTypes;

    fn description(&self) -> Option<&str> {
        self.description.as_ref().map(String::as_str)
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn union_member_types(&self) -> &Self::UnionMemberTypes {
        &self.member_types
    }
}

pub fn init() -> Result<(), Error> {
    let class = root().define_class("UnionTypeDefinition", Default::default())?;

    class.define_singleton_method("new", function!(UnionTypeDefinition::new, 1))?;

    Ok(())
}
