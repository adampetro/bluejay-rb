use crate::execution::FieldError;
use crate::ruby_api::{CoercionError, ExecutionError as RubyExecutionError};
use bluejay_parser::Error as ParseError;

#[derive(Debug)]
pub enum ExecutionError<'a> {
    NoOperationWithName { name: &'a str },
    CannotUseAnonymousOperation,
    RequiredVariableMissingValue { name: &'a str },
    ApplicationError(String),
    CoercionError(CoercionError),
    ParseError(ParseError),
    FieldError(FieldError),
}

impl<'a> From<ExecutionError<'a>> for RubyExecutionError {
    fn from(val: ExecutionError<'a>) -> Self {
        match val {
            ExecutionError::NoOperationWithName { name } => Self::new(format!("No operation definition named `{name}`")),
            ExecutionError::CannotUseAnonymousOperation => Self::new("Operation name is required when document does not contain exactly 1 operation definition"),
            ExecutionError::RequiredVariableMissingValue { name } => Self::new(format!("No value was provided for required variable `${name}`")),
            ExecutionError::ApplicationError(error) => Self::new(format!("Internal error: {error}")),
            ExecutionError::CoercionError(error) =>  error.into(),
            ExecutionError::ParseError(error) => Self::new(error.message().to_owned()),
            ExecutionError::FieldError(_) => Self::new("Field error"),
        }
    }
}
