use bluejay_core::BuiltinScalarDefinition;
use magnus::Error;

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
    ApplicationError(Error),
}
