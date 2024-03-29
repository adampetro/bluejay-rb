use crate::helpers::{HasDefinitionWrapper, WrappedDefinition};
use crate::ruby_api::{
    base, root, ArgumentsDefinition, DirectiveLocation, HasVisibility, Visibility,
};
use bluejay_core::definition::{
    DirectiveDefinition as CoreDirectiveDefinition, DirectiveLocation as CoreDirectiveLocation,
};
use magnus::{
    function, gc, memoize, method, scan_args::get_kwargs, scan_args::KwArgs, DataTypeFunctions,
    Error, Module, Object, RArray, RClass, RHash, RModule, TypedData,
};

#[derive(Debug, TypedData)]
#[magnus(class = "Bluejay::DirectiveDefinition", mark)]
pub struct DirectiveDefinition {
    name: String,
    description: Option<String>,
    arguments_definition: ArgumentsDefinition,
    is_repeatable: bool,
    locations: Vec<CoreDirectiveLocation>,
    ruby_class: RClass,
    is_builtin: bool,
    visibility: Option<Visibility>,
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
                "locations",
                "ruby_class",
                "visibility",
            ],
            &[],
        )?;
        type RequiredArgs = (
            String,
            RArray,
            Option<String>,
            bool,
            RArray,
            RClass,
            Option<Visibility>,
        );
        let (
            name,
            argument_definitions,
            description,
            is_repeatable,
            locations,
            ruby_class,
            visibility,
        ): RequiredArgs = args.required;
        let arguments_definition = ArgumentsDefinition::new(argument_definitions)?;
        let locations: Result<Vec<CoreDirectiveLocation>, Error> = locations
            .each()
            .map(|el| {
                el.and_then(|val| {
                    let directive_location: &DirectiveLocation = val.try_convert()?;
                    Ok(CoreDirectiveLocation::from(directive_location))
                })
            })
            .collect();
        let is_builtin = unsafe { ruby_class.name() }.starts_with("Bluejay::Builtin::Directives");
        Ok(Self {
            name,
            description,
            arguments_definition,
            is_repeatable,
            locations: locations?,
            ruby_class,
            is_builtin,
            visibility,
        })
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn arguments_definition(&self) -> ArgumentsDefinition {
        self.arguments_definition
    }

    pub fn ruby_class(&self) -> RClass {
        self.ruby_class
    }

    pub fn builtin_directive_definitions() -> &'static [WrappedDefinition<Self>] {
        memoize!([WrappedDefinition<DirectiveDefinition>; 4]: ["Skip", "Include", "Deprecated", "SpecifiedBy"].map(
            |builtin_directive_base_name| -> WrappedDefinition<DirectiveDefinition> {
                root()
                    .const_get::<_, RModule>("Builtin")
                    .unwrap()
                    .const_get::<_, RModule>("Directives")
                    .unwrap()
                    .const_get(builtin_directive_base_name)
                    .unwrap()
            },
        ))
    }

    pub fn deprecated() -> WrappedDefinition<Self> {
        Self::builtin_directive_definitions()[2].clone()
    }

    pub fn specified_by() -> WrappedDefinition<Self> {
        Self::builtin_directive_definitions()[3].clone()
    }
}

impl DataTypeFunctions for DirectiveDefinition {
    fn mark(&self) {
        gc::mark(self.arguments_definition);
        gc::mark(self.ruby_class);
        self.visibility.as_ref().map(Visibility::mark);
    }
}

impl HasDefinitionWrapper for DirectiveDefinition {
    type Wrapper = RClass;

    fn required_module() -> RModule {
        *memoize!(RModule: base().define_module("Directive").unwrap())
    }
}

impl CoreDirectiveDefinition for DirectiveDefinition {
    type ArgumentsDefinition = ArgumentsDefinition;
    type DirectiveLocations = Vec<CoreDirectiveLocation>;

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

    fn is_builtin(&self) -> bool {
        self.is_builtin
    }
}

impl HasVisibility for DirectiveDefinition {
    fn visibility(&self) -> Option<&Visibility> {
        self.visibility.as_ref()
    }
}

pub fn init() -> Result<(), Error> {
    let class = root().define_class("DirectiveDefinition", Default::default())?;

    class.define_singleton_method("new", function!(DirectiveDefinition::new, 1))?;
    class.define_method(
        "argument_definitions",
        method!(DirectiveDefinition::arguments_definition, 0),
    )?;
    class.define_method("name", method!(DirectiveDefinition::name, 0))?;
    class.define_method(
        "description",
        method!(
            <DirectiveDefinition as CoreDirectiveDefinition>::description,
            0
        ),
    )?;
    class.define_method(
        "locations",
        method!(
            |dd: &DirectiveDefinition| RArray::from_iter(dd.locations.iter().map(AsRef::as_ref)),
            0
        ),
    )?;
    class.define_method(
        "args",
        method!(DirectiveDefinition::arguments_definition, 0),
    )?;
    class.define_method(
        "repeatable?",
        method!(
            <DirectiveDefinition as CoreDirectiveDefinition>::is_repeatable,
            0
        ),
    )?;
    class.define_method(
        "resolve_typename",
        method!(|_: &DirectiveDefinition| "__Directive", 0),
    )?;

    Ok(())
}
