use super::{arguments_definition::ArgumentsDefinition, root};
use crate::helpers::HasDefinitionWrapper;
use bluejay_core::definition::{DirectiveDefinition as CoreDirectiveDefinition, DirectiveLocation};
use magnus::{
    function, gc, memoize, method, scan_args::get_kwargs, scan_args::KwArgs, DataTypeFunctions,
    Error, Module, Object, RArray, RClass, RHash, TypedData,
};

#[derive(Debug, TypedData)]
#[magnus(class = "Bluejay::DirectiveDefinition", mark)]
pub struct DirectiveDefinition {
    name: String,
    description: Option<String>,
    arguments_definition: ArgumentsDefinition,
    is_repeatable: bool,
    locations: Vec<DirectiveLocation>,
    ruby_class: RClass,
}

impl DirectiveDefinition {
    fn new(kw: RHash) -> Result<Self, Error> {
        let args: KwArgs<_, (), ()> = get_kwargs(
            kw,
            &[
                "name",
                "argument_definitions",
                "description",
                "is_repeatable",
                "ruby_class",
            ],
            &[],
        )?;
        let (name, argument_definitions, description, is_repeatable, ruby_class): (
            String,
            RArray,
            Option<String>,
            bool,
            RClass,
        ) = args.required;
        let arguments_definition = ArgumentsDefinition::new(argument_definitions)?;
        Ok(Self {
            name,
            description,
            arguments_definition,
            is_repeatable,
            locations: Vec::new(),
            ruby_class,
        })
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn arguments_definition(&self) -> &ArgumentsDefinition {
        &self.arguments_definition
    }
}

impl DataTypeFunctions for DirectiveDefinition {
    fn mark(&self) {
        gc::mark(self.arguments_definition);
        gc::mark(self.ruby_class);
    }
}

impl HasDefinitionWrapper for DirectiveDefinition {
    fn wrapping_class() -> RClass {
        *memoize!(RClass: root().define_class("Directive", Default::default()).unwrap())
    }
}

impl CoreDirectiveDefinition for DirectiveDefinition {
    type ArgumentsDefinition = ArgumentsDefinition;
    type DirectiveLocations = Vec<DirectiveLocation>;

    fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn arguments_definition(&self) -> Option<&Self::ArgumentsDefinition> {
        Some(&self.arguments_definition)
    }

    fn is_repeatable(&self) -> bool {
        self.is_repeatable
    }

    fn locations(&self) -> &Self::DirectiveLocations {
        &self.locations
    }
}

pub fn init() -> Result<(), Error> {
    let class = root().define_class("DirectiveDefinition", Default::default())?;

    class.define_singleton_method("new", function!(DirectiveDefinition::new, 1))?;
    class.define_method(
        "argument_definitions",
        method!(
            |dd: &DirectiveDefinition| -> RArray { (*dd.arguments_definition()).into() },
            0
        ),
    )?;

    Ok(())
}
