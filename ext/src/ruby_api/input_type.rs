use crate::helpers::{public_name, Variables, WrappedDefinition};
use crate::ruby_api::{
    introspection, root, wrapped_value::ValueInner, CoerceInput, CoercionError,
    CustomScalarTypeDefinition, EnumTypeDefinition, InputFieldsDefinition,
    InputObjectTypeDefinition, Scalar, WrappedValue,
};
use bluejay_core::definition::{
    BaseInputType as CoreBaseInputType, BaseInputTypeReference, InputType as CoreInputType,
    InputTypeReference,
};
use bluejay_core::executable::{VariableType as CoreVariableType, VariableTypeReference};
use bluejay_core::{AsIter, BuiltinScalarDefinition, Value as CoreValue, ValueReference};
use bluejay_parser::ast::executable::VariableType;
use bluejay_parser::ast::Value as ParserValue;
use magnus::{
    exception, function, gc, method, scan_args::get_kwargs, scan_args::KwArgs, typed_data::Obj,
    DataTypeFunctions, Error, Float, Integer, Module, Object, RArray, RHash, RString, TypedData,
    Value, QNIL,
};

#[derive(Debug, Clone)]
pub enum BaseInputType {
    BuiltinScalar(BuiltinScalarDefinition),
    CustomScalar(WrappedDefinition<CustomScalarTypeDefinition>),
    InputObject(WrappedDefinition<InputObjectTypeDefinition>),
    Enum(WrappedDefinition<EnumTypeDefinition>),
}

impl CoreBaseInputType for BaseInputType {
    type CustomScalarTypeDefinition = CustomScalarTypeDefinition;
    type InputObjectTypeDefinition = InputObjectTypeDefinition;
    type EnumTypeDefinition = EnumTypeDefinition;

    fn as_ref(&self) -> BaseInputTypeReference<'_, Self> {
        match self {
            Self::BuiltinScalar(bstd) => BaseInputTypeReference::BuiltinScalar(*bstd),
            Self::CustomScalar(cstd) => BaseInputTypeReference::CustomScalar(cstd.as_ref()),
            Self::Enum(etd) => BaseInputTypeReference::Enum(etd.as_ref()),
            Self::InputObject(iotd) => BaseInputTypeReference::InputObject(iotd.as_ref()),
        }
    }
}

impl BaseInputType {
    pub fn new(value: Value) -> Result<Self, Error> {
        if let Ok(wrapped_struct) = value.try_convert::<Obj<Scalar>>() {
            Ok(Self::BuiltinScalar(wrapped_struct.get().to_owned().into()))
        } else if let Ok(wrapped_definition) = value.try_convert() {
            Ok(Self::InputObject(wrapped_definition))
        } else if let Ok(wrapped_definition) = value.try_convert() {
            Ok(Self::Enum(wrapped_definition))
        } else if let Ok(wrapped_definition) = value.try_convert() {
            Ok(Self::CustomScalar(wrapped_definition))
        } else {
            Err(Error::new(
                exception::type_error(),
                format!("{value} is not a valid input type"),
            ))
        }
    }

    pub fn mark(&self) {
        match self {
            Self::BuiltinScalar(_) => {}
            Self::InputObject(wd) => wd.mark(),
            Self::Enum(wd) => wd.mark(),
            Self::CustomScalar(wd) => wd.mark(),
        }
    }

