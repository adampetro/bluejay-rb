use super::{
    custom_scalar_type_definition::CustomScalarTypeDefinition,
    enum_type_definition::EnumTypeDefinition, interface_type_definition::InterfaceTypeDefinition,
    object_type_definition::ObjectTypeDefinition, root, scalar::Scalar,
    union_type_definition::UnionTypeDefinition,
};
use crate::helpers::WrappedDefinition;
use bluejay_core::definition::{
    BaseOutputType as CoreBaseOutputType, BaseOutputTypeReference, OutputType as CoreOutputType,
    OutputTypeReference,
};
use bluejay_core::BuiltinScalarDefinition;
use magnus::{
    exception, function, gc, method, scan_args::get_kwargs, scan_args::KwArgs, typed_data::Obj,
    DataTypeFunctions, Error, Module, Object, RHash, TypedData, Value,
};

#[derive(Debug)]
pub enum BaseOutputType {
    BuiltinScalar(BuiltinScalarDefinition),
    CustomScalar(WrappedDefinition<CustomScalarTypeDefinition>),
    Enum(WrappedDefinition<EnumTypeDefinition>),
    Object(WrappedDefinition<ObjectTypeDefinition>),
    Interface(WrappedDefinition<InterfaceTypeDefinition>),
    Union(WrappedDefinition<UnionTypeDefinition>),
}

impl CoreBaseOutputType for BaseOutputType {
    type CustomScalarTypeDefinition = CustomScalarTypeDefinition;
    type EnumTypeDefinition = EnumTypeDefinition;
    type ObjectTypeDefinition = ObjectTypeDefinition;
    type InterfaceTypeDefinition = InterfaceTypeDefinition;
    type UnionTypeDefinition = UnionTypeDefinition;

    fn as_ref(&self) -> BaseOutputTypeReference<'_, Self> {
        match self {
            Self::BuiltinScalar(bstd) => BaseOutputTypeReference::BuiltinScalar(*bstd),
            Self::CustomScalar(cstd) => BaseOutputTypeReference::CustomScalar(cstd.as_ref()),
            Self::Enum(etd) => BaseOutputTypeReference::Enum(etd.as_ref()),
            Self::Object(otd) => BaseOutputTypeReference::Object(otd.as_ref()),
            Self::Interface(itd) => BaseOutputTypeReference::Interface(itd.as_ref()),
            Self::Union(utd) => BaseOutputTypeReference::Union(utd.as_ref()),
        }
    }
}

impl BaseOutputType {
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
#[magnus(class = "Bluejay::OutputType", mark)]
pub enum OutputType {
    Base(BaseOutputType, bool),
    List(Obj<Self>, bool),
}

impl CoreOutputType for OutputType {
    type BaseOutputType = BaseOutputType;

    fn as_ref(&self) -> OutputTypeReference<'_, Self> {
        match self {
            Self::Base(base, required) => OutputTypeReference::Base(base, *required),
            Self::List(inner, required) => OutputTypeReference::List(inner.get(), *required),
        }
    }
}

impl DataTypeFunctions for OutputType {
    fn mark(&self) {
        match self {
            Self::Base(base, _) => base.mark(),
            Self::List(inner, _) => gc::mark(*inner),
        }
    }
}
impl OutputType {
    pub fn new(kw: RHash) -> Result<Self, Error> {
        let args: KwArgs<(Value, bool), (), ()> = get_kwargs(kw, &["type", "required"], &[])?;
        let (r#type, required) = args.required;
        let base = BaseOutputType::new(r#type)?;
        Ok(Self::Base(base, required))
    }

    pub fn list(kw: RHash) -> Result<Self, Error> {
        let args: KwArgs<(Obj<Self>, bool), (), ()> = get_kwargs(kw, &["type", "required"], &[])?;
        let (r#type, required) = args.required;
        Ok(Self::List(r#type, required))
    }

    fn is_list(&self) -> bool {
        matches!(self, Self::List(_, _))
    }

    fn is_base(&self) -> bool {
        matches!(self, Self::Base(_, _))
    }

    fn is_required(&self) -> bool {
        self.as_ref().is_required()
    }

    fn unwrap_list(&self) -> Result<Obj<Self>, Error> {
        match self {
            Self::List(inner, _) => Ok(*inner),
            Self::Base(_, _) => Err(Error::new(
                exception::runtime_error(),
                "Tried to unwrap a non-list OutputType".to_owned(),
            )),
        }
    }

    fn sorbet_type(&self) -> String {
        let is_required = self.as_ref().is_required();
        let inner = match self {
            Self::List(inner, _) => {
                format!("T::Array[{}]", inner.get().sorbet_type())
            }
            Self::Base(base, _) => base.sorbet_type(),
        };
        if is_required {
            inner
        } else {
            format!("T.nilable({inner})")
        }
    }
}

pub fn init() -> Result<(), Error> {
    let class = root().define_class("OutputType", Default::default())?;

    class.define_singleton_method("new", function!(OutputType::new, 1))?;
    class.define_singleton_method("list", function!(OutputType::list, 1))?;
    class.define_method("list?", method!(OutputType::is_list, 0))?;
    class.define_method("base?", method!(OutputType::is_base, 0))?;
    class.define_method("required?", method!(OutputType::is_required, 0))?;
    class.define_method("sorbet_type", method!(OutputType::sorbet_type, 0))?;
    class.define_method("unwrap_list", method!(OutputType::unwrap_list, 0))?;

    Ok(())
}
