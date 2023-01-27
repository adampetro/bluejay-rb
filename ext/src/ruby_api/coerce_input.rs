use super::coercion_error::CoercionError;
use magnus::{Value, Error};

pub trait CoerceInput {
    fn coerce_input(&self, value: Value, path: &[String]) -> Result<Result<Value, Vec<CoercionError>>, Error>;
}
