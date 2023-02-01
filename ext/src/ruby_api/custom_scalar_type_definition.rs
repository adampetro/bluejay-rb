use super::{coerce_input::CoerceInput, coercion_error::CoercionError, root};
use crate::helpers::HasDefinitionWrapper;
use magnus::{
    function, memoize, scan_args::get_kwargs, DataTypeFunctions, Error, Module, Object, RClass,
    RHash, TypedData, Value,
};

#[derive(Clone, Debug, TypedData)]
#[magnus(class = "Bluejay::CustomScalarTypeDefinition", mark)]
pub struct CustomScalarTypeDefinition {
    name: String,
    description: Option<String>,
}

// TODO: add ability to coerce input and possibly coerce result

impl CustomScalarTypeDefinition {
    fn new(kw: RHash) -> Result<Self, Error> {
        let args = get_kwargs(kw, &["name", "description"], &[])?;
        let (name, description): (String, Option<String>) = args.required;
        let _: () = args.optional;
        let _: () = args.splat;
        Ok(Self { name, description })
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_ref().map(String::as_str)
    }
}

impl DataTypeFunctions for CustomScalarTypeDefinition {}

impl HasDefinitionWrapper for CustomScalarTypeDefinition {
    fn wrapping_class() -> RClass {
        *memoize!(RClass: root().define_class("CustomScalarType", Default::default()).unwrap())
    }
}

impl CoerceInput for CustomScalarTypeDefinition {
    fn coerce_input(
        &self,
        value: Value,
        _path: &[String],
    ) -> Result<Result<Value, Vec<CoercionError>>, Error> {
        Ok(Ok(value))
    }
}

impl bluejay_core::definition::ScalarTypeDefinition for CustomScalarTypeDefinition {
    fn description(&self) -> Option<&str> {
        self.description.as_ref().map(|s| s.as_str())
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }
}

pub fn init() -> Result<(), Error> {
    let class = root().define_class("CustomScalarTypeDefinition", Default::default())?;

    class.define_singleton_method("new", function!(CustomScalarTypeDefinition::new, 1))?;

    Ok(())
}
