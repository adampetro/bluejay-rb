use super::coercion_error::CoercionError;
use crate::helpers::Variables;
use crate::ruby_api::WrappedValue;
use bluejay_parser::ast::Value as ParserValue;
use bluejay_validator::Path;
use magnus::{Error, Value};

pub trait CoerceInput {
    fn coerced_ruby_value_to_wrapped_value(
        &self,
        value: Value,
        path: Path,
    ) -> Result<Result<WrappedValue, Vec<CoercionError>>, Error>;

    fn coerce_ruby_const_value(
        &self,
        value: Value,
        path: Path,
    ) -> Result<Result<Value, Vec<CoercionError>>, Error>;

    fn coerce_parser_value<const CONST: bool>(
        &self,
        value: &ParserValue<CONST>,
        path: Path,
        variables: &impl Variables<CONST>,
    ) -> Result<Result<Value, Vec<CoercionError>>, Error>;
}
