use crate::helpers::NewInstanceKw;
use crate::ruby_api::{root, DirectiveDefinition, Directives};
use bluejay_core::{definition::EnumValueDefinition as CoreEnumValueDefinition, AsIter};
use magnus::{
    function, method, scan_args::get_kwargs, scan_args::KwArgs, DataTypeFunctions, Error, Module,
    Object, RArray, RHash, TypedData,
};

#[derive(Debug, TypedData)]
#[magnus(class = "Bluejay::EnumValueDefinition", mark)]
pub struct EnumValueDefinition {
    name: String,
    description: Option<String>,
    directives: Directives,
    deprecation_reason: Option<String>,
}

impl EnumValueDefinition {
    pub fn new(kw: RHash) -> Result<Self, Error> {
        let args: KwArgs<(String,), _, ()> = get_kwargs(
            kw,
            &["name"],
            &["description", "directives", "deprecation_reason"],
        )?;
        let (name,) = args.required;
        let (description, directives, deprecation_reason): (
            Option<Option<String>>,
            Option<RArray>,
            Option<Option<String>>,
        ) = args.optional;
        let deprecation_reason = deprecation_reason.flatten();
        let directives = directives.unwrap_or_else(RArray::new);
        if let Some(deprecation_reason) = deprecation_reason.as_deref() {
            let directive_definition = DirectiveDefinition::deprecated();
            let args = RHash::from_iter([(
                directive_definition
                    .as_ref()
                    .arguments_definition()
                    .iter()
                    .next()
                    .unwrap()
                    .ruby_name(),
                deprecation_reason,
            )]);
            directives.push(directive_definition.wrapper().new_instance_kw(args)?)?;
        }
        let directives = directives.try_into()?;
        Ok(Self {
            name,
            description: description.flatten(),
            directives,
            deprecation_reason,
        })
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn directives(&self) -> &Directives {
        &self.directives
    }

    pub fn deprecation_reason(&self) -> Option<&str> {
        self.deprecation_reason.as_deref()
    }

    pub fn is_deprecated(&self) -> bool {
        self.deprecation_reason.is_some()
    }
}

impl DataTypeFunctions for EnumValueDefinition {
    fn mark(&self) {
        self.directives.mark();
    }
}

impl CoreEnumValueDefinition for EnumValueDefinition {
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

pub fn init() -> Result<(), Error> {
    let class = root().define_class("EnumValueDefinition", Default::default())?;

    class.define_singleton_method("new", function!(EnumValueDefinition::new, 1))?;
    class.define_method("name", method!(EnumValueDefinition::name, 0))?;
    class.define_method(
        "description",
        method!(
            <EnumValueDefinition as CoreEnumValueDefinition>::description,
            0
        ),
    )?;
    class.define_method(
        "deprecated?",
        method!(EnumValueDefinition::is_deprecated, 0),
    )?;
    class.define_method(
        "deprecation_reason",
        method!(EnumValueDefinition::deprecation_reason, 0),
    )?;
    class.define_method(
        "resolve_typename",
        method!(|_: &EnumValueDefinition| "__EnumValue", 0),
    )?;

    Ok(())
}
