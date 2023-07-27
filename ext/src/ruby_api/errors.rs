use crate::ruby_api::errors;
use magnus::{memoize, ExceptionClass, Module};

pub fn base_error() -> ExceptionClass {
    *memoize!(ExceptionClass: errors().define_error("BaseError", Default::default()).unwrap())
}

pub fn non_unique_definition_name_error() -> ExceptionClass {
    *memoize!(ExceptionClass: errors().define_error("NonUniqueDefinitionNameError", base_error()).unwrap())
}

pub fn default_value_error() -> ExceptionClass {
    *memoize!(ExceptionClass: errors().define_error("DefaultValueError", base_error()).unwrap())
}
