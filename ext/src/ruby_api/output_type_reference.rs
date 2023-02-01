use super::{
    custom_scalar_type_definition::CustomScalarTypeDefinition,
    enum_type_definition::EnumTypeDefinition, interface_type_definition::InterfaceTypeDefinition,
    object_type_definition::ObjectTypeDefinition, root, scalar::Scalar,
    union_type_definition::UnionTypeDefinition,
};
use crate::helpers::{WrappedDefinition, WrappedStruct};
use bluejay_core::definition::OutputTypeReference as CoreOutputTypeReference;
use magnus::{
    exception, function, method, scan_args::get_kwargs, scan_args::KwArgs, DataTypeFunctions,
    Error, Module, Object, RHash, TypedData, Value,
};

#[derive(Clone, Debug)]
pub enum BaseOutputTypeReference {
    BuiltinScalar(bluejay_core::BuiltinScalarDefinition),
    Enum(WrappedDefinition<EnumTypeDefinition>),
    Object(WrappedDefinition<ObjectTypeDefinition>),
    Union(WrappedDefinition<UnionTypeDefinition>),
    Interface(WrappedDefinition<InterfaceTypeDefinition>),
    CustomScalar(WrappedDefinition<CustomScalarTypeDefinition>),
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
            Self::BuiltinScalar(bsd) => {
                bluejay_core::definition::BaseOutputTypeReference::BuiltinScalarType(*bsd)
            }
            Self::Enum(etd) => bluejay_core::definition::BaseOutputTypeReference::EnumType(
                *etd.get(),
                Default::default(),
            ),
            Self::CustomScalar(cstd) => {
                bluejay_core::definition::BaseOutputTypeReference::CustomScalarType(
                    *cstd.get(),
                    Default::default(),
                )
            }
            Self::Object(otd) => bluejay_core::definition::BaseOutputTypeReference::ObjectType(
                *otd.get(),
                Default::default(),
            ),
            Self::Interface(itd) => {
                bluejay_core::definition::BaseOutputTypeReference::InterfaceType(
                    *itd.get(),
                    Default::default(),
                )
            }
            Self::Union(utd) => bluejay_core::definition::BaseOutputTypeReference::UnionType(
                *utd.get(),
                Default::default(),
            ),
        }
    }

    fn name(&self) -> &str {
        self.name()
    }
}

impl BaseOutputTypeReference {
    pub fn new(value: Value) -> Result<Self, Error> {
        if let Ok(wrapped_struct) = value.try_convert::<WrappedStruct<Scalar>>() {
            Ok(Self::BuiltinScalar(wrapped_struct.get().to_owned().into()))
        } else if let Ok(wrapped_definition) = value.try_convert() {
            Ok(Self::Enum(wrapped_definition))
        } else if let Ok(wrapped_definition) = value.try_convert() {
            Ok(Self::Object(wrapped_definition))
        } else if let Ok(wrapped_definition) = value.try_convert() {
            Ok(Self::Union(wrapped_definition))
        } else if let Ok(wrapped_definition) = value.try_convert() {
            Ok(Self::Interface(wrapped_definition))
        } else if let Ok(wrapped_definition) = value.try_convert() {
            Ok(Self::CustomScalar(wrapped_definition))
        } else {
            Err(Error::new(
                exception::type_error(),
                format!("{value} is not a valid output type"),
            ))
        }
    }

    pub fn mark(&self) {
        match self {
            Self::BuiltinScalar(_) => {}
            Self::Object(wd) => wd.mark(),
            Self::Enum(wd) => wd.mark(),
            Self::Union(wd) => wd.mark(),
            Self::Interface(wd) => wd.mark(),
            Self::CustomScalar(wd) => wd.mark(),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Self::BuiltinScalar(bstd) => bstd.name(),
            Self::CustomScalar(cstd) => cstd.as_ref().name(),
            Self::Enum(etd) => etd.as_ref().name(),
            Self::Interface(itd) => itd.as_ref().name(),
            Self::Object(otd) => otd.as_ref().name(),
            Self::Union(utd) => utd.as_ref().name(),
        }
    }

    fn sorbet_type(&self) -> String {
        match self {
            Self::BuiltinScalar(bstd) => Scalar::from(*bstd)
                .sorbet_type_fully_qualified_name()
                .to_owned(),
            Self::CustomScalar(_) => "T.untyped".to_string(),
            Self::Enum(_) => "String".to_string(),
            Self::Interface(itd) => format!("{}::Interface", itd.fully_qualified_name()),
            Self::Object(otd) => format!("{}::Interface", otd.fully_qualified_name()),
            Self::Union(utd) => utd.as_ref().sorbet_type(),
        }
    }
}

#[derive(Clone, Debug, TypedData)]
#[magnus(class = "Bluejay::OutputTypeReference", mark)]
#[repr(transparent)]
pub struct OutputTypeReference(
    CoreOutputTypeReference<BaseOutputTypeReference, WrappedOutputTypeReference>,
);

