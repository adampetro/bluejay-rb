use crate::execution::{CoerceResult, FieldError};
use crate::helpers::{public_name, HasDefinitionWrapper, Variables};
use crate::ruby_api::{
    base, coerce_input::CoerceInput, coercion_error::CoercionError,
    enum_value_definitions::EnumValueDefinitions, introspection, root, wrapped_value::ValueInner,
    Directives, HasVisibility, Visibility, WrappedValue,
};
use crate::visibility_scoped::ScopedEnumTypeDefinition;
use bluejay_core::{definition::prelude::*, AsIter};
use bluejay_parser::ast::Value as ParserValue;
use bluejay_validator::Path;
use magnus::{
    function, gc, memoize, scan_args::get_kwargs, scan_args::KwArgs, DataTypeFunctions, Error,
    Module, Object, RArray, RClass, RHash, RModule, RString, TypedData, Value,
};

#[derive(Debug, TypedData)]
#[magnus(class = "Bluejay::EnumTypeDefinition", mark)]
pub struct EnumTypeDefinition {
    name: String,
    description: Option<String>,
    enum_value_definitions: EnumValueDefinitions,
    directives: Directives,
    is_builtin: bool,
    visibility: Option<Visibility>,
}

impl EnumTypeDefinition {
    fn new(kw: RHash) -> Result<Self, Error> {
        let args: KwArgs<_, (), ()> = get_kwargs(
            kw,
            &[
                "name",
                "enum_value_definitions",
                "description",
                "directives",
                "ruby_class",
                "visibility",
            ],
            &[],
        )?;
        let (name, enum_value_definitions, description, directives, ruby_class, visibility): (
            String,
            EnumValueDefinitions,
            Option<String>,
            RArray,
            RClass,
            Option<Visibility>,
        ) = args.required;
        let directives = directives.try_into()?;
        let is_builtin = unsafe { ruby_class.name() }.starts_with("Bluejay::Builtin::EnumTypes");
        Ok(Self {
            name,
            description,
            enum_value_definitions,
            directives,
            is_builtin,
            visibility,
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
}

impl DataTypeFunctions for EnumTypeDefinition {
    fn mark(&self) {
        gc::mark(self.enum_value_definitions);
        self.directives.mark();
        self.visibility.as_ref().map(Visibility::mark);
    }
}

fn coerce_from_name(
    etd: &ScopedEnumTypeDefinition,
    name: &str,
    path: Path,
) -> Result<Value, Vec<CoercionError>> {
    if etd
        .enum_value_definitions()
        .iter()
        .any(|evd| evd.name() == name)
    {
        let r_value = RString::from_slice(name.as_bytes());
        Ok(*r_value)
    } else {
        Err(vec![CoercionError::new(
            format!("No member `{}` on {}", name, etd.name()),
            path.to_vec(),
        )])
    }
}

impl<'a> CoerceInput for ScopedEnumTypeDefinition<'a> {
    fn coerced_ruby_value_to_wrapped_value(
        &self,
        value: Value,
        path: Path,
    ) -> Result<Result<WrappedValue, Vec<CoercionError>>, Error> {
        let s: Result<String, _> = value.try_convert();
        match s {
            Ok(s) => {
                if self
                    .enum_value_definitions()
                    .iter()
                    .any(|evd| evd.name() == s.as_str())
                {
                    let inner = ValueInner::Enum(s);
                    Ok(Ok((value, inner).into()))
                } else {
                    Ok(Err(vec![CoercionError::new(
                        format!("No member `{}` on {}", s.as_str(), self.name()),
                        path.to_vec(),
                    )]))
                }
            }
            Err(_) => Ok(Err(vec![CoercionError::new(
                format!(
                    "No implicit conversion of {} to {}",
                    public_name(value),
                    self.name()
                ),
                path.to_vec(),
            )])),
        }
    }

    fn coerce_parser_value<const CONST: bool>(
        &self,
        value: &ParserValue<CONST>,
        path: Path,
        _: &impl Variables<CONST>,
    ) -> Result<Result<Value, Vec<CoercionError>>, Error> {
        if let ParserValue::Enum(e) = value {
            Ok(coerce_from_name(self, e.as_str(), path))
        } else {
            Ok(Err(vec![CoercionError::new(
                format!("No implicit conversion of {} to {}", value, self.name()),
                path.to_vec(),
            )]))
        }
    }

    fn coerce_ruby_const_value(
        &self,
        value: Value,
        path: Path,
    ) -> Result<Result<Value, Vec<CoercionError>>, Error> {
        let s: Result<String, _> = value.try_convert();
        match s {
            Ok(s) => Ok(coerce_from_name(self, s.as_str(), path)),
            Err(_) => Ok(Err(vec![CoercionError::new(
                format!(
                    "No implicit conversion of {} to {}",
                    public_name(value),
                    self.name()
                ),
                path.to_vec(),
            )])),
        }
    }
}

impl HasDefinitionWrapper for EnumTypeDefinition {
    type Wrapper = Value;

    fn required_module() -> RModule {
        *memoize!(RModule: base().define_module("EnumType").unwrap())
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
        self.directives.to_option()
    }

    fn is_builtin(&self) -> bool {
        self.is_builtin
    }
}

impl<'a> CoerceResult for ScopedEnumTypeDefinition<'a> {
    fn coerce_result(&self, value: Value) -> Result<Value, FieldError> {
        if value
            .try_convert()
            .ok()
            .map(|value: String| {
                self.enum_value_definitions()
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

impl introspection::Type for EnumTypeDefinition {
    type OfType = introspection::Never;

    fn description(&self) -> Option<&str> {
        self.description()
    }

    fn enum_values(&self) -> Option<EnumValueDefinitions> {
        Some(self.enum_value_definitions)
    }

    fn kind(&self) -> introspection::TypeKind {
        introspection::TypeKind::Enum
    }

    fn name(&self) -> Option<&str> {
        Some(&self.name)
    }
}

impl HasVisibility for EnumTypeDefinition {
    fn visibility(&self) -> Option<&Visibility> {
        self.visibility.as_ref()
    }
}

pub fn init() -> Result<(), Error> {
    let class = root().define_class("EnumTypeDefinition", Default::default())?;

    class.define_singleton_method("new", function!(EnumTypeDefinition::new, 1))?;
    introspection::implement_type!(EnumTypeDefinition, class);

    Ok(())
}
