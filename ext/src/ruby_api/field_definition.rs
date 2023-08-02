use crate::helpers::NewInstanceKw;
use crate::ruby_api::{
    root, ArgumentsDefinition, DirectiveDefinition, Directives, HasVisibility, OutputType,
    Visibility,
};
use convert_case::{Case, Casing};
use magnus::{
    function, gc, memoize, method,
    scan_args::{get_kwargs, KwArgs},
    typed_data::Obj,
    value::Id,
    DataTypeFunctions, Error, Module, Object, RArray, RHash, RString, Symbol, TypedData,
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
    ruby_resolver_method_id: Id,
    name_r_string: RString,
    extra_resolver_args: Vec<ExtraResolverArg>,
    deprecation_reason: Option<String>,
    visibility: Option<Visibility>,
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
                "deprecation_reason",
                "visibility",
            ],
        )?;
        let (name_r_string, r#type): (RString, Obj<OutputType>) = args.required;
        type OptionalArgs = (
            Option<ArgumentsDefinition>,
            Option<Option<String>>,
            Option<RArray>,
            Option<Option<String>>,
            Option<Option<String>>,
            Option<Option<Visibility>>,
        );
        let (
            argument_definitions,
            description,
            directives,
            resolver_method_name,
            deprecation_reason,
            visibility,
        ): OptionalArgs = args.optional;
        name_r_string.freeze();
        let name = name_r_string.to_string()?;
        let arguments_definition = argument_definitions.unwrap_or_default();
        let description = description.unwrap_or_default();
        let directives = directives.unwrap_or_else(RArray::new);
        let deprecation_reason = deprecation_reason.flatten();
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
        let is_builtin = name.starts_with("__");
        let ruby_resolver_method_name = resolver_method_name
            .flatten()
            .unwrap_or_else(|| name.to_case(Case::Snake));
        let ruby_resolver_method_id = Id::new(ruby_resolver_method_name.as_str());
        let extra_resolver_args = match name.as_str() {
            "__schema" | "__type" => vec![ExtraResolverArg::SchemaClass],
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
            ruby_resolver_method_id,
            name_r_string,
            extra_resolver_args,
            deprecation_reason,
            visibility: visibility.flatten(),
        })
    }

    pub(crate) fn ruby_resolver_method_name(&self) -> &str {
        self.ruby_resolver_method_name.as_str()
    }

    pub(crate) fn ruby_resolver_method_id(&self) -> Id {
        self.ruby_resolver_method_id
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

    pub fn is_deprecated(&self) -> bool {
        self.deprecation_reason.is_some()
    }

    pub fn deprecation_reason(&self) -> Option<&str> {
        self.deprecation_reason.as_deref()
    }
}

impl DataTypeFunctions for FieldDefinition {
    fn mark(&self) {
        gc::mark(self.arguments_definition);
        gc::mark(self.r#type);
        self.directives.mark();
        gc::mark(self.name_r_string);
        self.visibility.as_ref().map(Visibility::mark);
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
        self.directives.to_option()
    }
}

impl HasVisibility for FieldDefinition {
    fn visibility(&self) -> Option<&Visibility> {
        self.visibility.as_ref()
    }
}

#[derive(Debug)]
pub enum ExtraResolverArg {
    SchemaClass,
}

impl ExtraResolverArg {
    pub(crate) fn kwarg_name(&self) -> Symbol {
        match self {
            Self::SchemaClass => *memoize!(Symbol: Symbol::new("schema_class")),
        }
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
    class.define_method(
        "args",
        method!(
            |fd: &FieldDefinition| RArray::from(fd.arguments_definition),
            0
        ),
    )?;
    class.define_method("deprecated?", method!(FieldDefinition::is_deprecated, 0))?;
    class.define_method(
        "deprecation_reason",
        method!(FieldDefinition::deprecation_reason, 0),
    )?;
    class.define_method("description", method!(FieldDefinition::description, 0))?;
    class.define_method(
        "resolve_typename",
        method!(|_: &FieldDefinition| "__Field", 0),
    )?;

    Ok(())
}