impl AsRef<CoreOutputTypeReference<BaseOutputTypeReference, WrappedOutputTypeReference>>
    for OutputTypeReference
{
    fn as_ref(
        &self,
    ) -> &CoreOutputTypeReference<BaseOutputTypeReference, WrappedOutputTypeReference> {
        &self.0
    }
}

impl bluejay_core::definition::AbstractOutputTypeReference for OutputTypeReference {
    type BaseOutputTypeReference = BaseOutputTypeReference;
    type Wrapper = WrappedOutputTypeReference;
}

impl DataTypeFunctions for OutputTypeReference {
    fn mark(&self) {
        match &self.0 {
            CoreOutputTypeReference::List(inner, _) => inner.mark(),
            CoreOutputTypeReference::Base(inner, _) => inner.mark(),
        }
    }
}

impl From<CoreOutputTypeReference<BaseOutputTypeReference, WrappedOutputTypeReference>>
    for OutputTypeReference
{
    fn from(
        value: CoreOutputTypeReference<BaseOutputTypeReference, WrappedOutputTypeReference>,
    ) -> Self {
        Self(value)
    }
}

#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct WrappedOutputTypeReference(WrappedStruct<OutputTypeReference>);

impl AsRef<CoreOutputTypeReference<BaseOutputTypeReference, Self>> for WrappedOutputTypeReference {
    fn as_ref(&self) -> &CoreOutputTypeReference<BaseOutputTypeReference, Self> {
        self.0.get().as_ref()
    }
}

impl WrappedOutputTypeReference {
    fn mark(&self) {
        self.0.mark()
    }

    pub fn get(&self) -> &OutputTypeReference {
        self.0.get()
    }
}

impl From<WrappedStruct<OutputTypeReference>> for WrappedOutputTypeReference {
    fn from(value: WrappedStruct<OutputTypeReference>) -> Self {
        Self(value)
    }
}

impl From<OutputTypeReference> for WrappedOutputTypeReference {
    fn from(value: OutputTypeReference) -> Self {
        Self(WrappedStruct::wrap(value))
    }
}

impl AsRef<WrappedStruct<OutputTypeReference>> for WrappedOutputTypeReference {
    fn as_ref(&self) -> &WrappedStruct<OutputTypeReference> {
        &self.0
    }
}

impl OutputTypeReference {
    pub fn new(kw: RHash) -> Result<Self, Error> {
        let args: KwArgs<(Value, bool), (), ()> = get_kwargs(kw, &["type", "required"], &[])?;
        let (r#type, required) = args.required;
        let base = BaseOutputTypeReference::new(r#type)?;
        Ok(Self(CoreOutputTypeReference::Base(base, required)))
    }

    pub fn list(kw: RHash) -> Result<Self, Error> {
        let args: KwArgs<(WrappedStruct<OutputTypeReference>, bool), (), ()> =
            get_kwargs(kw, &["type", "required"], &[])?;
        let (r#type, required) = args.required;
        Ok(Self(CoreOutputTypeReference::List(r#type.into(), required)))
    }

    fn is_list(&self) -> bool {
        matches!(&self.0, CoreOutputTypeReference::List(_, _))
    }

    fn is_base(&self) -> bool {
        matches!(&self.0, CoreOutputTypeReference::Base(_, _))
    }

    fn is_required(&self) -> bool {
        self.0.is_required()
    }

    fn unwrap_list(&self) -> Result<WrappedStruct<OutputTypeReference>, Error> {
        match &self.0 {
            CoreOutputTypeReference::List(inner, _) => Ok(*inner.as_ref()),
            CoreOutputTypeReference::Base(_, _) => Err(Error::new(
                exception::runtime_error(),
                "Tried to unwrap a non-list OutputTypeReference".to_owned(),
            )),
        }
    }

    fn sorbet_type(&self) -> String {
        let is_required = self.0.is_required();
        let inner = match &self.0 {
            CoreOutputTypeReference::List(inner, _) => {
                format!("T::Array[{}]", inner.get().sorbet_type())
            }
            CoreOutputTypeReference::Base(base, _) => base.sorbet_type(),
        };
        if is_required {
            inner
        } else {
            format!("T.nilable({inner})")
        }
    }
}

pub fn init() -> Result<(), Error> {
    let class = root().define_class("OutputTypeReference", Default::default())?;

    class.define_singleton_method("new", function!(OutputTypeReference::new, 1))?;
    class.define_singleton_method("list", function!(OutputTypeReference::list, 1))?;
    class.define_method("list?", method!(OutputTypeReference::is_list, 0))?;
    class.define_method("base?", method!(OutputTypeReference::is_base, 0))?;
    class.define_method("required?", method!(OutputTypeReference::is_required, 0))?;
    class.define_method("sorbet_type", method!(OutputTypeReference::sorbet_type, 0))?;
    class.define_method("unwrap_list", method!(OutputTypeReference::unwrap_list, 0))?;

    Ok(())
}
