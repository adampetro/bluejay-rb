use crate::helpers::WrappedDefinition;
use crate::ruby_api::{
    BaseInputType, BaseOutputType, CustomScalarTypeDefinition, EnumTypeDefinition,
    InputObjectTypeDefinition, InterfaceTypeDefinition, ObjectTypeDefinition, Scalar,
    UnionTypeDefinition,
};
use bluejay_core::definition::{TypeDefinition as CoreTypeDefinition, TypeDefinitionReference};
use bluejay_core::BuiltinScalarDefinition;
use magnus::{Error, IntoValue};
use magnus::{Ruby, Value};

#[derive(Debug, Clone, PartialEq)]
pub enum TypeDefinition {
    BuiltinScalar(BuiltinScalarDefinition),
    CustomScalar(WrappedDefinition<CustomScalarTypeDefinition>),
    Object(WrappedDefinition<ObjectTypeDefinition>),
    InputObject(WrappedDefinition<InputObjectTypeDefinition>),
    Enum(WrappedDefinition<EnumTypeDefinition>),
    Union(WrappedDefinition<UnionTypeDefinition>),
    Interface(WrappedDefinition<InterfaceTypeDefinition>),
}

impl TypeDefinition {
    pub(crate) fn mark(&self) {
        match self {
            Self::BuiltinScalar(_) => {}
            Self::CustomScalar(cstd) => cstd.mark(),
            Self::Object(otd) => otd.mark(),
            Self::InputObject(iotd) => iotd.mark(),
            Self::Enum(etd) => etd.mark(),
            Self::Union(utd) => utd.mark(),
            Self::Interface(itd) => itd.mark(),
        }
    }

    pub(crate) fn classname(&self) -> String {
        match self {
            Self::BuiltinScalar(bstd) => format!("Bluejay::Scalar::{}", bstd.name()),
            Self::CustomScalar(cstd) => cstd.fully_qualified_name(),
            Self::Object(otd) => otd.fully_qualified_name(),
            Self::InputObject(iotd) => iotd.fully_qualified_name(),
            Self::Enum(etd) => etd.fully_qualified_name(),
            Self::Union(utd) => utd.fully_qualified_name(),
            Self::Interface(itd) => itd.fully_qualified_name(),
        }
    }

    pub(crate) fn try_init_wrapped_definition(&self) -> Result<(), Error> {
        match self {
            Self::BuiltinScalar(_) => Ok(()),
            Self::CustomScalar(cstd) => cstd.try_init(),
            Self::Object(otd) => otd.try_init(),
            Self::InputObject(iotd) => iotd.try_init(),
            Self::Enum(etd) => etd.try_init(),
            Self::Union(utd) => utd.try_init(),
            Self::Interface(itd) => itd.try_init(),
        }
    }
}

impl CoreTypeDefinition for TypeDefinition {
    type CustomScalarTypeDefinition = CustomScalarTypeDefinition;
    type ObjectTypeDefinition = ObjectTypeDefinition;
    type InputObjectTypeDefinition = InputObjectTypeDefinition;
    type EnumTypeDefinition = EnumTypeDefinition;
    type UnionTypeDefinition = UnionTypeDefinition;
    type InterfaceTypeDefinition = InterfaceTypeDefinition;

    fn as_ref(&self) -> TypeDefinitionReference<'_, Self> {
        match self {
            Self::BuiltinScalar(bstd) => TypeDefinitionReference::BuiltinScalar(*bstd),
            Self::CustomScalar(cstd) => TypeDefinitionReference::CustomScalar(cstd.as_ref()),
            Self::Object(otd) => TypeDefinitionReference::Object(otd.as_ref()),
            Self::InputObject(iotd) => TypeDefinitionReference::InputObject(iotd.as_ref()),
            Self::Enum(etd) => TypeDefinitionReference::Enum(etd.as_ref()),
            Self::Union(utd) => TypeDefinitionReference::Union(utd.as_ref()),
            Self::Interface(itd) => TypeDefinitionReference::Interface(itd.as_ref()),
        }
    }
}

impl From<&BaseInputType> for TypeDefinition {
    fn from(value: &BaseInputType) -> Self {
        match value {
            BaseInputType::BuiltinScalar(bstd) => Self::BuiltinScalar(*bstd),
            BaseInputType::CustomScalar(cstd) => Self::CustomScalar(cstd.clone()),
            BaseInputType::Enum(etd) => Self::Enum(etd.clone()),
            BaseInputType::InputObject(iotd) => Self::InputObject(iotd.clone()),
        }
    }
}

impl TryInto<BaseInputType> for &TypeDefinition {
    type Error = ();

    fn try_into(self) -> Result<BaseInputType, Self::Error> {
        match self {
            TypeDefinition::BuiltinScalar(bstd) => Ok(BaseInputType::BuiltinScalar(*bstd)),
            TypeDefinition::CustomScalar(cstd) => Ok(BaseInputType::CustomScalar(cstd.clone())),
            TypeDefinition::Enum(etd) => Ok(BaseInputType::Enum(etd.clone())),
            TypeDefinition::InputObject(iotd) => Ok(BaseInputType::InputObject(iotd.clone())),
            TypeDefinition::Interface(_) | TypeDefinition::Object(_) | TypeDefinition::Union(_) => {
                Err(())
            }
        }
    }
}

impl From<&BaseOutputType> for TypeDefinition {
    fn from(value: &BaseOutputType) -> Self {
        match value {
            BaseOutputType::BuiltinScalar(bstd) => Self::BuiltinScalar(*bstd),
            BaseOutputType::CustomScalar(cstd) => Self::CustomScalar(cstd.clone()),
            BaseOutputType::Enum(etd) => Self::Enum(etd.clone()),
            BaseOutputType::Object(otd) => Self::Object(otd.clone()),
            BaseOutputType::Interface(itd) => Self::Interface(itd.clone()),
            BaseOutputType::Union(utd) => Self::Union(utd.clone()),
        }
    }
}

impl IntoValue for TypeDefinition {
    fn into_value_with(self, handle: &Ruby) -> Value {
        match self {
            Self::BuiltinScalar(bstd) => Scalar::from(bstd).into_value_with(handle),
            Self::CustomScalar(cstd) => cstd.into_value_with(handle),
            Self::Enum(etd) => etd.into_value_with(handle),
            Self::InputObject(iotd) => iotd.into_value_with(handle),
            Self::Interface(itd) => itd.into_value_with(handle),
            Self::Object(otd) => otd.into_value_with(handle),
            Self::Union(utd) => utd.into_value_with(handle),
        }
    }
}

impl IntoValue for &TypeDefinition {
    fn into_value_with(self, handle: &Ruby) -> Value {
        self.clone().into_value_with(handle)
    }
}
