use crate::execution::{CoerceResult, FieldError};
use crate::helpers::{value_from_core_value, HasDefinitionWrapper, Variables};
use crate::ruby_api::{
    introspection, root, wrapped_value::value_inner_from_ruby_const_value, CoerceInput,
    CoercionError, DirectiveDefinition, Directives, RResult, WrappedValue,
};
use bluejay_parser::ast::Value as ParserValue;
use magnus::{
    function, memoize, scan_args::get_kwargs, scan_args::KwArgs, typed_data::Obj,
    DataTypeFunctions, Error, Module, Object, RArray, RClass, RHash, TypedData, Value,
};

#[derive(Debug, TypedData)]
#[magnus(class = "Bluejay::CustomScalarTypeDefinition", mark)]
pub struct CustomScalarTypeDefinition {
    name: String,
    description: Option<String>,
    directives: Directives,
    specified_by_url: Option<String>,
    ruby_class: RClass,
    internal_representation_sorbet_type_name: String,
}

impl CustomScalarTypeDefinition {
    fn new(kw: RHash) -> Result<Self, Error> {
        let args: KwArgs<_, (), ()> = get_kwargs(
            kw,
            &[
                "name",
                "description",
                "directives",
                "specified_by_url",
                "ruby_class",
                "internal_representation_sorbet_type_name",
            ],
            &[],
        )?;
        let (
            name,
            description,
            directives,
            specified_by_url,
            ruby_class,
            internal_representation_sorbet_type_name,
        ): (
            String,
            Option<String>,
            RArray,
            Option<String>,
            RClass,
            String,
        ) = args.required;
        if let Some(specified_by_url) = specified_by_url.as_deref() {
            directives.push(
                DirectiveDefinition::specified_by()
                    .class()
                    .new_instance((specified_by_url,))?,
            )?;
        }
        let directives: Directives = directives.try_into()?;
        Ok(Self {
            name,
            description,
            directives,
            specified_by_url,
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

    pub fn specified_by_url(&self) -> Option<&str> {
        self.specified_by_url.as_deref()
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
    fn coerced_ruby_value_to_wrapped_value(
        &self,
        value: Value,
        path: &[String],
    ) -> Result<Result<WrappedValue, Vec<CoercionError>>, Error> {
        let inner = value_inner_from_ruby_const_value(value)?;

        let coerced_r_result: Obj<RResult> = self.ruby_class.funcall("coerce_input", (value,))?;

        let coerced_result: Result<Value, String> = coerced_r_result.get().try_into()?;

        Ok(coerced_result
            .map(|coerced_value| WrappedValue::from((coerced_value, inner)))
            .map_err(|message| vec![CoercionError::new(message, path.to_vec())]))
    }

    fn coerce_parser_value<const CONST: bool>(
        &self,
        value: &ParserValue<CONST>,
        path: &[String],
        variables: &impl Variables<CONST>,
    ) -> Result<Result<Value, Vec<CoercionError>>, Error> {
        let r_value = value_from_core_value(value, variables);

        let coerced_r_result: Obj<RResult> = self.ruby_class.funcall("coerce_input", (r_value,))?;

        let coerced_result: Result<Value, String> = coerced_r_result.get().try_into()?;

        Ok(coerced_result.map_err(|message| vec![CoercionError::new(message, path.to_vec())]))
    }

    fn coerce_ruby_const_value(
        &self,
        value: Value,
        path: &[String],
    ) -> Result<Result<Value, Vec<CoercionError>>, Error> {
        let coerced_r_result: Obj<RResult> = self.ruby_class.funcall("coerce_input", (value,))?;

        let coerced_result: Result<Value, String> = coerced_r_result.get().try_into()?;

        Ok(coerced_result.map_err(|message| vec![CoercionError::new(message, path.to_vec())]))
    }
}

impl CoerceResult for CustomScalarTypeDefinition {
    fn coerce_result(&self, value: Value) -> Result<Value, FieldError> {
        let coerced_r_result: Obj<RResult> = self
            .ruby_class
            .funcall("coerce_result", (value,))
            .map_err(|error| FieldError::ApplicationError(error.to_string()))?;

        let coerced_result: Result<Value, String> = coerced_r_result
            .get()
            .try_into()
            .map_err(|error: Error| FieldError::ApplicationError(error.to_string()))?;

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
        self.directives.to_option()
    }
}

impl introspection::Type for CustomScalarTypeDefinition {
    type OfType = introspection::Never;

    fn kind(&self) -> introspection::TypeKind {
        introspection::TypeKind::Scalar
    }

    fn description(&self) -> Option<&str> {
        self.description()
    }

    fn name(&self) -> Option<&str> {
        Some(&self.name)
    }

    fn specified_by_url(&self) -> Option<&str> {
        self.specified_by_url()
    }
}

pub fn init() -> Result<(), Error> {
    let class = root().define_class("CustomScalarTypeDefinition", Default::default())?;

    class.define_singleton_method("new", function!(CustomScalarTypeDefinition::new, 1))?;
    introspection::implement_type!(CustomScalarTypeDefinition, class);

    Ok(())
}
