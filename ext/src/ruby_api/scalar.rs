use crate::ruby_api::{introspection, root};
use bluejay_core::BuiltinScalarDefinition;
use magnus::{DataTypeFunctions, Error, Module, TypedData};

#[derive(Clone, Debug, TypedData)]
#[magnus(class = "Bluejay::Scalar")]
pub struct Scalar(BuiltinScalarDefinition);

impl DataTypeFunctions for Scalar {}

impl From<Scalar> for BuiltinScalarDefinition {
    fn from(val: Scalar) -> BuiltinScalarDefinition {
        val.0
    }
}

impl From<BuiltinScalarDefinition> for Scalar {
    fn from(value: BuiltinScalarDefinition) -> Self {
        Self(value)
    }
}

impl Scalar {
    pub fn sorbet_type_fully_qualified_name(&self) -> &str {
        match self.0 {
            BuiltinScalarDefinition::Boolean => "T::Boolean",
            BuiltinScalarDefinition::Float => "Numeric",
            BuiltinScalarDefinition::ID => "T.any(String, Integer)",
            BuiltinScalarDefinition::Int => "Integer",
            BuiltinScalarDefinition::String => "String",
        }
    }
}

impl introspection::Type for Scalar {
    type OfType = introspection::Never;

    fn kind(&self) -> introspection::TypeKind {
        introspection::TypeKind::Scalar
    }

    fn name(&self) -> Option<&str> {
        Some(self.0.name())
    }
}

pub fn init() -> Result<(), Error> {
    let class = root().define_class("Scalar", Default::default())?;

    class.const_set("Int", Scalar(BuiltinScalarDefinition::Int))?;
    class.const_set("Float", Scalar(BuiltinScalarDefinition::Float))?;
    class.const_set("String", Scalar(BuiltinScalarDefinition::String))?;
    class.const_set("Boolean", Scalar(BuiltinScalarDefinition::Boolean))?;
    class.const_set("ID", Scalar(BuiltinScalarDefinition::ID))?;
    introspection::implement_type!(Scalar, class);

    Ok(())
}
