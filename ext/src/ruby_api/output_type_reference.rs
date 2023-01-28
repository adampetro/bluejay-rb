use magnus::{Error, Value, exception, TypedData, DataTypeFunctions, Module, scan_args::get_kwargs, RHash, Object, function};
use super::{root, enum_type_definition::EnumTypeDefinition, object_type_definition::ObjectTypeDefinition, union_type_definition::UnionTypeDefinition, custom_scalar_type_definition::CustomScalarTypeDefinition, scalar::Scalar, interface_type_definition::InterfaceTypeDefinition};
use crate::helpers::{WrappedStruct, WrappedDefinition};

#[derive(Clone, Debug)]
pub enum BaseOutputTypeReference {
    BuiltinScalarType(bluejay_core::BuiltinScalarDefinition),
    EnumType(WrappedDefinition<EnumTypeDefinition>),
    ObjectType(WrappedDefinition<ObjectTypeDefinition>),
    UnionType(WrappedDefinition<UnionTypeDefinition>),
    InterfaceType(WrappedDefinition<InterfaceTypeDefinition>),
    CustomScalarType(WrappedDefinition<CustomScalarTypeDefinition>),
}

impl bluejay_core::definition::AbstractBaseOutputTypeReference for BaseOutputTypeReference {
    type CustomScalarTypeDefinition = CustomScalarTypeDefinition;
    type EnumTypeDefinition = EnumTypeDefinition;
    type ObjectTypeDefinition = ObjectTypeDefinition;
    type InterfaceTypeDefinition = InterfaceTypeDefinition;
    type UnionTypeDefinition = UnionTypeDefinition;
    type WrappedCustomScalarTypeDefinition = WrappedStruct<CustomScalarTypeDefinition>;
    type WrappedEnumTypeDefinition = WrappedStruct<EnumTypeDefinition>;
    type WrappedObjectTypeDefinition = WrappedStruct<ObjectTypeDefinition>;
    type WrappedInterfaceTypeDefinition = WrappedStruct<InterfaceTypeDefinition>;
    type WrappedUnionTypeDefinition = WrappedStruct<UnionTypeDefinition>;

    fn to_concrete(&self) -> bluejay_core::definition::BaseOutputTypeReferenceFromAbstract<Self> {
        match self {
            Self::BuiltinScalarType(bsd) => bluejay_core::definition::BaseOutputTypeReference::BuiltinScalarType(*bsd),
            Self::EnumType(etd) => bluejay_core::definition::BaseOutputTypeReference::EnumType(*etd.get(), Default::default()),
            Self::CustomScalarType(cstd) => bluejay_core::definition::BaseOutputTypeReference::CustomScalarType(*cstd.get(), Default::default()),
            Self::ObjectType(otd) => bluejay_core::definition::BaseOutputTypeReference::ObjectType(*otd.get(), Default::default()),
            Self::InterfaceType(itd) => bluejay_core::definition::BaseOutputTypeReference::InterfaceType(*itd.get(), Default::default()),
            Self::UnionType(utd) => bluejay_core::definition::BaseOutputTypeReference::UnionType(*utd.get(), Default::default()),
        }
    }
}

impl BaseOutputTypeReference {
    pub fn new(value: Value) -> Result<Self, Error> {
        if let Ok(wrapped_struct) = value.try_convert::<WrappedStruct<Scalar>>() {
            Ok(Self::BuiltinScalarType(wrapped_struct.get().to_owned().into()))
        } else if let Ok(wrapped_definition) = value.try_convert() {
            Ok(Self::EnumType(wrapped_definition))
        } else if let Ok(wrapped_definition) = value.try_convert() {
            Ok(Self::ObjectType(wrapped_definition))
        } else if let Ok(wrapped_definition) = value.try_convert() {
            Ok(Self::UnionType(wrapped_definition))
        } else if let Ok(wrapped_definition) = value.try_convert() {
            Ok(Self::InterfaceType(wrapped_definition))
        } else if let Ok(wrapped_definition) = value.try_convert() {
            Ok(Self::CustomScalarType(wrapped_definition))
        } else {
            Err(Error::new(
                exception::type_error(),
                format!(
                    "{} is not a valid output type",
                    value
                ),
            ))
        }
    }

    pub fn mark(&self) {
        match self {
            Self::BuiltinScalarType(_) => {},
            Self::ObjectType(wd) => wd.mark(),
            Self::EnumType(wd) => wd.mark(),
            Self::UnionType(wd) => wd.mark(),
            Self::InterfaceType(wd) => wd.mark(),
            Self::CustomScalarType(wd) => wd.mark(),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Self::BuiltinScalarType(bstd) => bstd.name(),
            Self::CustomScalarType(cstd) => cstd.as_ref().name(),
            Self::EnumType(etd) => etd.as_ref().name(),
            Self::InterfaceType(itd) => itd.as_ref().name(),
            Self::ObjectType(otd) => otd.as_ref().name(),
            Self::UnionType(utd) => utd.as_ref().name(),
        }
    }
}

#[derive(Clone, Debug, TypedData)]
#[magnus(class = "Bluejay::OutputTypeReference", mark)]
pub enum OutputTypeReference {
    Base(BaseOutputTypeReference, bool),
    List(WrappedStruct<Self>, bool),
}

impl DataTypeFunctions for OutputTypeReference {
    fn mark(&self) {
        match self {
            Self::List(inner, _) => inner.mark(),
            Self::Base(inner, _) => inner.mark(),
        }
    }
}

impl OutputTypeReference {
    pub fn new(kw: RHash) -> Result<Self, Error> {
        let args = get_kwargs(kw, &["type", "required"], &[])?;
        let (r#type, required): (Value, bool) = args.required;
        let _: () = args.optional;
        let _: () = args.splat;
        let base = BaseOutputTypeReference::new(r#type)?;
        Ok(Self::Base(base, required))
    }

    pub fn list(kw: RHash) -> Result<Self, Error> {
        let args = get_kwargs(kw, &["type", "required"], &[])?;
        let (r#type, required): (WrappedStruct<Self>, bool) = args.required;
        let _: () = args.optional;
        let _: () = args.splat;
        Ok(Self::List(r#type, required))
    }

    pub fn is_required(&self) -> bool {
        match self {
            Self::Base(_, required) => *required,
            Self::List(_, required) => *required,
        }
    }

    pub fn base(&self) -> &BaseOutputTypeReference {
        match self {
            Self::Base(inner, _) => inner,
            Self::List(inner, _) => inner.get().base(),
        }
    }
}

impl bluejay_core::definition::AbstractOutputTypeReference for OutputTypeReference {
    type BaseOutputTypeReference = BaseOutputTypeReference;

    fn to_concrete(&self) -> bluejay_core::definition::OutputTypeReferenceFromAbstract<Self> {
        match self {
            Self::Base(botr, required) => bluejay_core::definition::OutputTypeReference::Base(bluejay_core::definition::AbstractBaseOutputTypeReference::to_concrete(botr), *required),
            Self::List(inner, required) => bluejay_core::definition::OutputTypeReference::List(Box::new(bluejay_core::definition::AbstractOutputTypeReference::to_concrete(inner.get())), *required),
        }
    }

    fn base_name(&self) -> &str {
        self.base().name()
    }
}

pub fn init() -> Result<(), Error> {
    let class = root().define_class("OutputTypeReference", Default::default())?;

    class.define_singleton_method("new", function!(OutputTypeReference::new, 1))?;
    class.define_singleton_method("list", function!(OutputTypeReference::list, 1))?;

    Ok(())
}
