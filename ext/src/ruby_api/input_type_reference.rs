use super::{
    coerce_input::CoerceInput, coercion_error::CoercionError,
    custom_scalar_type_definition::CustomScalarTypeDefinition,
    enum_type_definition::EnumTypeDefinition,
    input_object_type_definition::InputObjectTypeDefinition, root, scalar::Scalar,
};
use crate::helpers::{public_name, WrappedDefinition, WrappedStruct};
use bluejay_core::definition::{
    BaseInputTypeReference as CoreBaseInputTypeReference,
    InputTypeReference as CoreInputTypeReference,
};
use bluejay_core::{ListTypeReference, NamedTypeReference};
use bluejay_parser::ast::TypeReference as ParserTypeReference;
use magnus::{
    exception, function, method, scan_args::get_kwargs, scan_args::KwArgs, DataTypeFunctions,
    Error, Float, Integer, Module, Object, RArray, RHash, RString, TypedData, Value,
};

type BaseInputTypeReferenceInner = CoreBaseInputTypeReference<
    CustomScalarTypeDefinition,
    WrappedDefinition<CustomScalarTypeDefinition>,
    InputObjectTypeDefinition,
    WrappedDefinition<InputObjectTypeDefinition>,
    EnumTypeDefinition,
    WrappedDefinition<EnumTypeDefinition>,
>;

#[derive(Debug)]
#[repr(transparent)]
pub struct BaseInputTypeReference(BaseInputTypeReferenceInner);

impl bluejay_core::definition::AbstractBaseInputTypeReference for BaseInputTypeReference {
    type CustomScalarTypeDefinition = CustomScalarTypeDefinition;
    type InputObjectTypeDefinition = InputObjectTypeDefinition;
    type EnumTypeDefinition = EnumTypeDefinition;
    type WrappedCustomScalarTypeDefinition = WrappedDefinition<CustomScalarTypeDefinition>;
    type WrappedInputObjectTypeDefinition = WrappedDefinition<InputObjectTypeDefinition>;
    type WrappedEnumTypeDefinition = WrappedDefinition<EnumTypeDefinition>;
}

impl AsRef<BaseInputTypeReferenceInner> for BaseInputTypeReference {
    fn as_ref(&self) -> &BaseInputTypeReferenceInner {
        &self.0
    }
}

impl From<BaseInputTypeReferenceInner> for BaseInputTypeReference {
    fn from(value: BaseInputTypeReferenceInner) -> Self {
        Self(value)
    }
}

impl BaseInputTypeReference {
    pub fn new(value: Value) -> Result<Self, Error> {
        if let Ok(wrapped_struct) = value.try_convert::<WrappedStruct<Scalar>>() {
            Ok(Self(CoreBaseInputTypeReference::BuiltinScalarType(
                wrapped_struct.get().to_owned().into(),
            )))
        } else if let Ok(wrapped_definition) = value.try_convert() {
            Ok(Self(CoreBaseInputTypeReference::InputObjectType(
                wrapped_definition,
                Default::default(),
            )))
        } else if let Ok(wrapped_definition) = value.try_convert() {
            Ok(Self(CoreBaseInputTypeReference::EnumType(
                wrapped_definition,
                Default::default(),
            )))
        } else if let Ok(wrapped_definition) = value.try_convert() {
            Ok(Self(CoreBaseInputTypeReference::CustomScalarType(
                wrapped_definition,
                Default::default(),
            )))
        } else {
            Err(Error::new(
                exception::type_error(),
                format!("{value} is not a valid input type"),
            ))
        }
    }

    pub fn mark(&self) {
        match &self.0 {
            CoreBaseInputTypeReference::BuiltinScalarType(_) => {}
            CoreBaseInputTypeReference::InputObjectType(wd, _) => wd.mark(),
            CoreBaseInputTypeReference::EnumType(wd, _) => wd.mark(),
            CoreBaseInputTypeReference::CustomScalarType(wd, _) => wd.mark(),
        }
    }

