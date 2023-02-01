use crate::execution::FieldError;
use bluejay_core::BuiltinScalarDefinition;
use magnus::{Float, Integer, Value};

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
                // TODO: handle BigDecimal and possibly Numeric
                if matches!(Float::from_value(value), Some(f) if f.to_f64().is_finite()) {
                    Ok(value)
                } else if value.is_kind_of(magnus::class::integer()) {
                    Ok(value.funcall("to_f", ()).unwrap())
                } else {
                    Err(FieldError::CannotCoerceResultToBuiltinScalar {
                        builtin_scalar: *self,
                    })
                }
            }
            Self::ID => {
                if value.is_kind_of(magnus::class::string()) {
                    Ok(value)
                } else if value.is_kind_of(magnus::class::integer()) {
                    Ok(value.funcall("to_s", ()).unwrap())
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
