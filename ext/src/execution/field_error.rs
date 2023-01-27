use bluejay_core::BuiltinScalarDefinition;

pub enum FieldError {
    ReturnedNullForNonNullType,
    ReturnedNonListForListType,
    CannotCoerceResultToBuiltinScalar { builtin_scalar: BuiltinScalarDefinition },
    CannotCoerceResultToEnumType,
}
