use bluejay_core::definition::{TypeDefinitionReference, prelude::*, TypeDefinition};

pub(crate) fn type_description<'a, T: TypeDefinition>(type_def: &'a TypeDefinitionReference<'a, T>) -> Option<&'a str> {
    match type_def {
        TypeDefinitionReference::BuiltinScalar(bs) => None,
        TypeDefinitionReference::CustomScalar(cs) => cs.description(),
        TypeDefinitionReference::Enum(et) => et.description(),
        TypeDefinitionReference::InputObject(iot) => iot.description(),
        TypeDefinitionReference::Interface(it) => it.description(),
        TypeDefinitionReference::Object(ot) => ot.description(),
        TypeDefinitionReference::Union(ut) => ut.description(),
    }.clone()
}

pub(crate) fn type_kind<'a, T: TypeDefinition>(type_def: &'a TypeDefinitionReference<'a, T>) -> &'static str {
    match type_def {
        TypeDefinitionReference::BuiltinScalar(bs) => "Scalar",
        TypeDefinitionReference::CustomScalar(cs) => "Scalar",
        TypeDefinitionReference::Enum(et) => "Enum",
        TypeDefinitionReference::InputObject(iot) => "InputObject",
        TypeDefinitionReference::Interface(it) => "Interface",
        TypeDefinitionReference::Object(ot) => "Object",
        TypeDefinitionReference::Union(ut) => "Union",
    }
}
