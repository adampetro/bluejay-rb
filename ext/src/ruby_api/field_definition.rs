use crate::ruby_api::{root, ArgumentsDefinition, Directives, OutputType};
use convert_case::{Case, Casing};
use magnus::{
    function, gc, method, scan_args::get_kwargs, scan_args::KwArgs, typed_data::Obj,
    DataTypeFunctions, Error, Module, Object, RArray, RHash, RString, TypedData, QNIL,
};

#[derive(Debug, TypedData)]
#[magnus(class = "Bluejay::FieldDefinition", mark)]
pub struct FieldDefinition {
    name: String,
    description: Option<String>,
    arguments_definition: ArgumentsDefinition,
    r#type: Obj<OutputType>,
    directives: Directives,
    is_builtin: bool,
    ruby_resolver_method_name: String,
    name_r_string: RString,
    extra_resolver_args: Vec<ExtraResolverArg>,
}

impl FieldDefinition {
    pub fn new(kw: RHash) -> Result<Self, Error> {
        let args: KwArgs<_, _, ()> = get_kwargs(
            kw,
            &["name", "type"],
            &[
                "argument_definitions",
                "description",
                "directives",
                "resolver_method_name",
            ],
        )?;
        let (name_r_string, r#type): (RString, Obj<OutputType>) = args.required;
        type OptionalArgs = (
            Option<RArray>,
            Option<Option<String>>,
            Option<RArray>,
            Option<Option<String>>,
        );
        let (argument_definitions, description, directives, resolver_method_name): OptionalArgs =
            args.optional;
        name_r_string.freeze();
        let name = name_r_string.to_string()?;
        let arguments_definition =
            ArgumentsDefinition::new(argument_definitions.unwrap_or_else(RArray::new))?;
        let description = description.unwrap_or_default();
        let directives = directives.try_into()?;
        let is_builtin = name.starts_with("__");
        let ruby_resolver_method_name = resolver_method_name
            .and_then(|r| r)
            .unwrap_or_else(|| name.to_case(Case::Snake));
        let extra_resolver_args = match name.as_str() {
            "__schema" | "__type" => vec![ExtraResolverArg::SchemaDefinition],
            _ => vec![],
        };
        Ok(Self {
            name,
            description,
            arguments_definition,
            r#type,
            directives,
            is_builtin,
            ruby_resolver_method_name,
            name_r_string,
            extra_resolver_args,
        })
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

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn argument_definitions(&self) -> &ArgumentsDefinition {
        &self.arguments_definition
    }

    pub fn r#type(&self) -> &OutputType {
        self.r#type.get()
    }

    pub fn extra_resolver_args(&self) -> &[ExtraResolverArg] {
        &self.extra_resolver_args
    }

    pub fn resolver_arg_count(&self) -> usize {
        self.arguments_definition.len() + self.extra_resolver_args.len()
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

impl bluejay_core::definition::FieldDefinition for FieldDefinition {
    type ArgumentsDefinition = ArgumentsDefinition;
    type OutputType = OutputType;
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

    fn r#type(&self) -> &Self::OutputType {
        self.r#type.get()
    }

    fn is_builtin(&self) -> bool {
        self.is_builtin
    }

    fn directives(&self) -> Option<&Self::Directives> {
        Some(&self.directives)
    }
}

#[derive(Debug)]
pub enum ExtraResolverArg {
    SchemaDefinition,
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
    class.define_method(
        "args",
        method!(
            |fd: &FieldDefinition| RArray::from(fd.arguments_definition),
            0
        ),
    )?;
    class.define_method("deprecated?", method!(|_: &FieldDefinition| false, 0))?;
    class.define_method("deprecation_reason", method!(|_: &FieldDefinition| QNIL, 0))?;
    class.define_method("description", method!(FieldDefinition::description, 0))?;

    Ok(())
}
