use crate::execution::{CoerceResult, FieldError};
use crate::helpers::{HasDefinitionWrapper, WrappedStruct};
use crate::ruby_api::{
    coerce_input::CoerceInput, coercion_error::CoercionError, root, Directives, RResult,
};
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
    ruby_class: RClass,
    internal_representation_sorbet_type_name: String,
}

// TODO: add ability to coerce input and possibly coerce result

impl CustomScalarTypeDefinition {
    fn new(kw: RHash) -> Result<Self, Error> {
        let args: KwArgs<_, (), ()> = get_kwargs(
            kw,
            &[
                "name",
                "description",
                "directives",
                "ruby_class",
                "internal_representation_sorbet_type_name",
            ],
            &[],
        )?;
        let (name, description, directives, ruby_class, internal_representation_sorbet_type_name): (String, Option<String>, RArray, RClass, String) = args.required;
        let directives: Directives = directives.try_into()?;
        Ok(Self {
            name,
            description,
            directives,
            ruby_class,
            internal_representation_sorbet_type_name,
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

    pub(crate) fn internal_representation_sorbet_type_name(&self) -> &str {
        &self.internal_representation_sorbet_type_name
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

impl CoerceResult for CustomScalarTypeDefinition {
    fn coerce_result(&self, value: Value) -> Result<Value, FieldError> {
        let coerced_r_result: WrappedStruct<RResult> = self
            .ruby_class
            .funcall("coerce_result", (value,))
            .map_err(FieldError::ApplicationError)?;

        let coerced_result: Result<Value, String> = coerced_r_result
            .get()
            .try_into()
            .map_err(FieldError::ApplicationError)?;

        coerced_result.map_err(|message| FieldError::CannotCoerceResultToCustomScalar { message })
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