    fn sorbet_type(&self) -> String {
        match &self.0 {
            CoreBaseInputTypeReference::BuiltinScalarType(bstd) => Scalar::from(*bstd)
                .sorbet_type_fully_qualified_name()
                .to_owned(),
            CoreBaseInputTypeReference::CustomScalarType(_, _) => "T.untyped".to_string(),
            CoreBaseInputTypeReference::EnumType(_, _) => "String".to_string(),
            CoreBaseInputTypeReference::InputObjectType(iotd, _) => iotd.fully_qualified_name(),
        }
    }

    fn coerce_string(value: Value, path: &[String]) -> Result<Value, Vec<CoercionError>> {
        if RString::from_value(value).is_some() {
            Ok(value)
        } else {
            Err(vec![CoercionError::new(
                format!("No implicit conversion of {} to String", public_name(value)),
                path.to_owned(),
            )])
        }
    }

    fn coerce_integer(value: Value, path: &[String]) -> Result<Value, Vec<CoercionError>> {
        if let Some(int_value) = Integer::from_value(value) {
            int_value.to_i32().map(|_| value).map_err(|_| {
                vec![CoercionError::new(
                    "Integer values must fit within 32 bits signed".to_owned(),
                    path.to_owned(),
                )]
            })
        } else {
            Err(vec![CoercionError::new(
                format!(
                    "No implicit conversion of {} to integer",
                    public_name(value)
                ),
                path.to_owned(),
            )])
        }
    }

    fn coerce_float(value: Value, path: &[String]) -> Result<Value, Vec<CoercionError>> {
        if Float::from_value(value).is_some() {
            Ok(value)
        } else if Integer::from_value(value).is_some() {
            Self::coerce_integer(value, path)
        } else {
            Err(vec![CoercionError::new(
                format!("No implicit conversion of {} to Float", public_name(value)),
                path.to_owned(),
            )])
        }
    }

    fn coerce_boolean(value: Value, path: &[String]) -> Result<Value, Vec<CoercionError>> {
        if value.is_kind_of(magnus::class::true_class())
            || value.is_kind_of(magnus::class::false_class())
        {
            Ok(value)
        } else {
            Err(vec![CoercionError::new(
                format!(
                    "No implicit conversion of {} to Boolean",
                    public_name(value)
                ),
                path.to_owned(),
            )])
        }
    }

    fn coerce_id(value: Value, path: &[String]) -> Result<Value, Vec<CoercionError>> {
        if RString::from_value(value).is_some() {
            Ok(value)
        } else if Integer::from_value(value).is_some() {
            Self::coerce_integer(value, path)
        } else {
            Err(vec![CoercionError::new(
                format!("No implicit conversion of {} to ID", public_name(value)),
                path.to_owned(),
            )])
        }
    }
}

impl CoerceInput for bluejay_core::BuiltinScalarDefinition {
    fn coerce_input(
        &self,
        value: Value,
        path: &[String],
    ) -> Result<Result<Value, Vec<CoercionError>>, Error> {
        Ok(match self {
            Self::String => BaseInputTypeReference::coerce_string(value, path),
            Self::Int => BaseInputTypeReference::coerce_integer(value, path),
            Self::Float => BaseInputTypeReference::coerce_float(value, path),
            Self::Boolean => BaseInputTypeReference::coerce_boolean(value, path),
            Self::ID => BaseInputTypeReference::coerce_id(value, path),
        })
    }
}

impl CoerceInput for BaseInputTypeReference {
    fn coerce_input(
        &self,
        value: Value,
        path: &[String],
    ) -> Result<Result<Value, Vec<CoercionError>>, Error> {
        match &self.0 {
            CoreBaseInputTypeReference::BuiltinScalarType(bstd) => bstd.coerce_input(value, path),
            CoreBaseInputTypeReference::InputObjectType(wrapped_definition, _) => {
                wrapped_definition.as_ref().coerce_input(value, path)
            }
            CoreBaseInputTypeReference::EnumType(wrapped_definition, _) => {
                wrapped_definition.as_ref().coerce_input(value, path)
            }
            CoreBaseInputTypeReference::CustomScalarType(wrapped_definition, _) => {
                wrapped_definition.as_ref().coerce_input(value, path)
            }
        }
    }
}

