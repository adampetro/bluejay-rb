use magnus::{function, Error, Module, Object, scan_args::get_kwargs, RHash, Value, memoize, TypedData, RArray, DataTypeFunctions, RClass, gc};
use super::{root, enum_value_definitions::EnumValueDefinitions, coerce_input::CoerceInput, coercion_error::CoercionError};
use crate::helpers::{public_name, HasDefinitionWrapper};
use crate::execution::{FieldError, CoerceResult};

#[derive(Debug, TypedData)]
#[magnus(class = "Bluejay::EnumTypeDefinition", mark)]
pub struct EnumTypeDefinition {
    name: String,
    description: Option<String>,
    enum_value_definitions: EnumValueDefinitions,
    ruby_class: RClass
}

impl EnumTypeDefinition {
    fn new(kw: RHash) -> Result<Self, Error> {
        let args = get_kwargs(kw, &["name", "enum_value_definitions", "description", "ruby_class"], &[])?;
        let (name, enum_value_definitions, description, ruby_class): (String, RArray, Option<String>, RClass) = args.required;
        let _: () = args.optional;
        let _: () = args.splat;
        let enum_value_definitions = EnumValueDefinitions::new(enum_value_definitions)?;
        Ok(Self { name, description, enum_value_definitions, ruby_class })
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_ref().map(String::as_str)
    }

    pub fn enum_value_definitions(&self) -> &EnumValueDefinitions {
        &self.enum_value_definitions
    }
}

impl DataTypeFunctions for EnumTypeDefinition {
    fn mark(&self) {
        self.enum_value_definitions.mark();
        gc::mark(self.ruby_class);
    }
}

impl CoerceInput for EnumTypeDefinition {
    fn coerce_input(&self, value: Value, path: &[String]) -> Result<Result<Value, Vec<CoercionError>>, Error> {
        let s: Result<String, _> = value.try_convert();
        match s {
            Ok(s) => {
                // TODO: don't use const_get
                Ok(self.ruby_class.const_get(s.as_str()).map_err(|_| vec![CoercionError::new(
                    format!("No member `{}` on {}", s.as_str(), self.name.as_str()),
                    path.to_owned(),
                )]))
            },
            Err(_) => {
                Ok(Err(vec![CoercionError::new(
                    format!("No implicit conversion of {} to {}", public_name(value), self.name.as_str()),
                    path.to_owned(),
                )]))
            }
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

    fn description(&self) -> Option<&str> {
        self.description.as_ref().map(String::as_str)
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn enum_value_definitions(&self) -> &Self::EnumValueDefinitions {
        &self.enum_value_definitions
    }
}

impl CoerceResult for EnumTypeDefinition {
    fn coerce_result(&self, value: Value) -> Result<Value, FieldError> {
        if value.is_kind_of(self.ruby_class) {
            Ok(value.funcall("serialize", ()).unwrap())
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
