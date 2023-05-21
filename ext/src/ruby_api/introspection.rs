use crate::ruby_api::{
    EnumValueDefinitions, FieldsDefinition, InputFieldsDefinition, InterfaceImplementations,
    UnionMemberTypes,
};
use magnus::{typed_data::Obj, DataType, RClass, TypedData};
use strum::IntoStaticStr;

#[derive(IntoStaticStr, Clone, Copy)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum TypeKind {
    Scalar,
    Object,
    Interface,
    Union,
    Enum,
    InputObject,
    List,
    NonNull,
}

pub trait Type {
    type OfType: Type + TypedData;

    fn description(&self) -> Option<&str> {
        None
    }
    fn enum_values(&self) -> Option<EnumValueDefinitions> {
        None
    }
    fn fields(&self) -> Option<FieldsDefinition> {
        None
    }
    fn input_fields(&self) -> Option<InputFieldsDefinition> {
        None
    }
    fn interfaces(&self) -> Option<InterfaceImplementations> {
        None
    }
    fn kind(&self) -> TypeKind;
    fn name(&self) -> Option<&str> {
        None
    }
    fn of_type(&self) -> Option<Obj<Self::OfType>> {
        None
    }
    fn possible_types(&self) -> Option<UnionMemberTypes> {
        None
    }
    fn specified_by_url(&self) -> Option<&str> {
        None
    }
}

pub enum Never {}

unsafe impl TypedData for Never {
    fn class() -> RClass {
        unreachable!()
    }

    fn data_type() -> &'static DataType {
        unreachable!()
    }
}

impl Type for Never {
    type OfType = Self;

    fn kind(&self) -> TypeKind {
        unreachable!()
    }

    fn name(&self) -> Option<&str> {
        unreachable!()
    }
}

macro_rules! implement_type {
    ($t:ty, $class:ident) => {
        $class.define_method(
            "description",
            magnus::method!(<$t as crate::ruby_api::introspection::Type>::description, 0),
        )?;
        $class.define_method(
            "enum_values",
            magnus::method!(
                |t: &$t, include_deprecated: bool| {
                    <$t as crate::ruby_api::introspection::Type>::enum_values(t).map(
                        |enum_values| {
                            magnus::RArray::from_iter(
                                enum_values
                                    .iter_objects()
                                    .filter(|evd| include_deprecated || !evd.get().is_deprecated()),
                            )
                        },
                    )
                },
                1
            ),
        )?;
        $class.define_method(
            "fields",
            magnus::method!(
                |t: &$t, include_deprecated: bool| {
                    <$t as crate::ruby_api::introspection::Type>::fields(t).map(|fields| {
                        magnus::RArray::from_iter(fields.iter_objects().filter(|fd| {
                            let fd = fd.get();
                            !bluejay_core::definition::FieldDefinition::is_builtin(fd)
                                && (include_deprecated || !fd.is_deprecated())
                        }))
                    })
                },
                1
            ),
        )?;
        $class.define_method(
            "input_fields",
            magnus::method!(
                <$t as crate::ruby_api::introspection::Type>::input_fields,
                0
            ),
        )?;
        $class.define_method(
            "interfaces",
            magnus::method!(
                |t: &$t| <$t as crate::ruby_api::introspection::Type>::interfaces(t).map(
                    |interface_implemenations| {
                        magnus::RArray::from_iter(
                            bluejay_core::AsIter::iter(&interface_implemenations)
                                .map(crate::ruby_api::InterfaceImplementation::interface),
                        )
                    }
                ),
                0
            ),
        )?;
        $class.define_method(
            "kind",
            magnus::method!(
                |t: &$t| -> &'static str {
                    <$t as crate::ruby_api::introspection::Type>::kind(t).into()
                },
                0
            ),
        )?;
        $class.define_method(
            "name",
            magnus::method!(<$t as crate::ruby_api::introspection::Type>::name, 0),
        )?;
        $class.define_method(
            "of_type",
            magnus::method!(<$t as crate::ruby_api::introspection::Type>::of_type, 0),
        )?;
        $class.define_method(
            "possible_types",
            magnus::method!(
                |t: &$t| <$t as crate::ruby_api::introspection::Type>::possible_types(t).map(
                    |member_types| {
                        magnus::RArray::from_iter(
                            bluejay_core::AsIter::iter(&member_types)
                                .map(|member_type| *member_type.r#type().get()),
                        )
                    }
                ),
                0
            ),
        )?;
        $class.define_method(
            "specified_by_url",
            magnus::method!(
                <$t as crate::ruby_api::introspection::Type>::specified_by_url,
                0
            ),
        )?;
    };
}

pub(crate) use implement_type;
