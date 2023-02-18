use crate::execution::{CoerceResult, FieldError};
use crate::helpers::{public_name, HasDefinitionWrapper, Variables};
use crate::ruby_api::{
    coerce_input::CoerceInput, coercion_error::CoercionError,
    enum_value_definitions::EnumValueDefinitions, root, wrapped_value::ValueInner, Directives,
    WrappedValue,
};
use bluejay_core::AsIter;
use bluejay_parser::ast::Value as ParserValue;
use magnus::{
    function, gc, memoize, scan_args::get_kwargs, scan_args::KwArgs, DataTypeFunctions, Error,
    Module, Object, RArray, RClass, RHash, RString, TypedData, Value,
};

#[derive(Debug, TypedData)]
#[magnus(class = "Bluejay::EnumTypeDefinition", mark)]
pub struct EnumTypeDefinition {
    name: String,
    description: Option<String>,
    enum_value_definitions: EnumValueDefinitions,
    directives: Directives,
}

impl EnumTypeDefinition {
    fn new(kw: RHash) -> Result<Self, Error> {
        let args: KwArgs<(String, RArray, Option<String>, RArray), (), ()> = get_kwargs(
            kw,
            &[
                "name",
                "enum_value_definitions",
                "description",
                "directives",
            ],
            &[],
        )?;
        let (name, enum_value_definitions, description, directives) = args.required;
        let enum_value_definitions = EnumValueDefinitions::new(enum_value_definitions)?;
        let directives = directives.try_into()?;
        Ok(Self {
            name,
            description,
            enum_value_definitions,
            directives,
        })
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn enum_value_definitions(&self) -> &EnumValueDefinitions {
        &self.enum_value_definitions
    }

    pub fn directives(&self) -> &Directives {
        &self.directives
    }

    fn coerce_from_name(&self, name: &str, path: &[String]) -> Result<Value, Vec<CoercionError>> {
        if self
            .enum_value_definitions
            .iter()
            .any(|evd| evd.name() == name)
        {
            let r_value = RString::from(name);
            Ok(*r_value)
        } else {
            Err(vec![CoercionError::new(
                format!("No member `{}` on {}", name, self.name.as_str()),
                path.to_owned(),
            )])
        }
    }
}

impl DataTypeFunctions for EnumTypeDefinition {
    fn mark(&self) {
        gc::mark(self.enum_value_definitions);
        self.directives.mark();
    }
}

impl CoerceInput for EnumTypeDefinition {
    fn coerced_ruby_value_to_wrapped_value(
        &self,
        value: Value,
        path: &[String],
    ) -> Result<Result<WrappedValue, Vec<CoercionError>>, Error> {
        let s: Result<String, _> = value.try_convert();
        match s {
            Ok(s) => {
                if self
                    .enum_value_definitions
                    .iter()
                    .any(|evd| evd.name() == s.as_str())
                {
                    let inner = ValueInner::Enum(s);
                    Ok(Ok((value, inner).into()))
                } else {
                    Ok(Err(vec![CoercionError::new(
                        format!("No member `{}` on {}", s.as_str(), self.name.as_str()),
                        path.to_owned(),
                    )]))
                }
            }
            Err(_) => Ok(Err(vec![CoercionError::new(
                format!(
                    "No implicit conversion of {} to {}",
                    public_name(value),
                    self.name.as_str()
                ),
                path.to_owned(),
            )])),
        }
    }

    fn coerce_parser_value<const CONST: bool>(
        &self,
        value: &ParserValue<CONST>,
        path: &[String],
        _: &impl Variables<CONST>,
    ) -> Result<Result<Value, Vec<CoercionError>>, Error> {
        if let ParserValue::Enum(e) = value {
            Ok(self.coerce_from_name(e.as_str(), path))
        } else {
            Ok(Err(vec![CoercionError::new(
                format!(
                    "No implicit conversion of {} to {}",
                    value,
                    self.name.as_str()
                ),
                path.to_owned(),
            )]))
        }
    }

    fn coerce_ruby_const_value(
        &self,
        value: Value,
        path: &[String],
    ) -> Result<Result<Value, Vec<CoercionError>>, Error> {
        let s: Result<String, _> = value.try_convert();
        match s {
            Ok(s) => Ok(self.coerce_from_name(s.as_str(), path)),
            Err(_) => Ok(Err(vec![CoercionError::new(
                format!(
                    "No implicit conversion of {} to {}",
                    public_name(value),
                    self.name.as_str()
                ),
                path.to_owned(),
            )])),
        }
    }
}

impl HasDefinitionWrapper for EnumTypeDefinition {
    fn wrapping_class() -> RClass {
        *memoize!(RClass: root().define_class("EnumType", Default::default()).unwrap())
    }
}

impl bluejay_core::definition::EnumTypeDefinition for EnumTypeDefinition {
    type EnumValueDefinitions = EnumValueDefinitions;
    type Directives = Directives;

    fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn enum_value_definitions(&self) -> &Self::EnumValueDefinitions {
        &self.enum_value_definitions
    }

    fn directives(&self) -> Option<&Self::Directives> {
        Some(&self.directives)
    }
}

impl CoerceResult for EnumTypeDefinition {
    fn coerce_result(&self, value: Value) -> Result<Value, FieldError> {
        if value
            .try_convert()
            .ok()
            .map(|value: String| {
                self.enum_value_definitions
                    .iter()
                    .any(|evd| evd.name() == value.as_str())
            })
            .unwrap_or(false)
        {
            Ok(value)
        } else {
            Err(FieldError::CannotCoerceResultToEnumType)
        }
    }
}

pub fn init() -> Result<(), Error> {
    let class = root().define_class("EnumTypeDefinition", Default::default())?;

    class.define_singleton_method("new", function!(EnumTypeDefinition::new, 1))?;

    Ok(())
}
