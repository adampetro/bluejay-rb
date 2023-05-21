use bluejay_core::BuiltinScalarDefinition;

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
