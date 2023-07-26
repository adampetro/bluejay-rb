use super::{object_type_definition::ObjectTypeDefinition, root, HasVisibility, Visibility};
use crate::helpers::WrappedDefinition;
use magnus::{
    function,
    scan_args::{get_kwargs, KwArgs},
    DataTypeFunctions, Error, Module, Object, RClass, RHash, TypedData,
};

#[derive(Debug, TypedData)]
#[magnus(class = "Bluejay::UnionMemberType", mark)]
pub struct UnionMemberType {
    r#type: WrappedDefinition<ObjectTypeDefinition>,
    visibility: Option<Visibility>,
}

impl UnionMemberType {
    fn new(kw: RHash) -> Result<Self, Error> {
        let args: KwArgs<(RClass,), (Option<Option<Visibility>>,), ()> =
            get_kwargs(kw, &["type"], &["visibility"])?;
        let (r#type,) = args.required;
        let (visibility,) = args.optional;
        WrappedDefinition::new(r#type).map(|r#type| Self {
            r#type,
            visibility: visibility.flatten(),
        })
    }

    pub fn r#type(&self) -> WrappedDefinition<ObjectTypeDefinition> {
        self.r#type.clone()
    }
}

impl DataTypeFunctions for UnionMemberType {
    fn mark(&self) {
        self.r#type.mark();
        self.visibility.as_ref().map(Visibility::mark);
    }
}

impl bluejay_core::definition::UnionMemberType for UnionMemberType {
    type ObjectTypeDefinition = ObjectTypeDefinition;

    fn member_type(&self) -> &Self::ObjectTypeDefinition {
        self.r#type.as_ref()
    }
}

impl HasVisibility for UnionMemberType {
    fn visibility(&self) -> Option<&Visibility> {
        self.visibility.as_ref()
    }
}

pub fn init() -> Result<(), Error> {
    let class = root().define_class("UnionMemberType", Default::default())?;

    class.define_singleton_method("new", function!(UnionMemberType::new, 1))?;

    Ok(())
}
