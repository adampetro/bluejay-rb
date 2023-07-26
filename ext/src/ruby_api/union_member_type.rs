use super::{object_type_definition::ObjectTypeDefinition, root};
use crate::helpers::{WrappedDefinition, Wrapper};
use magnus::{
    function,
    scan_args::{get_kwargs, KwArgs},
    DataTypeFunctions, Error, Module, Object, RHash, TypedData,
};

#[derive(Clone, Debug, TypedData)]
#[magnus(class = "Bluejay::UnionMemberType", mark)]
pub struct UnionMemberType {
    r#type: WrappedDefinition<ObjectTypeDefinition>,
}

impl UnionMemberType {
    pub fn new(kw: RHash) -> Result<Self, Error> {
        let args: KwArgs<(Wrapper,), (), ()> = get_kwargs(kw, &["type"], &[])?;
        let (r#type,) = args.required;
        WrappedDefinition::new(r#type).map(|r#type| Self { r#type })
    }

    pub fn r#type(&self) -> WrappedDefinition<ObjectTypeDefinition> {
        self.r#type.clone()
    }
}

impl DataTypeFunctions for UnionMemberType {
    fn mark(&self) {
        self.r#type.mark();
    }
}

impl bluejay_core::definition::UnionMemberType for UnionMemberType {
    type ObjectTypeDefinition = ObjectTypeDefinition;

    fn member_type(&self) -> &Self::ObjectTypeDefinition {
        self.r#type.as_ref()
    }
}

pub fn init() -> Result<(), Error> {
    let class = root().define_class("UnionMemberType", Default::default())?;

    class.define_singleton_method("new", function!(UnionMemberType::new, 1))?;

    Ok(())
}
