use super::root;
use super::{input_value_definition::InputValueDefinition, output_type_reference::{OutputTypeReference, BaseOutputTypeReference}, arguments_definition::ArgumentsDefinition};
use crate::helpers::{WrappedStruct};
use magnus::{function, Error, Module, Object, scan_args::get_kwargs, RHash, RArray, Value, TypedData, DataTypeFunctions, method, memoize, value::BoxValue};
use bluejay_core::BuiltinScalarDefinition;

#[derive(Clone, Debug, TypedData)]
#[magnus(class = "Bluejay::FieldDefinition", mark)]
pub struct FieldDefinition {
    name: String,
    description: Option<String>,
    arguments_definition: ArgumentsDefinition,
    r#type: WrappedStruct<OutputTypeReference>,
    is_builtin: bool,
}

impl FieldDefinition {
    pub fn new(kw: RHash) -> Result<Self, Error> {
        let args = get_kwargs(kw, &["name", "type"], &["argument_definitions", "description"])?;
        let (name, r#type): (String, WrappedStruct<OutputTypeReference>) = args.required;
        let (argument_definitions, description): (Option<RArray>, Option<Option<String>>) = args.optional;
        let _: () = args.splat;
        let arguments_definition = ArgumentsDefinition::new(argument_definitions)?;
        let description = description.unwrap_or_default();
        Ok(Self { name, description, arguments_definition, r#type, is_builtin: false })
    }

    pub(crate) fn typename() -> WrappedStruct<Self> {
        memoize!((BoxValue<Value>, BoxValue<Value>, WrappedStruct<FieldDefinition>): {
            let t = WrappedStruct::wrap(OutputTypeReference::Base(BaseOutputTypeReference::BuiltinScalarType(BuiltinScalarDefinition::String), true));
            let fd = Self {
                name: "__typename".to_string(),
                description: None,
                arguments_definition: ArgumentsDefinition::empty(),
                r#type: t,
                is_builtin: true,
            };
            let ws = WrappedStruct::wrap(fd);
            (BoxValue::new(ws.to_value()), BoxValue::new(t.to_value()), ws)
        }).2
    }
}

impl DataTypeFunctions for FieldDefinition {
    fn mark(&self) {
        self.arguments_definition.mark();
        self.r#type.mark();
    }
}

impl FieldDefinition {
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_ref().map(String::as_str)
    }

    pub fn argument_definitions(&self) -> &[WrappedStruct<InputValueDefinition>] {
        self.arguments_definition.as_ref()
    }

    pub fn r#type(&self) -> &OutputTypeReference {
        self.r#type.get()
    }
}

impl bluejay_core::definition::FieldDefinition for FieldDefinition {
    type ArgumentsDefinition = ArgumentsDefinition;
    type OutputTypeReference = OutputTypeReference;

    fn description(&self) -> Option<&str> {
        self.description.as_ref().map(String::as_str)
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn arguments_definition(&self) -> &Self::ArgumentsDefinition {
        &self.arguments_definition
    }

    fn r#type(&self) -> &Self::OutputTypeReference {
        self.r#type.get()
    }

    fn is_builtin(&self) -> bool {
        self.is_builtin
    }
}

pub fn init() -> Result<(), Error> {
    let class = root().define_class("FieldDefinition", Default::default())?;

    class.define_singleton_method("new", function!(FieldDefinition::new, 1))?;

    Ok(())
}
