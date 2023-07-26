use crate::execution::{CoerceResult, FieldError};
use crate::helpers::{value_from_core_value, HasDefinitionWrapper, NewInstanceKw, Variables};
use crate::ruby_api::{
    base, introspection, root, wrapped_value::value_inner_from_ruby_const_value, CoerceInput,
    CoercionError, DirectiveDefinition, Directives, RResult, WrappedValue,
};
use crate::visibility_scoped::ScopedScalarTypeDefinition;
use bluejay_core::AsIter;
use bluejay_parser::ast::Value as ParserValue;
use bluejay_validator::Path;
use magnus::{
    function, memoize, scan_args::get_kwargs, scan_args::KwArgs, typed_data::Obj, value::Id,
    DataTypeFunctions, Error, ExceptionClass, Module, Object, RArray, RClass, RHash, RModule,
    TypedData, Value,
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
    input_coercion_method_signature: CoercionMethodSignature,
    result_coercion_method_signature: CoercionMethodSignature,
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
                "input_coercion_method_signature",
                "result_coercion_method_signature",
            ],
            &[],
        )?;
        type RequiredArgs = (
            String,
            Option<String>,
            RArray,
            Option<String>,
            RClass,
            String,
            Obj<CoercionMethodSignature>,
            Obj<CoercionMethodSignature>,
        );
        let (
            name,
            description,
            directives,
            specified_by_url,
            ruby_class,
            internal_representation_sorbet_type_name,
            input_coercion_method_signature,
            result_coercion_method_signature,
        ): RequiredArgs = args.required;
        if let Some(specified_by_url) = specified_by_url.as_deref() {
            let directive_definition = DirectiveDefinition::specified_by();
            let args = RHash::from_iter([(
                directive_definition
                    .as_ref()
                    .arguments_definition()
                    .iter()
                    .next()
                    .unwrap()
                    .ruby_name(),
                specified_by_url,
            )]);
            directives.push(directive_definition.class()?.new_instance_kw(args)?)?;
        }
        let directives: Directives = directives.try_into()?;
        Ok(Self {
            name,
            description,
            directives,
            specified_by_url,
            ruby_class,
            internal_representation_sorbet_type_name,
            input_coercion_method_signature: input_coercion_method_signature.get().clone(),
            result_coercion_method_signature: result_coercion_method_signature.get().clone(),
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

    fn coerce_input(
        &self,
        value: Value,
        path: Path,
    ) -> Result<Result<Value, Vec<CoercionError>>, Error> {
        match self
            .ruby_class
            .funcall::<_, _, Value>(*memoize!(Id: Id::new("coerce_input")), (value,))
        {
            Ok(value) => match self.input_coercion_method_signature {
                CoercionMethodSignature::Result => {
                    let coerced_r_result: Obj<RResult> = value.try_convert()?;

                    let coerced_result: Result<Value, String> =
                        coerced_r_result.get().try_into()?;

                    Ok(coerced_result
                        .map_err(|message| vec![CoercionError::new(message, path.to_vec())]))
                }
                CoercionMethodSignature::Exception(_) => Ok(Ok(value)),
            },
            Err(error) if matches!(self.input_coercion_method_signature, CoercionMethodSignature::Exception(cls) if error.is_kind_of(cls)) =>
            {
                // TODO: ensure proper message formatting
                Ok(Err(vec![CoercionError::new(
                    error.to_string(),
                    path.to_vec(),
                )]))
            }
            Err(error) => Err(error),
        }
    }
}

impl DataTypeFunctions for CustomScalarTypeDefinition {
    fn mark(&self) {
        self.directives.mark();
    }
}

impl HasDefinitionWrapper for CustomScalarTypeDefinition {
    fn required_module() -> RModule {
        *memoize!(RModule: base().define_module("CustomScalarType").unwrap())
    }
}

impl<'a> CoerceInput for ScopedScalarTypeDefinition<'a> {
    fn coerced_ruby_value_to_wrapped_value(
        &self,
        value: Value,
        path: Path,
    ) -> Result<Result<WrappedValue, Vec<CoercionError>>, Error> {
        let inner = value_inner_from_ruby_const_value(value)?;

        self.inner()
            .coerce_input(value, path)
            .map(|r| r.map(|coerced_value| WrappedValue::from((coerced_value, inner))))
    }

    fn coerce_parser_value<const CONST: bool>(
        &self,
        value: &ParserValue<CONST>,
        path: Path,
        variables: &impl Variables<CONST>,
    ) -> Result<Result<Value, Vec<CoercionError>>, Error> {
        let r_value = value_from_core_value(value, variables);

        self.inner().coerce_input(r_value, path)
    }

    fn coerce_ruby_const_value(
        &self,
        value: Value,
        path: Path,
    ) -> Result<Result<Value, Vec<CoercionError>>, Error> {
        self.inner().coerce_input(value, path)
    }
}

impl<'a> CoerceResult for ScopedScalarTypeDefinition<'a> {
    fn coerce_result(&self, value: Value) -> Result<Value, FieldError> {
        match self
            .inner()
            .ruby_class
            .funcall::<_, _, Value>(*memoize!(Id: Id::new("coerce_result")), (value,))
        {
            Ok(value) => match self.inner().result_coercion_method_signature {
                CoercionMethodSignature::Result => {
                    let coerced_result: Result<Value, String> = value
                        .try_convert()
                        .and_then(|r_result: Obj<RResult>| Result::try_from(r_result.get()))
                        .map_err(|error| FieldError::ApplicationError(error.to_string()))?;

                    coerced_result
                        .map_err(|message| FieldError::CannotCoerceResultToCustomScalar { message })
                }
                CoercionMethodSignature::Exception(_) => Ok(value),
            },
            Err(error) if matches!(self.inner().result_coercion_method_signature, CoercionMethodSignature::Exception(cls) if error.is_kind_of(cls)) =>
            {
                // TODO: ensure proper message formatting
                Err(FieldError::CannotCoerceResultToCustomScalar {
                    message: error.to_string(),
                })
            }
            Err(error) => Err(FieldError::ApplicationError(error.to_string())),
        }
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

#[derive(Clone, Debug)]
#[magnus::wrap(class = "Bluejay::CustomScalarTypeDefinition::CoercionMethodSignature")]
enum CoercionMethodSignature {
    Result,
    Exception(ExceptionClass),
}

impl CoercionMethodSignature {
    fn exception(exception_class: ExceptionClass) -> Self {
        Self::Exception(exception_class)
    }
}

pub fn init() -> Result<(), Error> {
    let class = root().define_class("CustomScalarTypeDefinition", Default::default())?;

    class.define_singleton_method("new", function!(CustomScalarTypeDefinition::new, 1))?;
    introspection::implement_type!(CustomScalarTypeDefinition, class);

    let coercion_method_signature_class =
        class.define_class("CoercionMethodSignature", Default::default())?;
    coercion_method_signature_class.const_set("Result", CoercionMethodSignature::Result)?;
    coercion_method_signature_class.define_singleton_method(
        "exception",
        function!(CoercionMethodSignature::exception, 1),
    )?;

    Ok(())
}
