use crate::ruby_api::SchemaDefinition;
use bluejay_visibility::{Cache, NullWarden};

pub type VisibilityCache<'a> = Cache<'a, SchemaDefinition, NullWarden<SchemaDefinition>>;

macro_rules! scoped_types {
    ($($ty:ty, $(,)?)*) => {
        paste::paste! {
            $(
                pub type [<Scoped $ty>]<'a> = bluejay_visibility::$ty<'a, SchemaDefinition, NullWarden<SchemaDefinition>>;
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
