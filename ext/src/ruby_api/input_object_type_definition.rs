use crate::helpers::{public_name, HasDefinitionWrapper};
use crate::ruby_api::{
    coerce_input::CoerceInput, coercion_error::CoercionError,
    input_fields_definition::InputFieldsDefinition, r_result::RResult, root, Directives,
};
use bluejay_core::AsIter;
use magnus::{
    function, gc, memoize, method, scan_args::get_kwargs, scan_args::KwArgs, DataTypeFunctions,
    Error, Module, Object, RArray, RClass, RHash, TypedData, Value, QNIL,
};
use std::collections::HashSet;

#[derive(Debug, TypedData)]
#[magnus(class = "Bluejay::InputObjectTypeDefinition", mark)]
pub struct InputObjectTypeDefinition {
    name: String,
    description: Option<String>,
    input_fields_definition: InputFieldsDefinition,
    input_value_definition_names: HashSet<String>,
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
        let input_value_definition_names = HashSet::from_iter(
            input_fields_definition
                .iter()
                .map(|ivd| ivd.name().to_owned()),
        );
        let directives = directives.try_into()?;
        Ok(Self {
            name,
            description,
            input_fields_definition,
            input_value_definition_names,
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
    fn wrapping_class() -> RClass {
        *memoize!(RClass: root().define_class("InputType", Default::default()).unwrap())
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
        Some(&self.directives)
    }
}

impl CoerceInput for InputObjectTypeDefinition {
    fn coerce_input(
        &self,
        value: Value,
        path: &[String],
    ) -> Result<Result<Value, Vec<CoercionError>>, Error> {
        if let Some(hash) = RHash::from_value(value) {
            let args = RArray::new();
            let mut errors = Vec::new();

            for ivd in self.input_fields_definition.iter() {
                let value = hash.get(ivd.name());
                let required = ivd.is_required();
                let default_value = ivd.default_value();
                if required && value.is_none() {
                    errors.push(CoercionError::new(
                        format!("No value for required field {}", ivd.name()),
                        path.to_owned(),
                    ));
                } else {
                    match default_value {
                        Some(default_value) if value.is_none() => {
                            args.push(default_value).unwrap();
                        }
                        _ => {
                            let mut inner_path = path.to_owned();
                            inner_path.push(ivd.name().to_owned());
                            match ivd.coerce_input(value.unwrap_or(*QNIL), &inner_path)? {
                                Ok(coerced_value) => {
                                    args.push(coerced_value).unwrap();
                                }
                                Err(errs) => {
                                    errors.extend(errs);
                                }
                            }
                        }
                    }
                }
            }

            let keys: Vec<String> = hash.check_funcall("keys", ()).unwrap()?;

            errors.extend(keys.iter().filter_map(|key| {
                if !self.input_value_definition_names.contains(key) {
                    Some(CoercionError::new(
                        format!("No field named `{}` on {}", key, self.name),
                        path.to_owned(),
                    ))
                } else {
                    None
                }
            }));

            if errors.is_empty() {
                self.ruby_class
                    .new_instance(unsafe { args.as_slice() })
                    .map(Ok)
            } else {
                Ok(Err(errors))
            }
        } else if value.is_kind_of(self.ruby_class) {
            // TODO: this is kind of a hack for when a coerced variable value is nested in an uncoerced input
            // see if there is a less hacky way to do this
            Ok(Ok(value))
        } else {
            Ok(Err(vec![CoercionError::new(
                format!(
                    "No implicit conversion of {} to {}",
                    public_name(value),
                    self.name
                ),
                path.to_owned(),
            )]))
        }
    }
}

pub fn init() -> Result<(), Error> {
    let class = root().define_class("InputObjectTypeDefinition", Default::default())?;

    class.define_singleton_method("new", function!(InputObjectTypeDefinition::new, 1))?;
    class.define_method(
        "coerce_input",
        method!(
            |itd: &InputObjectTypeDefinition, input: Value| -> Result<RResult, Error> {
                itd.coerce_input(input, &[]).map(Into::into)
            },
            1
        ),
    )?;
    class.define_method(
        "input_field_definitions",
        method!(
            |itd: &InputObjectTypeDefinition| -> RArray { (*itd.input_fields_definition()).into() },
            0
        ),
    )?;

    Ok(())
}
