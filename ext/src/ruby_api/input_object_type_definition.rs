use crate::helpers::{
    public_name, rhash_with_capacity, HasDefinitionWrapper, NewInstanceKw, Variables, Warden,
};
use crate::ruby_api::{
    base, introspection, root, wrapped_value::value_inner_from_ruby_const_value, CoerceInput,
    CoercionError, Directives, InputFieldsDefinition, RResult, WrappedValue,
};
use crate::visibility_scoped::{ScopedInputObjectTypeDefinition, VisibilityCache};
use bluejay_core::{definition::prelude::*, AsIter};
use bluejay_parser::ast::Value as ParserValue;
use bluejay_validator::Path;
use magnus::{
    function, gc, memoize, method, r_hash::ForEach, scan_args::get_kwargs, scan_args::KwArgs,
    DataTypeFunctions, Error, Module, Object, RArray, RClass, RHash, RModule, TypedData, Value,
    QNIL,
};

#[derive(Debug, TypedData)]
#[magnus(class = "Bluejay::InputObjectTypeDefinition", mark)]
pub struct InputObjectTypeDefinition {
    name: String,
    description: Option<String>,
    input_fields_definition: InputFieldsDefinition,
    directives: Directives,
    ruby_class: RClass,
}

impl InputObjectTypeDefinition {
    fn new(kw: RHash) -> Result<Self, Error> {
        let args: KwArgs<_, (), ()> = get_kwargs(
            kw,
            &[
                "name",
                "input_field_definitions",
                "description",
                "directives",
                "ruby_class",
            ],
            &[],
        )?;
        let (name, input_field_definitions, description, directives, ruby_class): (
            String,
            RArray,
            Option<String>,
            RArray,
            RClass,
        ) = args.required;
        let input_fields_definition = InputFieldsDefinition::new(input_field_definitions)?;
        let directives = directives.try_into()?;
        Ok(Self {
            name,
            description,
            input_fields_definition,
            directives,
            ruby_class,
        })
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn input_fields_definition(&self) -> &InputFieldsDefinition {
        &self.input_fields_definition
    }

    pub fn directives(&self) -> &Directives {
        &self.directives
    }
}

impl DataTypeFunctions for InputObjectTypeDefinition {
    fn mark(&self) {
        gc::mark(self.input_fields_definition);
        gc::mark(self.ruby_class);
        self.directives.mark();
    }
}

impl HasDefinitionWrapper for InputObjectTypeDefinition {
    fn required_module() -> RModule {
        *memoize!(RModule: base().define_module("InputObjectType").unwrap())
    }
}

impl bluejay_core::definition::InputObjectTypeDefinition for InputObjectTypeDefinition {
    type InputFieldsDefinition = InputFieldsDefinition;
    type Directives = Directives;

    fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn input_field_definitions(&self) -> &Self::InputFieldsDefinition {
        &self.input_fields_definition
    }

    fn directives(&self) -> Option<&Self::Directives> {
        self.directives.to_option()
    }
}

impl<'a> CoerceInput for ScopedInputObjectTypeDefinition<'a> {
    fn coerced_ruby_value_to_wrapped_value(
        &self,
        value: Value,
        path: Path,
    ) -> Result<Result<WrappedValue, Vec<CoercionError>>, Error> {
        if let Some(hash) = RHash::from_value(value) {
            let args = rhash_with_capacity(self.input_field_definitions().len());
            let mut errors = Vec::new();

            for ivd in self.input_field_definitions().iter() {
                let key = ivd.inner().ruby_name();
                let value = hash.get(ivd.inner().name_r_string());
                let required = ivd.is_required();
                let default_value = ivd.inner().default_value();
                if required && value.is_none() {
                    errors.push(CoercionError::new(
                        format!("No value for required field {}", ivd.name()),
                        path.to_vec(),
                    ));
                } else {
                    match default_value {
                        Some(default_value) if value.is_none() => {
                            args.aset(key, default_value.to_value()).unwrap();
                        }
                        _ => {
                            let inner_path = path.push(ivd.name());
                            match ivd.r#type().coerced_ruby_value_to_wrapped_value(
                                value.unwrap_or(*QNIL),
                                inner_path,
                            )? {
                                Ok(coerced_value) => {
                                    args.aset(key, coerced_value.to_value()).unwrap();
                                }
                                Err(errs) => {
                                    errors.extend(errs);
                                }
                            }
                        }
                    }
                }
            }

            hash.foreach(|key: String, _: Value| {
                if self
                    .input_field_definitions()
                    .iter()
                    .all(|ivd| ivd.name() != key)
                {
                    errors.push(CoercionError::new(
                        format!("No field named `{}` on {}", key, self.name()),
                        path.to_vec(),
                    ))
                }

                Ok(ForEach::Continue)
            })?;

            if errors.is_empty() {
                let r_value = self.inner().ruby_class.new_instance_kw(args)?;

                let inner = value_inner_from_ruby_const_value(value)?;

                Ok(Ok((r_value, inner).into()))
            } else {
                Ok(Err(errors))
            }
        } else {
            Ok(Err(vec![CoercionError::new(
                format!(
                    "No implicit conversion of {} to {}",
                    public_name(value),
                    self.name()
                ),
                path.to_vec(),
            )]))
        }
    }

