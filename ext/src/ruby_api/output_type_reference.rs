use super::{
    custom_scalar_type_definition::CustomScalarTypeDefinition,
    enum_type_definition::EnumTypeDefinition, interface_type_definition::InterfaceTypeDefinition,
    object_type_definition::ObjectTypeDefinition, root, scalar::Scalar,
    union_type_definition::UnionTypeDefinition,
};
use crate::helpers::WrappedDefinition;
use bluejay_core::definition::{
    AbstractBaseOutputTypeReference, BaseOutputTypeReference as CoreBaseOutputTypeReference,
    BaseOutputTypeReferenceFromAbstract, OutputTypeReference as CoreOutputTypeReference,
};
use bluejay_core::BuiltinScalarDefinition;
use magnus::{
    exception, function, gc, method, scan_args::get_kwargs, scan_args::KwArgs, typed_data::Obj,
    DataTypeFunctions, Error, Module, Object, RHash, TypedData, Value,
};

#[derive(Debug)]
pub enum BaseOutputTypeReference {
    BuiltinScalar(BuiltinScalarDefinition),
    CustomScalar(WrappedDefinition<CustomScalarTypeDefinition>),
    Enum(WrappedDefinition<EnumTypeDefinition>),
    Object(WrappedDefinition<ObjectTypeDefinition>),
    Interface(WrappedDefinition<InterfaceTypeDefinition>),
    Union(WrappedDefinition<UnionTypeDefinition>),
}

impl AbstractBaseOutputTypeReference for BaseOutputTypeReference {
    type CustomScalarTypeDefinition = CustomScalarTypeDefinition;
    type EnumTypeDefinition = EnumTypeDefinition;
    type ObjectTypeDefinition = ObjectTypeDefinition;
    type InterfaceTypeDefinition = InterfaceTypeDefinition;
    type UnionTypeDefinition = UnionTypeDefinition;

    fn get(&self) -> BaseOutputTypeReferenceFromAbstract<'_, Self> {
        match self {
            Self::BuiltinScalar(bstd) => CoreBaseOutputTypeReference::BuiltinScalarType(*bstd),
            Self::CustomScalar(cstd) => {
                CoreBaseOutputTypeReference::CustomScalarType(cstd.as_ref())
            }
            Self::Enum(etd) => CoreBaseOutputTypeReference::EnumType(etd.as_ref()),
            Self::Object(otd) => CoreBaseOutputTypeReference::ObjectType(otd.as_ref()),
            Self::Interface(itd) => CoreBaseOutputTypeReference::InterfaceType(itd.as_ref()),
            Self::Union(utd) => CoreBaseOutputTypeReference::UnionType(utd.as_ref()),
        }
    }
}

impl BaseOutputTypeReference {
    pub fn new(value: Value) -> Result<Self, Error> {
        if let Ok(wrapped_struct) = value.try_convert::<Obj<Scalar>>() {
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
            Self::CustomScalar(cstd) => cstd
                .as_ref()
                .internal_representation_sorbet_type_name()
                .to_string(),
            Self::Enum(_) => "String".to_string(),
            Self::Interface(itd) => {
                format!("{}::Interface", itd.fully_qualified_name())
            }
            Self::Object(otd) => {
                format!("{}::Interface", otd.fully_qualified_name())
            }
            Self::Union(utd) => utd.as_ref().sorbet_type(),
        }
    }

    pub(crate) fn builtin_string() -> Self {
        Self::BuiltinScalar(BuiltinScalarDefinition::String)
    }
}

#[derive(Debug, TypedData)]
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
pub struct WrappedOutputTypeReference(Obj<OutputTypeReference>);

impl AsRef<CoreOutputTypeReference<BaseOutputTypeReference, Self>> for WrappedOutputTypeReference {
    fn as_ref(&self) -> &CoreOutputTypeReference<BaseOutputTypeReference, Self> {
        self.0.get().as_ref()
    }
}

impl WrappedOutputTypeReference {
    fn mark(&self) {
        gc::mark(self.0)
    }

    pub fn get(&self) -> &OutputTypeReference {
        self.0.get()
    }
}

impl From<Obj<OutputTypeReference>> for WrappedOutputTypeReference {
    fn from(value: Obj<OutputTypeReference>) -> Self {
        Self(value)
    }
}

impl From<OutputTypeReference> for WrappedOutputTypeReference {
    fn from(value: OutputTypeReference) -> Self {
        Self(Obj::wrap(value))
    }
}

impl AsRef<Obj<OutputTypeReference>> for WrappedOutputTypeReference {
    fn as_ref(&self) -> &Obj<OutputTypeReference> {
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
        let args: KwArgs<(Obj<OutputTypeReference>, bool), (), ()> =
            get_kwargs(kw, &["type", "required"], &[])?;
        let (r#type, required) = args.required;
        Ok(Self(CoreOutputTypeReference::List(r#type.into(), required)))
    }

    pub(crate) fn base(&self) -> &BaseOutputTypeReference {
        match &self.0 {
            CoreOutputTypeReference::Base(b, _) => b,
            CoreOutputTypeReference::List(inner, _) => inner.get().base(),
        }
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

    fn unwrap_list(&self) -> Result<Obj<OutputTypeReference>, Error> {
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