#[derive(Debug, TypedData)]
#[magnus(class = "Bluejay::InputTypeReference", mark)]
#[repr(transparent)]
pub struct InputTypeReference(
    CoreInputTypeReference<BaseInputTypeReference, WrappedInputTypeReference>,
);

impl AsRef<CoreInputTypeReference<BaseInputTypeReference, WrappedInputTypeReference>>
    for InputTypeReference
{
    fn as_ref(&self) -> &CoreInputTypeReference<BaseInputTypeReference, WrappedInputTypeReference> {
        &self.0
    }
}

impl bluejay_core::definition::AbstractInputTypeReference for InputTypeReference {
    type BaseInputTypeReference = BaseInputTypeReference;
    type Wrapper = WrappedInputTypeReference;
}

impl DataTypeFunctions for InputTypeReference {
    fn mark(&self) {
        match &self.0 {
            CoreInputTypeReference::List(inner, _) => inner.mark(),
            CoreInputTypeReference::Base(inner, _) => inner.mark(),
        }
    }
}

#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct WrappedInputTypeReference(WrappedStruct<InputTypeReference>);

impl AsRef<CoreInputTypeReference<BaseInputTypeReference, Self>> for WrappedInputTypeReference {
    fn as_ref(&self) -> &CoreInputTypeReference<BaseInputTypeReference, Self> {
        self.0.get().as_ref()
    }
}

impl WrappedInputTypeReference {
    fn mark(&self) {
        self.0.mark()
    }

    fn get(&self) -> &InputTypeReference {
        self.0.get()
    }
}

impl From<WrappedStruct<InputTypeReference>> for WrappedInputTypeReference {
    fn from(value: WrappedStruct<InputTypeReference>) -> Self {
        Self(value)
    }
}

impl From<InputTypeReference> for WrappedInputTypeReference {
    fn from(value: InputTypeReference) -> Self {
        Self(WrappedStruct::wrap(value))
    }
}

impl AsRef<WrappedStruct<InputTypeReference>> for WrappedInputTypeReference {
    fn as_ref(&self) -> &WrappedStruct<InputTypeReference> {
        &self.0
    }
}