    fn coerce_parser_value<const CONST: bool>(
        &self,
        value: &ParserValue<CONST>,
        path: Path,
        variables: &impl Variables<CONST>,
    ) -> Result<Result<Value, Vec<CoercionError>>, Error> {
        if let ParserValue::Object(o) = value {
            let args = rhash_with_capacity(self.input_field_definitions().len());
            let mut errors = Vec::new();

            for ivd in self.input_field_definitions().iter() {
                let key = ivd.inner().ruby_name();
                let value = o
                    .iter()
                    .find(|(name, _)| ivd.name() == name.as_str())
                    .map(|(_, value)| value);
                let required = ivd.is_required();
                let default_value = ivd.inner().default_value();

                match (value, default_value) {
                    (None, None) => {
                        if required {
                            errors.push(CoercionError::new(
                                format!("No value for required field {}", ivd.name()),
                                path.to_vec(),
                            ));
                        }
                    }
                    (None, Some(default_value)) => {
                        args.aset(key, default_value.to_value()).unwrap();
                    }
                    (Some(value), _) => {
                        let inner_path = path.push(ivd.name());
                        match ivd
                            .r#type()
                            .coerce_parser_value(value, inner_path, variables)?
                        {
                            Ok(coerced_value) => {
                                args.aset(key, coerced_value).unwrap();
                            }
                            Err(errs) => errors.extend(errs),
                        }
                    }
                }
            }

            errors.extend(o.iter().filter_map(|(key, _)| {
                let key = key.as_ref();
                if !self
                    .input_field_definitions()
                    .iter()
                    .any(|ivd| ivd.name() == key)
                {
                    Some(CoercionError::new(
                        format!("No field named `{}` on {}", key, self.name()),
                        path.to_vec(),
                    ))
                } else {
                    None
                }
            }));

            if errors.is_empty() {
                self.inner().ruby_class.new_instance_kw(args).map(Ok)
            } else {
                Ok(Err(errors))
            }
        } else {
            Ok(Err(vec![CoercionError::new(
                format!("No implicit conversion of {} to {}", value, self.name(),),
                path.to_vec(),
            )]))
        }
    }

    fn coerce_ruby_const_value(
        &self,
        value: Value,
        path: Path,
    ) -> Result<Result<Value, Vec<CoercionError>>, Error> {
        if let Some(hash) = RHash::from_value(value) {
            let args = rhash_with_capacity(self.input_field_definitions().len());
            let mut errors = Vec::new();

            for ivd in self.input_field_definitions().iter() {
                let key = ivd.inner().ruby_name();
                let value = hash.get(ivd.inner().name_r_string());
                let required = ivd.is_required();
                let default_value = ivd.inner().default_value();
                if required && value.is_none() {
                    errors.push(CoercionError::new(
                        format!("No value for required field {}", ivd.name()),
                        path.to_vec(),
                    ));
                } else {
                    match default_value {
                        Some(default_value) if value.is_none() => {
                            args.aset(key, default_value.to_value()).unwrap();
                        }
                        _ => {
                            let inner_path = path.push(ivd.name());
                            match ivd.r#type().coerced_ruby_value_to_wrapped_value(
                                value.unwrap_or(*QNIL),
                                inner_path,
                            )? {
                                Ok(coerced_value) => {
                                    args.aset(key, coerced_value.to_value()).unwrap();
                                }
                                Err(errs) => {
                                    errors.extend(errs);
                                }
                            }
                        }
                    }
                }
            }

            hash.foreach(|key: String, _: Value| {
                if self
                    .input_field_definitions()
                    .iter()
                    .all(|ivd| ivd.name() != key)
                {
                    errors.push(CoercionError::new(
                        format!("No field named `{}` on {}", key, self.name()),
                        path.to_vec(),
                    ))
                }

                Ok(ForEach::Continue)
            })?;

            if errors.is_empty() {
                self.inner().ruby_class.new_instance_kw(args).map(Ok)
            } else {
                Ok(Err(errors))
            }
        } else {
            Ok(Err(vec![CoercionError::new(
                format!(
                    "No implicit conversion of {} to {}",
                    public_name(value),
                    self.name()
                ),
                path.to_vec(),
            )]))
        }
    }
}

impl introspection::Type for InputObjectTypeDefinition {
    type OfType = introspection::Never;

    fn description(&self) -> Option<&str> {
        self.description()
    }

    fn input_fields(&self) -> Option<InputFieldsDefinition> {
        Some(self.input_fields_definition)
    }

    fn kind(&self) -> introspection::TypeKind {
        introspection::TypeKind::InputObject
    }

    fn name(&self) -> Option<&str> {
        Some(&self.name)
    }
}

pub fn init() -> Result<(), Error> {
    let class = root().define_class("InputObjectTypeDefinition", Default::default())?;

    class.define_singleton_method("new", function!(InputObjectTypeDefinition::new, 1))?;
    class.define_method(
        "coerce_input",
        method!(
            |itd: &InputObjectTypeDefinition,
             input: Value,
             context: Value|
             -> Result<RResult, Error> {
                let visibility_cache = VisibilityCache::new(Warden::new(context));
                let scoped_definition =
                    ScopedInputObjectTypeDefinition::new(itd, &visibility_cache);
                scoped_definition
                    .coerce_ruby_const_value(input, Default::default())
                    .map(|result| {
                        result
                            .map_err(|errors| {
                                let arr = RArray::from_iter(errors);
                                let _ = arr.len();
                                arr
                            })
                            .into()
                    })
            },
            2
        ),
    )?;
    class.define_method(
        "input_field_definitions",
        method!(
            |itd: &InputObjectTypeDefinition| RArray::from(*itd.input_fields_definition()),
            0
        ),
    )?;
    introspection::implement_type!(InputObjectTypeDefinition, class);

    Ok(())
}
