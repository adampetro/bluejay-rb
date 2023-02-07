use crate::helpers::HasDefinitionWrapper;
use crate::ruby_api::{coerce_input::CoerceInput, coercion_error::CoercionError, root, Directives};
use magnus::{
    function, memoize, scan_args::get_kwargs, scan_args::KwArgs, DataTypeFunctions, Error, Module,
    Object, RArray, RClass, RHash, TypedData, Value,
};

#[derive(Debug, TypedData)]
#[magnus(class = "Bluejay::CustomScalarTypeDefinition", mark)]
pub struct CustomScalarTypeDefinition {
    name: String,
    description: Option<String>,
    directives: Directives,
}

// TODO: add ability to coerce input and possibly coerce result

impl CustomScalarTypeDefinition {
    fn new(kw: RHash) -> Result<Self, Error> {
        let args: KwArgs<(String, Option<String>, RArray), (), ()> =
            get_kwargs(kw, &["name", "description", "directives"], &[])?;
        let (name, description, directives) = args.required;
        let directives: Directives = directives.try_into()?;
        Ok(Self {
            name,
            description,
            directives,
        })
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn directives(&self) -> &Directives {
        &self.directives
    }
}

impl DataTypeFunctions for CustomScalarTypeDefinition {
    fn mark(&self) {
        self.directives.mark();
    }
}

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
    type Directives = Directives;

    fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn directives(&self) -> Option<&Self::Directives> {
        Some(&self.directives)
    }
}

pub fn init() -> Result<(), Error> {
    let class = root().define_class("CustomScalarTypeDefinition", Default::default())?;

    class.define_singleton_method("new", function!(CustomScalarTypeDefinition::new, 1))?;

    Ok(())
}
