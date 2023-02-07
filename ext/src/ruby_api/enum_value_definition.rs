use crate::ruby_api::{root, Directives};
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
}

impl EnumValueDefinition {
    pub fn new(kw: RHash) -> Result<Self, Error> {
        let args: KwArgs<(String,), (Option<String>, Option<RArray>), ()> =
            get_kwargs(kw, &["name"], &["description", "directives"])?;
        let (name,) = args.required;
        let (description, directives) = args.optional;
        let directives = directives.try_into()?;
        Ok(Self {
            name,
            description,
            directives,
        })
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn directives(&self) -> &Directives {
        &self.directives
    }
}

impl DataTypeFunctions for EnumValueDefinition {
    fn mark(&self) {
        self.directives.mark();
    }
}

impl bluejay_core::definition::EnumValueDefinition for EnumValueDefinition {
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
    let class = root().define_class("EnumValueDefinition", Default::default())?;

    class.define_singleton_method("new", function!(EnumValueDefinition::new, 1))?;
    class.define_method("name", method!(EnumValueDefinition::name, 0))?;

    Ok(())
}
