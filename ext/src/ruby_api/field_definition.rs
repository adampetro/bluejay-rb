use crate::helpers::NewInstanceKw;
use crate::ruby_api::{
    root, ArgumentsDefinition, DirectiveDefinition, Directives, OutputType, ResolverStrategy,
};
use bluejay_core::AsIter;
use convert_case::{Case, Casing};
use magnus::{
    exception, function, gc, memoize, method,
    scan_args::{get_kwargs, KwArgs},
    typed_data::Obj,
    DataTypeFunctions, Error, Module, Object, RArray, RHash, RString, Symbol, TypedData, Value,
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
    resolver_strategy: ResolverStrategy,
    ruby_resolver_method_name: String,
    name_r_string: RString,
    extra_resolver_args: Vec<ExtraResolverArg>,
    deprecation_reason: Option<String>,
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
                "resolver_strategy",
            ],
        )?;
        let (name, r#type): (Value, Obj<OutputType>) = args.required;
        type OptionalArgs = (
            Option<RArray>,
            Option<Option<String>>,
            Option<RArray>,
            Option<Option<Symbol>>,
            Option<Option<String>>,
            Option<Obj<ResolverStrategy>>,
        );
        let (
            argument_definitions,
            description,
            directives,
            resolver_method_name,
            deprecation_reason,
            resolver_strategy,
        ): OptionalArgs = args.optional;
        let resolver_method_name = resolver_method_name.flatten().map(|s| s.to_string());
        let (name_r_string, name, ruby_resolver_method_name) =
            if let Some(r_string) = RString::from_value(name) {
                let name = r_string.to_string()?;
                (
                    r_string,
                    r_string.to_string()?,
                    resolver_method_name.unwrap_or_else(|| name.to_case(Case::Snake)),
                )
            } else if let Some(symbol) = Symbol::from_value(name) {
                let symbol_str = symbol.to_string();
                let name = symbol_str.to_case(Case::Camel);
                let resolver_method_name = resolver_method_name.unwrap_or(symbol_str);
                (RString::new(&name), name, resolver_method_name)
            } else {
                return Err(Error::new(
                    exception::arg_error(),
                    "Must provide a `String` or `Symbol` for `name`",
                ));
            };

        name_r_string.freeze();
        let arguments_definition =
            ArgumentsDefinition::new(argument_definitions.unwrap_or_else(RArray::new))?;
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
            directives.push(directive_definition.class().new_instance_kw(args)?)?;
        }
        let directives = directives.try_into()?;
        let is_builtin = name.starts_with("__");
        let extra_resolver_args = match name.as_str() {
            "__schema" | "__type" => vec![ExtraResolverArg::SchemaClass],
            _ => vec![],
        };
        let resolver_strategy = resolver_strategy.map(|obj| *obj.get()).unwrap_or_default();
        Ok(Self {
            name,
            description,
            arguments_definition,
            r#type,
            directives,
            is_builtin,
            resolver_strategy,
            ruby_resolver_method_name,
            name_r_string,
            extra_resolver_args,
            deprecation_reason,
        })
    }

    pub(crate) fn resolver_strategy(&self) -> ResolverStrategy {
        self.resolver_strategy
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
        "resolver_strategy",
        method!(FieldDefinition::resolver_strategy, 0),
    )?;
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
