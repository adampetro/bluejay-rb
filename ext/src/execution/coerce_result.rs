use crate::execution::FieldError;
use bluejay_core::BuiltinScalarDefinition;
use magnus::{Integer, TryConvert, Value};

pub trait CoerceResult {
    fn coerce_result(&self, value: Value) -> Result<Value, FieldError>;
}

impl CoerceResult for BuiltinScalarDefinition {
    fn coerce_result(&self, value: Value) -> Result<Value, FieldError> {
        match self {
            Self::Boolean => {
                if value.is_kind_of(magnus::class::true_class())
                    || value.is_kind_of(magnus::class::false_class())
                {
                    Ok(value)
                } else {
                    Err(FieldError::CannotCoerceResultToBuiltinScalar {
                        builtin_scalar: *self,
                    })
                }
            }
            Self::Float => {
                if matches!(f64::try_convert(value), Ok(f) if f.is_finite()) {
                    Ok(value)
                } else {
                    Err(FieldError::CannotCoerceResultToBuiltinScalar {
                        builtin_scalar: *self,
                    })
                }
            }
            Self::ID => {
                if value.is_kind_of(magnus::class::string())
                    || matches!(Integer::from_value(value), Some(i) if i.to_i32().is_ok())
                {
                    Ok(value)
                } else {
                    Err(FieldError::CannotCoerceResultToBuiltinScalar {
                        builtin_scalar: *self,
                    })
                }
            }
            Self::Int => {
                if matches!(Integer::from_value(value), Some(i) if i.to_i32().is_ok()) {
                    Ok(value)
                } else {
                    Err(FieldError::CannotCoerceResultToBuiltinScalar {
                        builtin_scalar: *self,
                    })
                }
            }
            Self::String => {
                if value.is_kind_of(magnus::class::string()) {
                    Ok(value)
                } else {
                    Err(FieldError::CannotCoerceResultToBuiltinScalar {
                        builtin_scalar: *self,
                    })
                }
            }
        }
    }
}