impl InputTypeReference {
    pub fn new(kw: RHash) -> Result<Self, Error> {
        let args: KwArgs<(Value, bool), (), ()> = get_kwargs(kw, &["type", "required"], &[])?;
        let (r#type, required) = args.required;
        let base = BaseInputTypeReference::new(r#type)?;
        Ok(Self(CoreInputTypeReference::Base(base, required)))
    }

    pub fn list(kw: RHash) -> Result<Self, Error> {
        let args: KwArgs<(WrappedStruct<InputTypeReference>, bool), (), ()> =
            get_kwargs(kw, &["type", "required"], &[])?;
        let (r#type, required) = args.required;
        Ok(Self(CoreInputTypeReference::List(r#type.into(), required)))
    }

    pub(crate) fn base(&self) -> &BaseInputTypeReference {
        match &self.0 {
            CoreInputTypeReference::Base(b, _) => b,
            CoreInputTypeReference::List(inner, _) => inner.get().base(),
        }
    }

    fn coerce_required<
        F: Fn(Value, &[String]) -> Result<Result<Value, Vec<CoercionError>>, Error>,
    >(
        value: Value,
        required: bool,
        path: &[String],
        f: F,
    ) -> Result<Result<Value, Vec<CoercionError>>, Error> {
        if required && value.is_nil() {
            Ok(Err(vec![CoercionError::new(
                "Got null when a non-null value was expected".to_owned(),
                path.to_owned(),
            )]))
        } else if value.is_nil() {
            Ok(Ok(value))
        } else {
            f(value, path)
        }
    }

    pub fn from_parser_type_reference(
        parser_type_reference: &ParserTypeReference,
        base: BaseInputTypeReference,
    ) -> Self {
        match parser_type_reference {
            ParserTypeReference::NamedType(ntr) => {
                Self(CoreInputTypeReference::Base(base, ntr.required()))
            }
            ParserTypeReference::ListType(ltr) => Self(CoreInputTypeReference::List(
                Self::from_parser_type_reference(ltr.inner(), base).into(),
                ltr.required(),
            )),
        }
    }

    fn is_list(&self) -> bool {
        matches!(&self.0, CoreInputTypeReference::List(_, _))
    }

    fn is_base(&self) -> bool {
        matches!(&self.0, CoreInputTypeReference::Base(_, _))
    }

    pub fn is_required(&self) -> bool {
        self.0.is_required()
    }

    fn unwrap_list(&self) -> Result<WrappedStruct<InputTypeReference>, Error> {
        match &self.0 {
            CoreInputTypeReference::List(inner, _) => Ok(*inner.as_ref()),
            CoreInputTypeReference::Base(_, _) => Err(Error::new(
                exception::runtime_error(),
                "Tried to unwrap a non-list InputTypeReference".to_owned(),
            )),
        }
    }

    fn sorbet_type(&self) -> String {
        let is_required = self.0.is_required();
        let inner = match &self.0 {
            CoreInputTypeReference::List(inner, _) => {
                format!("T::Array[{}]", inner.get().sorbet_type())
            }
            CoreInputTypeReference::Base(base, _) => base.sorbet_type(),
        };
        if is_required {
            inner
        } else {
            format!("T.nilable({inner})")
        }
    }
}

impl CoerceInput for InputTypeReference {
    fn coerce_input(
        &self,
        value: Value,
        path: &[String],
    ) -> Result<Result<Value, Vec<CoercionError>>, Error> {
        match &self.0 {
            CoreInputTypeReference::Base(inner, required) => {
                Self::coerce_required(value, *required, path, |value, path| {
                    inner.coerce_input(value, path)
                })
            }
            CoreInputTypeReference::List(inner, required) => {
                Self::coerce_required(value, *required, path, |value, path| {
                    let inner = inner.get();

                    if let Some(array) = RArray::from_value(value) {
                        let coerced = RArray::new();
                        let mut errors = Vec::new();

                        unsafe {
                            for (idx, value) in array.as_slice().iter().enumerate() {
                                let mut path = path.to_owned();
                                path.push(idx.to_string());

                                match inner.coerce_input(*value, &path)? {
                                    Ok(coerced_value) => {
                                        coerced.push(coerced_value).unwrap();
                                    }
                                    Err(errs) => {
                                        errors.extend(errs);
                                    }
                                }
                            }
                        }

                        Ok(if errors.is_empty() {
                            Ok(*coerced)
                        } else {
                            Err(errors)
                        })
                    } else {
                        let inner_result = inner.coerce_input(value, path)?;
                        Ok(inner_result.map(|coerced_value| *RArray::from_slice(&[coerced_value])))
                    }
                })
            }
        }
    }
}

pub fn init() -> Result<(), Error> {
    let class = root().define_class("InputTypeReference", Default::default())?;

    class.define_singleton_method("new", function!(InputTypeReference::new, 1))?;
    class.define_singleton_method("list", function!(InputTypeReference::list, 1))?;
    class.define_method("list?", method!(InputTypeReference::is_list, 0))?;
    class.define_method("base?", method!(InputTypeReference::is_base, 0))?;
    class.define_method("required?", method!(InputTypeReference::is_required, 0))?;
    class.define_method("sorbet_type", method!(InputTypeReference::sorbet_type, 0))?;
    class.define_method("unwrap_list", method!(InputTypeReference::unwrap_list, 0))?;

    Ok(())
}
