use bluejay_core::BuiltinScalarDefinition;
use std::borrow::Cow;

#[derive(Debug)]
pub enum FieldError {
    ReturnedNullForNonNullType,
    ReturnedNonListForListType,
    CannotCoerceResultToBuiltinScalar {
        builtin_scalar: BuiltinScalarDefinition,
    },
    CannotCoerceResultToEnumType,
    CannotCoerceResultToCustomScalar {
        message: String,
    },
    ApplicationError(String),
}

impl FieldError {
    pub fn message(&self) -> Cow<'_, str> {
        match self {
            Self::ReturnedNullForNonNullType => "Cannot return null for non-nullable field".into(),
            Self::ReturnedNonListForListType => "Cannot return non-list for list field".into(),
            Self::CannotCoerceResultToBuiltinScalar { builtin_scalar } => format!(
                "Cannot coerce result to builtin scalar `{}`",
                builtin_scalar.name()
            )
            .into(),
            Self::CannotCoerceResultToEnumType => "Cannot coerce result to enum type".into(),
            Self::CannotCoerceResultToCustomScalar { message } => message.as_str().into(),
            Self::ApplicationError(message) => format!("Application error: {}", message).into(),
        }
    }
}
