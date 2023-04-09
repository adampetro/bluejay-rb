use crate::ruby_api::{
    arguments_definition::ArgumentsDefinition,
    output_type_reference::{BaseOutputTypeReference, OutputTypeReference},
    root, Directives,
};
use convert_case::{Case, Casing};
use magnus::{
    function, gc, memoize, method, scan_args::get_kwargs, scan_args::KwArgs, typed_data::Obj,
    value::BoxValue, DataTypeFunctions, Error, Module, Object, RArray, RHash, RString, TypedData,
    Value,
};

#[derive(Debug, TypedData)]
#[magnus(class = "Bluejay::FieldDefinition", mark)]
pub struct FieldDefinition {
    name: String,
    description: Option<String>,
    arguments_definition: ArgumentsDefinition,
    r#type: Obj<OutputTypeReference>,
    directives: Directives,
    is_builtin: bool,
    ruby_resolver_method_name: String,
    name_r_string: RString,
}

impl FieldDefinition {
    pub fn new(kw: RHash) -> Result<Self, Error> {
        let args: KwArgs<_, _, ()> = get_kwargs(
            kw,
            &["name", "type"],
            &["argument_definitions", "description", "directives"],
        )?;
        let (name_r_string, r#type): (RString, Obj<OutputTypeReference>) = args.required;
        let (argument_definitions, description, directives): (
            Option<RArray>,
            Option<Option<String>>,
            Option<RArray>,
        ) = args.optional;
        name_r_string.freeze();
        let name = name_r_string.to_string()?;
        let arguments_definition =
            ArgumentsDefinition::new(argument_definitions.unwrap_or_else(RArray::new))?;
        let description = description.unwrap_or_default();
        let directives = directives.try_into()?;
        let ruby_resolver_method_name = format!("resolve_{}", name.to_case(Case::Snake));
        Ok(Self {
            name,
            description,
            arguments_definition,
            r#type,
            directives,
            is_builtin: false,
            ruby_resolver_method_name,
            name_r_string,
        })
    }

    pub(crate) fn typename() -> Obj<Self> {
        memoize!(([BoxValue<Value>; 4], Obj<FieldDefinition>): {
            let t = Obj::wrap(OutputTypeReference::Base(BaseOutputTypeReference::builtin_string(), true));
            let arguments_definition = ArgumentsDefinition::empty();
            let directives = Directives::empty();
            let directives_rarray: RArray = (&directives).into();
            let name_r_string = RString::new("__typename");
            name_r_string.freeze();
            let fd = Self {
                name: "__typename".to_string(),
                description: None,
                arguments_definition,
                r#type: t,
                directives,
                is_builtin: true,
                ruby_resolver_method_name: "resolve_typename".to_string(),
                name_r_string,
            };
            let obj = Obj::wrap(fd);
            ([BoxValue::new(*obj), BoxValue::new(*arguments_definition), BoxValue::new(*t), BoxValue::new(*directives_rarray)], obj)
        }).1
    }

    pub(crate) fn ruby_resolver_method_name(&self) -> &str {
        self.ruby_resolver_method_name.as_str()
    }

    pub(crate) fn name_r_string(&self) -> RString {
        self.name_r_string
    }

    pub fn directives(&self) -> &Directives {
        &self.directives
    }
}

impl DataTypeFunctions for FieldDefinition {
    fn mark(&self) {
        gc::mark(self.arguments_definition);
        gc::mark(self.r#type);
        self.directives.mark();
        gc::mark(self.name_r_string);
    }
}

impl FieldDefinition {
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn argument_definitions(&self) -> &ArgumentsDefinition {
        &self.arguments_definition
    }

    pub fn r#type(&self) -> &OutputTypeReference {
        self.r#type.get()
    }
}

impl bluejay_core::definition::FieldDefinition for FieldDefinition {
    type ArgumentsDefinition = ArgumentsDefinition;
    type OutputTypeReference = OutputTypeReference;
    type Directives = Directives;

    fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn arguments_definition(&self) -> Option<&Self::ArgumentsDefinition> {
        Some(&self.arguments_definition)
    }

    fn r#type(&self) -> &Self::OutputTypeReference {
        self.r#type.get()
    }

    fn is_builtin(&self) -> bool {
        self.is_builtin
    }

    fn directives(&self) -> Option<&Self::Directives> {
        Some(&self.directives)
    }
}

pub fn init() -> Result<(), Error> {
    let class = root().define_class("FieldDefinition", Default::default())?;

    class.define_singleton_method("new", function!(FieldDefinition::new, 1))?;
    class.define_method("name", method!(FieldDefinition::name, 0))?;
    class.define_method(
        "resolver_method_name",
        method!(FieldDefinition::ruby_resolver_method_name, 0),
    )?;
    class.define_method(
        "argument_definitions",
        method!(
            |fd: &FieldDefinition| -> RArray { (*fd.argument_definitions()).into() },
            0
        ),
    )?;
    class.define_method("type", method!(|fd: &FieldDefinition| fd.r#type, 0))?;
    class.define_method(
        "directives",
        method!(
            |fd: &FieldDefinition| -> RArray { (&fd.directives).into() },
            0
        ),
    )?;

    Ok(())
}
