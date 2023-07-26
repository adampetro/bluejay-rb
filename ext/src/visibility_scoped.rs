use crate::helpers::Warden;
use crate::ruby_api::SchemaDefinition;
use bluejay_visibility::Cache;

pub type VisibilityCache<'a> = Cache<'a, SchemaDefinition, Warden>;

macro_rules! scoped_types {
    ($($ty:ty, $(,)?)*) => {
        paste::paste! {
            $(
                pub type [<Scoped $ty>]<'a> = bluejay_visibility::$ty<'a, SchemaDefinition, Warden>;
            )*
        }
    };
}

scoped_types!(
    SchemaDefinition,
    ObjectTypeDefinition,
    FieldDefinition,
    OutputType,
    BaseOutputType,
    InputValueDefinition,
    EnumTypeDefinition,
    InputType,
    BaseInputType,
    ScalarTypeDefinition,
    InputObjectTypeDefinition,
    DirectiveDefinition,
    InterfaceTypeDefinition,
    UnionTypeDefinition,
);
