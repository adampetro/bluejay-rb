use crate::execution::FieldError;
use crate::ruby_api::{CoercionError, ExecutionError as RubyExecutionError};
use bluejay_parser::Error as ParseError;
use bluejay_validator::Path;

#[derive(Debug)]
pub enum ExecutionError<'a> {
    NoOperationWithName { name: &'a str },
    CannotUseAnonymousOperation,
    RequiredVariableMissingValue { name: &'a str },
    ApplicationError(String),
    CoercionError(CoercionError),
    ParseError(ParseError),
    FieldError { error: FieldError, path: Path<'a> },
}

impl<'a> From<ExecutionError<'a>> for RubyExecutionError {
    fn from(val: ExecutionError<'a>) -> Self {
        match val {
            ExecutionError::NoOperationWithName { name } => Self::new(format!("No operation definition named `{name}`"), None),
            ExecutionError::CannotUseAnonymousOperation => Self::new("Operation name is required when document does not contain exactly 1 operation definition", None),
            ExecutionError::RequiredVariableMissingValue { name } => Self::new(format!("No value was provided for required variable `${name}`"), None),
            ExecutionError::ApplicationError(error) => Self::new(format!("Internal error: {error}"), None),
            ExecutionError::CoercionError(error) =>  error.into(),
            ExecutionError::ParseError(error) => Self::new(error.message().to_owned(), None),
            ExecutionError::FieldError { error, path } => Self::new(error.message().to_string(), Some(path.to_vec())),
        }
    }
}
