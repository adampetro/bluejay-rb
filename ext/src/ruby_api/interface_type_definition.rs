use crate::helpers::HasDefinitionWrapper;
use crate::ruby_api::{
    base, introspection, root, Directives, FieldsDefinition, InterfaceImplementations,
};
use magnus::{
    function, gc, memoize, scan_args::get_kwargs, scan_args::KwArgs, DataTypeFunctions, Error,
    Module, Object, RArray, RHash, RModule, TypedData, Value,
};

#[derive(Debug, TypedData)]
#[magnus(class = "Bluejay::InterfaceTypeDefinition", mark)]
pub struct InterfaceTypeDefinition {
    name: String,
    description: Option<String>,
    fields_definition: FieldsDefinition,
    directives: Directives,
    interface_implementations: InterfaceImplementations,
}

impl InterfaceTypeDefinition {
    fn new(kw: RHash) -> Result<Self, Error> {
        let args: KwArgs<_, (), ()> = get_kwargs(
            kw,
            &[
                "name",
                "field_definitions",
                "interface_implementations",
                "description",
                "directives",
            ],
            &[],
        )?;
        let (name, field_definitions, interface_implementations, description, directives): (
            String,
            RArray,
            RArray,
            Option<String>,
            RArray,
        ) = args.required;
        let fields_definition = FieldsDefinition::new(field_definitions)?;
        let interface_implementations = InterfaceImplementations::new(interface_implementations)?;
        let directives = directives.try_into()?;
        Ok(Self {
            name,
            description,
            fields_definition,
            directives,
            interface_implementations,
        })
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn fields_definition(&self) -> &FieldsDefinition {
        &self.fields_definition
    }

    pub fn interface_implementations(&self) -> &InterfaceImplementations {
        &self.interface_implementations
    }

    pub fn directives(&self) -> &Directives {
        &self.directives
    }
}

impl DataTypeFunctions for InterfaceTypeDefinition {
    fn mark(&self) {
        gc::mark(self.fields_definition);
        gc::mark(self.interface_implementations);
        self.directives.mark();
    }
}

impl HasDefinitionWrapper for InterfaceTypeDefinition {
    type Wrapper = Value;

    fn required_module() -> RModule {
        *memoize!(RModule: base().define_module("InterfaceType").unwrap())
    }
}

impl bluejay_core::definition::InterfaceTypeDefinition for InterfaceTypeDefinition {
    type FieldsDefinition = FieldsDefinition;
    type InterfaceImplementations = InterfaceImplementations;
    type Directives = Directives;

    fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn fields_definition(&self) -> &Self::FieldsDefinition {
        &self.fields_definition
    }

    fn interface_implementations(&self) -> Option<&Self::InterfaceImplementations> {
        Some(&self.interface_implementations)
    }

    fn directives(&self) -> Option<&Self::Directives> {
        self.directives.to_option()
    }
}

impl introspection::Type for InterfaceTypeDefinition {
    type OfType = introspection::Never;

    fn description(&self) -> Option<&str> {
        self.description()
    }

    fn fields(&self) -> Option<FieldsDefinition> {
        Some(self.fields_definition)
    }

    fn interfaces(&self) -> Option<InterfaceImplementations> {
        Some(self.interface_implementations)
    }

    fn kind(&self) -> introspection::TypeKind {
        introspection::TypeKind::Interface
    }

    fn name(&self) -> Option<&str> {
        Some(&self.name)
    }
}

pub fn init() -> Result<(), Error> {
    let class = root().define_class("InterfaceTypeDefinition", Default::default())?;

    class.define_singleton_method("new", function!(InterfaceTypeDefinition::new, 1))?;
    introspection::implement_type!(InterfaceTypeDefinition, class);

    Ok(())
}
