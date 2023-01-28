use bluejay_core::BuiltinScalarDefinition;
use magnus::{Error, Module, TypedData, DataTypeFunctions};
use super::{root};

#[derive(Clone, Debug, TypedData)]
#[magnus(class = "Bluejay::Scalar", mark)]
pub struct Scalar(BuiltinScalarDefinition);

impl DataTypeFunctions for Scalar {}

impl Into<BuiltinScalarDefinition> for Scalar {
    fn into(self) -> BuiltinScalarDefinition {
        self.0
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
            BuiltinScalarDefinition::Float => "Float",
            BuiltinScalarDefinition::ID => "T.any(String, Integer)",
            BuiltinScalarDefinition::Int => "Integer",
            BuiltinScalarDefinition::String => "String",
        }
    }
}

pub fn init() -> Result<(), Error> {
    let class = root().define_class("Scalar", Default::default())?;

    class.const_set("Int", Scalar(BuiltinScalarDefinition::Int))?;
    class.const_set("Float", Scalar(BuiltinScalarDefinition::Float))?;
    class.const_set("String", Scalar(BuiltinScalarDefinition::String))?;
    class.const_set("Boolean", Scalar(BuiltinScalarDefinition::Boolean))?;
    class.const_set("ID", Scalar(BuiltinScalarDefinition::ID))?;

    Ok(())
}