    fn sorbet_type(&self) -> String {
        match self {
            Self::BuiltinScalar(bstd) => Scalar::from(*bstd)
                .sorbet_type_fully_qualified_name()
                .to_owned(),
            Self::CustomScalar(_) => "T.untyped".to_string(),
            Self::Enum(_) => "String".to_string(),
            Self::InputObject(iotd) => iotd.fully_qualified_name(),
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
        if let Some(f) = Float::from_value(value) {
            let finite: bool = f.to_f64().is_finite();
            if finite {
                Ok(value)
            } else {
                Err(vec![CoercionError::new(
                    "Float values must be finite".to_string(),
                    path.to_owned(),
                )])
            }
        } else if let Some(i) = Integer::from_value(value) {
            Self::coerce_integer(value, path).map(|_| *Float::from_f64(i.to_i32().unwrap().into()))
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

    fn coerce_parser_value<const CONST: bool>(
        t: &BuiltinScalarDefinition,
        value: &ParserValue<CONST>,
        path: &[String],
    ) -> Result<Value, Vec<CoercionError>> {
        match (t, value.as_ref()) {
            (BuiltinScalarDefinition::Boolean, ValueReference::Boolean(b)) => {
                let r_value: Value = if b { *magnus::QTRUE } else { *magnus::QFALSE };
                Ok(r_value)
            }
            (BuiltinScalarDefinition::Float, ValueReference::Float(f)) => Ok(*Float::from_f64(f)),
            (BuiltinScalarDefinition::Float, ValueReference::Integer(i)) => {
                Ok(*Float::from_f64(i.into()))
            }
            (BuiltinScalarDefinition::ID, ValueReference::Integer(i)) => {
                Ok(*Integer::from_i64(i.into()))
            }
            (
                BuiltinScalarDefinition::ID | BuiltinScalarDefinition::String,
                ValueReference::String(s),
            ) => Ok(*RString::new(s)),
            (BuiltinScalarDefinition::Int, ValueReference::Integer(i)) => {
                Ok(*Integer::from_i64(i.into()))
            }
            _ => Err(vec![CoercionError::new(
                format!("No implicit conversion of {value} to {t}"),
                path.to_owned(),
            )]),
        }
    }
}

impl CoerceInput for BuiltinScalarDefinition {
    fn coerced_ruby_value_to_wrapped_value(
        &self,
        value: Value,
        path: &[String],
    ) -> Result<Result<WrappedValue, Vec<CoercionError>>, Error> {
        let r_value_result = match self {
            Self::String => BaseInputType::coerce_string(value, path),
            Self::Int => BaseInputType::coerce_integer(value, path),
            Self::Float => BaseInputType::coerce_float(value, path),
            Self::Boolean => BaseInputType::coerce_boolean(value, path),
            Self::ID => BaseInputType::coerce_id(value, path),
        };
        match r_value_result {
            Ok(value) => value.try_into().map(Ok),
            Err(err) => Ok(Err(err)),
        }
    }

    fn coerce_parser_value<const CONST: bool>(
        &self,
        value: &ParserValue<CONST>,
        path: &[String],
        _: &impl Variables<CONST>,
    ) -> Result<Result<Value, Vec<CoercionError>>, Error> {
        Ok(BaseInputType::coerce_parser_value(self, value, path))
    }

    fn coerce_ruby_const_value(
        &self,
        value: Value,
        path: &[String],
    ) -> Result<Result<Value, Vec<CoercionError>>, Error> {
        Ok(match self {
            Self::String => BaseInputType::coerce_string(value, path),
            Self::Int => BaseInputType::coerce_integer(value, path),
            Self::Float => BaseInputType::coerce_float(value, path),
            Self::Boolean => BaseInputType::coerce_boolean(value, path),
            Self::ID => BaseInputType::coerce_id(value, path),
        })
    }
}

impl CoerceInput for BaseInputType {
    fn coerced_ruby_value_to_wrapped_value(
        &self,
        value: Value,
        path: &[String],
    ) -> Result<Result<WrappedValue, Vec<CoercionError>>, Error> {
        match self {
            Self::BuiltinScalar(bstd) => bstd.coerced_ruby_value_to_wrapped_value(value, path),
            Self::InputObject(wrapped_definition) => wrapped_definition
                .as_ref()
                .coerced_ruby_value_to_wrapped_value(value, path),
            Self::Enum(wrapped_definition) => wrapped_definition
                .as_ref()
                .coerced_ruby_value_to_wrapped_value(value, path),
            Self::CustomScalar(wrapped_definition) => wrapped_definition
                .as_ref()
                .coerced_ruby_value_to_wrapped_value(value, path),
        }
    }

    fn coerce_parser_value<const CONST: bool>(
        &self,
        value: &ParserValue<CONST>,
        path: &[String],
        variables: &impl Variables<CONST>,
    ) -> Result<Result<Value, Vec<CoercionError>>, Error> {
        match self {
            Self::BuiltinScalar(bstd) => bstd.coerce_parser_value(value, path, variables),
            Self::CustomScalar(cstd) => cstd.as_ref().coerce_parser_value(value, path, variables),
            Self::Enum(etd) => etd.as_ref().coerce_parser_value(value, path, variables),
            Self::InputObject(iotd) => iotd.as_ref().coerce_parser_value(value, path, variables),
        }
    }

    fn coerce_ruby_const_value(
        &self,
        value: Value,
        path: &[String],
    ) -> Result<Result<Value, Vec<CoercionError>>, Error> {
        match self {
            Self::BuiltinScalar(bstd) => bstd.coerce_ruby_const_value(value, path),
            Self::InputObject(wrapped_definition) => wrapped_definition
                .as_ref()
                .coerce_ruby_const_value(value, path),
            Self::Enum(wrapped_definition) => wrapped_definition
                .as_ref()
                .coerce_ruby_const_value(value, path),
            Self::CustomScalar(wrapped_definition) => wrapped_definition
                .as_ref()
                .coerce_ruby_const_value(value, path),
        }
    }
}

impl introspection::Type for BaseInputType {
    type OfType = introspection::Never;

    fn description(&self) -> Option<&str> {
        match self {
            Self::BuiltinScalar(_) => None,
            Self::CustomScalar(cstd) => cstd.as_ref().description(),
            Self::Enum(etd) => etd.as_ref().description(),
            Self::InputObject(iotd) => iotd.as_ref().description(),
        }
    }

    fn kind(&self) -> introspection::TypeKind {
        match self {
            Self::BuiltinScalar(_) | Self::CustomScalar(_) => introspection::TypeKind::Scalar,
            Self::Enum(_) => introspection::TypeKind::Enum,
            Self::InputObject(_) => introspection::TypeKind::InputObject,
        }
    }

    fn name(&self) -> Option<&str> {
        Some(match self {
            Self::BuiltinScalar(bstd) => bstd.name(),
            Self::CustomScalar(cstd) => cstd.as_ref().name(),
            Self::Enum(etd) => etd.as_ref().name(),
            Self::InputObject(iotd) => iotd.as_ref().name(),
        })
    }

    fn input_fields(&self) -> Option<InputFieldsDefinition> {
        match self {
            Self::InputObject(iotd) => iotd.as_ref().input_fields(),
            _ => None,
        }
    }

    fn specified_by_url(&self) -> Option<&str> {
        match self {
            Self::CustomScalar(cstd) => cstd.as_ref().specified_by_url(),
            _ => None,
        }
    }
}

#[derive(Debug, TypedData, Clone)]
#[magnus(class = "Bluejay::InputType", mark)]
pub enum InputType {
    Base(BaseInputType, bool),
    List(Obj<Self>, bool),
}

impl CoreInputType for InputType {
    type BaseInputType = BaseInputType;

    fn as_ref(&self) -> InputTypeReference<'_, Self> {
        match self {
            Self::Base(base, required) => InputTypeReference::Base(base, *required),
            Self::List(inner, required) => InputTypeReference::List(inner.get(), *required),
        }
    }
}

impl DataTypeFunctions for InputType {
    fn mark(&self) {
        match self {
            Self::Base(base, _) => base.mark(),
            Self::List(inner, _) => gc::mark(*inner),
        }
    }
}

impl InputType {
    pub fn new(kw: RHash) -> Result<Self, Error> {
        let args: KwArgs<(Value, bool), (), ()> = get_kwargs(kw, &["type", "required"], &[])?;
        let (r#type, required) = args.required;
        let base = BaseInputType::new(r#type)?;
        Ok(Self::Base(base, required))
    }

    pub fn list(kw: RHash) -> Result<Self, Error> {
        let args: KwArgs<(Obj<Self>, bool), (), ()> = get_kwargs(kw, &["type", "required"], &[])?;
        let (r#type, required) = args.required;
        Ok(Self::List(r#type, required))
    }

    fn coerce_required_ruby<
        T,
        F: Fn(Value, &[String]) -> Result<Result<T, Vec<CoercionError>>, Error>,
        G: Fn() -> Result<T, Error>,
    >(
        value: Value,
        required: bool,
        path: &[String],
        f: F,
        generate_empty: G,
    ) -> Result<Result<T, Vec<CoercionError>>, Error> {
        if required && value.is_nil() {
            Ok(Err(vec![CoercionError::new(
                "Got null when a non-null value was expected".to_owned(),
                path.to_owned(),
            )]))
        } else if value.is_nil() {
            generate_empty().map(Ok)
        } else {
            f(value, path)
        }
    }

    pub fn from_parser_variable_type(
        parse_variable_type: &VariableType,
        base: BaseInputType,
    ) -> Self {
        match parse_variable_type.as_ref() {
            VariableTypeReference::Named(_, required) => Self::Base(base, required),
            VariableTypeReference::List(inner, required) => Self::List(
                Obj::wrap(Self::from_parser_variable_type(inner, base)),
                required,
            ),
        }
    }

    fn is_list(&self) -> bool {
        matches!(self, Self::List(_, _))
    }

    fn is_base(&self) -> bool {
        matches!(self, Self::Base(_, _))
    }

    pub fn is_required(&self) -> bool {
        self.as_ref().is_required()
    }

    fn unwrap_list(&self) -> Result<Obj<Self>, Error> {
        match self {
            Self::List(inner, _) => Ok(*inner),
            Self::Base(_, _) => Err(Error::new(
                exception::runtime_error(),
                "Tried to unwrap a non-list InputTypeReference".to_owned(),
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

    fn coerce_parser_value<const CONST: bool>(
        &self,
        value: &ParserValue<CONST>,
        path: &[String],
        variables: &impl Variables<CONST>,
        allow_implicit_list: bool,
    ) -> Result<Result<Value, Vec<CoercionError>>, Error> {
        let required = self.is_required();
        match value {
            ParserValue::Null(_) if required => Ok(Err(vec![CoercionError::new(
                "Got null when a non-null value was expected".to_owned(),
                path.to_owned(),
            )])),
            ParserValue::Null(_) => Ok(Ok(*QNIL)),
            ParserValue::Variable(var) => {
                let value = variables.get(var.name());
                match value {
                    Some(value) if required && value.is_nil() => Ok(Err(vec![CoercionError::new(
                        format!(
                            "Received `null` for ${}, which is invalid for {}",
                            var.name(),
                            self.as_ref().display_name(),
                        ),
                        path.to_owned(),
                    )])),
                    Some(value) => Ok(Ok(value)),
                    None => Ok(Ok(*QNIL)),
                }
            }
            _ => match self {
                Self::Base(inner, _) => inner.coerce_parser_value(value, path, variables),
                Self::List(inner, _) => {
                    let inner = inner.get();

                    if let ParserValue::List(l) = value {
                        let coerced = RArray::with_capacity(l.len());
                        let mut errors = Vec::new();

                        for (idx, value) in l.iter().enumerate() {
                            let mut path = path.to_owned();
                            path.push(idx.to_string());

                            match inner.coerce_parser_value(value, &path, variables, false)? {
                                Ok(coerced_value) => {
                                    coerced.push(coerced_value).unwrap();
                                }
                                Err(errs) => {
                                    errors.extend(errs);
                                }
                            }
                        }

                        Ok(if errors.is_empty() {
                            Ok(*coerced)
                        } else {
                            Err(errors)
                        })
                    } else if allow_implicit_list {
                        let inner_result =
                            inner.coerce_parser_value(value, path, variables, true)?;
                        Ok(inner_result.map(|coerced_value| *RArray::from_slice(&[coerced_value])))
                    } else {
                        Ok(Err(vec![CoercionError::new(
                            format!(
                                "No implicit conversion of {value} to {}",
                                self.as_ref().display_name()
                            ),
                            path.to_owned(),
                        )]))
                    }
                }
            },
        }
    }

    fn coerce_ruby_const_value(
        &self,
        value: Value,
        path: &[String],
        allow_implicit_list: bool,
    ) -> Result<Result<Value, Vec<CoercionError>>, Error> {
        match self {
            Self::Base(inner, required) => Self::coerce_required_ruby(
                value,
                *required,
                path,
                |value, path| inner.coerce_ruby_const_value(value, path),
                || Ok(*QNIL),
            ),
            Self::List(inner, required) => Self::coerce_required_ruby(
                value,
                *required,
                path,
                |value, path| {
                    let inner = inner.get();

                    if let Some(array) = RArray::from_value(value) {
                        let coerced = RArray::with_capacity(array.len());
                        let mut errors = Vec::new();

                        unsafe {
                            for (idx, value) in array.as_slice().iter().enumerate() {
                                let mut path = path.to_owned();
                                path.push(idx.to_string());

                                match inner.coerce_ruby_const_value(*value, &path, false)? {
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
                    } else if allow_implicit_list {
                        let inner_result = inner.coerce_ruby_const_value(value, path, true)?;
                        Ok(inner_result.map(|coerced_value| *RArray::from_slice(&[coerced_value])))
                    } else {
                        Ok(Err(vec![CoercionError::new(
                            format!(
                                "No implicit conversion of {} to {}",
                                public_name(value),
                                self.as_ref().display_name()
                            ),
                            path.to_owned(),
                        )]))
                    }
                },
                || Ok(*QNIL),
            ),
        }
    }
}

impl CoerceInput for InputType {
    fn coerced_ruby_value_to_wrapped_value(
        &self,
        value: Value,
        path: &[String],
    ) -> Result<Result<WrappedValue, Vec<CoercionError>>, Error> {
        match self {
            Self::Base(inner, required) => Self::coerce_required_ruby(
                value,
                *required,
                path,
                |value, path| inner.coerced_ruby_value_to_wrapped_value(value, path),
                || (*QNIL).try_into(),
            ),
            Self::List(inner, required) => Self::coerce_required_ruby(
                value,
                *required,
                path,
                |value, path| {
                    let inner = inner.get();

                    if let Some(array) = RArray::from_value(value) {
                        let coerced = RArray::with_capacity(array.len());
                        let mut inner_value = Vec::with_capacity(array.len());
                        let mut errors = Vec::new();

                        unsafe {
                            for (idx, value) in array.as_slice().iter().enumerate() {
                                let mut path = path.to_owned();
                                path.push(idx.to_string());

                                match inner.coerced_ruby_value_to_wrapped_value(*value, &path)? {
                                    Ok(coerced_value) => {
                                        let (r_value, inner) = coerced_value.into();
                                        coerced.push(r_value).unwrap();
                                        inner_value.push(inner);
                                    }
                                    Err(errs) => {
                                        errors.extend(errs);
                                    }
                                }
                            }
                        }

                        Ok(if errors.is_empty() {
                            Ok((*coerced, ValueInner::List(inner_value.into())).into())
                        } else {
                            Err(errors)
                        })
                    } else {
                        let inner_result =
                            inner.coerced_ruby_value_to_wrapped_value(value, path)?;
                        Ok(inner_result.map(|coerced_value| {
                            let (r_value, inner) = coerced_value.into();
                            (
                                *RArray::from_slice(&[r_value]),
                                ValueInner::List(vec![inner].into()),
                            )
                                .into()
                        }))
                    }
                },
                || (*QNIL).try_into(),
            ),
        }
    }

    fn coerce_parser_value<const CONST: bool>(
        &self,
        value: &ParserValue<CONST>,
        path: &[String],
        variables: &impl Variables<CONST>,
    ) -> Result<Result<Value, Vec<CoercionError>>, Error> {
        self.coerce_parser_value(value, path, variables, true)
    }

    fn coerce_ruby_const_value(
        &self,
        value: Value,
        path: &[String],
    ) -> Result<Result<Value, Vec<CoercionError>>, Error> {
        self.coerce_ruby_const_value(value, path, true)
    }
}

impl introspection::Type for InputType {
    type OfType = Self;

    fn description(&self) -> Option<&str> {
        match self {
            Self::Base(base, required) if !required => base.description(),
            _ => None,
        }
    }

    fn kind(&self) -> introspection::TypeKind {
        match self {
            Self::Base(base, required) => {
                if *required {
                    introspection::TypeKind::NonNull
                } else {
                    base.kind()
                }
            }
            Self::List(_, required) => {
                if *required {
                    introspection::TypeKind::NonNull
                } else {
                    introspection::TypeKind::List
                }
            }
        }
    }

    fn input_fields(&self) -> Option<InputFieldsDefinition> {
        match self {
            Self::Base(base, required) if !required => base.input_fields(),
            _ => None,
        }
    }

    fn name(&self) -> Option<&str> {
        match self {
            Self::Base(base, required) if !required => base.name(),
            _ => None,
        }
    }

    fn of_type(&self) -> Option<Self::OfType> {
        match self {
            Self::Base(base, required) => required.then(|| Self::Base(base.clone(), false)),
            Self::List(inner, required) => {
                if *required {
                    Some(Self::List(*inner, false))
                } else {
                    Some(inner.get().clone())
                }
            }
        }
    }
}

pub fn init() -> Result<(), Error> {
    let class = root().define_class("InputType", Default::default())?;

    class.define_singleton_method("new", function!(InputType::new, 1))?;
    class.define_singleton_method("list", function!(InputType::list, 1))?;
    class.define_method("list?", method!(InputType::is_list, 0))?;
    class.define_method("base?", method!(InputType::is_base, 0))?;
    class.define_method("required?", method!(InputType::is_required, 0))?;
    class.define_method("sorbet_type", method!(InputType::sorbet_type, 0))?;
    class.define_method("unwrap_list", method!(InputType::unwrap_list, 0))?;
    introspection::implement_type!(InputType, class);

    Ok(())
}
