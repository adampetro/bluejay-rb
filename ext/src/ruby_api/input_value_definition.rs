use crate::helpers::WrappedStruct;
use crate::ruby_api::{
    coerce_input::CoerceInput,
    coercion_error::CoercionError,
    input_type_reference::InputTypeReference,
    root,
    value::{Value, ValueInner},
    Directives,
};
use convert_case::{Case, Casing};
use magnus::{
    function, method, scan_args::get_kwargs, scan_args::KwArgs, DataTypeFunctions, Error, Module,
    Object, RArray, RHash, TypedData, Value as RValue,
};

#[derive(Debug, TypedData)]
#[magnus(class = "Bluejay::InputValueDefinition", mark)]
pub struct InputValueDefinition {
    name: String,
    description: Option<String>,
    r#type: WrappedStruct<InputTypeReference>,
    directives: Directives,
    default_value: Option<Value>,
    ruby_name: String,
}

impl InputValueDefinition {
    pub fn new(kw: RHash) -> Result<Self, Error> {
        let args: KwArgs<_, _, ()> = get_kwargs(
            kw,
            &["name", "type"],
            &["description", "directives", "default_value"],
        )?;
        let (name, r#type): (String, WrappedStruct<InputTypeReference>) = args.required;
        let (description, directives, default_value): (
            Option<Option<String>>,
            Option<RArray>,
            Option<Value>,
        ) = args.optional;
        let description = description.unwrap_or_default();
        let directives = directives.try_into()?;
        let ruby_name = name.to_case(Case::Snake);
        Ok(Self {
            name,
            description,
            r#type,
            directives,
            default_value,
            ruby_name,
        })
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn r#type(&self) -> &InputTypeReference {
        self.r#type.get()
    }

    pub fn default_value(&self) -> Option<RValue> {
        None
    }

    pub fn is_required(&self) -> bool {
        if self.default_value.is_some() {
            false
        } else {
            self.r#type.get().is_required()
        }
    }

    pub(crate) fn ruby_name(&self) -> &str {
        self.ruby_name.as_str()
    }

    pub fn directives(&self) -> &Directives {
        &self.directives
    }
}

impl DataTypeFunctions for InputValueDefinition {
    fn mark(&self) {
        self.r#type.mark();
        self.directives.mark();
    }
}

impl CoerceInput for InputValueDefinition {
    fn coerce_input(
        &self,
        value: RValue,
        path: &[String],
    ) -> Result<Result<RValue, Vec<CoercionError>>, Error> {
        self.r#type.get().coerce_input(value, path)
    }
}

impl bluejay_core::definition::InputValueDefinition for InputValueDefinition {
    type InputTypeReference = InputTypeReference;
    type Value = ValueInner;
    type Directives = Directives;

    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    fn r#type(&self) -> &Self::InputTypeReference {
        self.r#type.get()
    }

    fn default_value(&self) -> Option<&Self::Value> {
        self.default_value.as_ref().map(AsRef::as_ref)
    }

    fn directives(&self) -> Option<&Self::Directives> {
        Some(&self.directives)
    }
}

pub fn init() -> Result<(), Error> {
    let class = root().define_class("InputValueDefinition", Default::default())?;

    class.define_singleton_method("new", function!(InputValueDefinition::new, 1))?;
    class.define_method("name", method!(InputValueDefinition::name, 0))?;
    class.define_method("type", method!(|ivd: &InputValueDefinition| ivd.r#type, 0))?;
    class.define_method("ruby_name", method!(InputValueDefinition::ruby_name, 0))?;

    Ok(())
}
