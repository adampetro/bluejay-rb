use super::root;
use crate::helpers::WrappedStruct;
use magnus::{
    function, method, scan_args::get_kwargs, scan_args::KwArgs, DataTypeFunctions, Error, Module,
    Object, RHash, TypedData,
};

#[derive(Clone, Debug, TypedData)]
#[magnus(class = "Bluejay::EnumValueDefinition", mark)]
pub struct EnumValueDefinition {
    name: String,
    description: Option<String>,
}

impl EnumValueDefinition {
    pub fn new(kw: RHash) -> Result<Self, Error> {
        let args: KwArgs<(String,), (Option<String>,), ()> =
            get_kwargs(kw, &["name"], &["description"])?;
        let (name,) = args.required;
        let (description,) = args.optional;
        Ok(Self { name, description })
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl DataTypeFunctions for EnumValueDefinition {}

impl bluejay_core::definition::EnumValueDefinition for EnumValueDefinition {
    fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl bluejay_core::definition::EnumValueDefinition for WrappedStruct<EnumValueDefinition> {
    fn description(&self) -> Option<&str> {
        self.get().description()
    }

    fn name(&self) -> &str {
        self.get().name()
    }
}

pub fn init() -> Result<(), Error> {
    let class = root().define_class("EnumValueDefinition", Default::default())?;

    class.define_singleton_method("new", function!(EnumValueDefinition::new, 1))?;
    class.define_method("name", method!(EnumValueDefinition::name, 0))?;

    Ok(())
}
