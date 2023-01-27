use crate::ruby_api::{ExecutionError as RubyExecutionError, CoercionError};
use crate::execution::FieldError;
use magnus::{Error as MagnusError};
use bluejay_parser::{Error as ParseError};

pub enum ExecutionError<'a> {
    NoOperationWithName { name: &'a str },
    CannotUseAnonymousOperation,
    RequiredVariableMissingValue { name: &'a str },
    ApplicationError(MagnusError),
    CoercionError(CoercionError),
    ParseError(ParseError),
    FieldError(FieldError),
}

impl<'a> Into<RubyExecutionError> for ExecutionError<'a> {
    fn into(self) -> RubyExecutionError {
        match self {
            Self::NoOperationWithName { name } => RubyExecutionError::new(format!("No operation definition named `{}`", name)),
            Self::CannotUseAnonymousOperation => RubyExecutionError::new("Operation name is required when document does not contain exactly 1 operation definition".to_string()),
            Self::RequiredVariableMissingValue { name } => RubyExecutionError::new(format!("No value was provided for required variable `${}`", name)),
            Self::ApplicationError(_) => RubyExecutionError::new("Internal error".to_string()),
            Self::CoercionError(error) =>  error.into(),
            Self::ParseError(error) => RubyExecutionError::new(error.message),
            Self::FieldError(error) => RubyExecutionError::new("Field error".to_string()),
        }
    }
}
