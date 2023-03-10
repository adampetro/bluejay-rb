use crate::helpers::HasDefinitionWrapper;
use crate::ruby_api::{
    field_definition::FieldDefinition, fields_definition::FieldsDefinition,
    interface_implementations::InterfaceImplementations,
    interface_type_definition::InterfaceTypeDefinition, root, Directives,
};
use bluejay_core::AsIter;
use magnus::{
    function, gc, memoize, method, scan_args::get_kwargs, scan_args::KwArgs, DataTypeFunctions,
    Error, Module, Object, RArray, RClass, RHash, TypedData,
};

#[derive(Debug, TypedData)]
#[magnus(class = "Bluejay::ObjectTypeDefinition", mark)]
pub struct ObjectTypeDefinition {
    name: String,
    description: Option<String>,
    fields_definition: FieldsDefinition,
    directives: Directives,
    interface_implementations: InterfaceImplementations,
}

impl ObjectTypeDefinition {
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
        field_definitions.push(FieldDefinition::typename())?;
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
}

impl DataTypeFunctions for ObjectTypeDefinition {
    fn mark(&self) {
        gc::mark(self.fields_definition);
        gc::mark(self.interface_implementations);
        self.directives.mark();
    }
}

impl HasDefinitionWrapper for ObjectTypeDefinition {
    fn wrapping_class() -> RClass {
        *memoize!(RClass: root().define_class("ObjectType", Default::default()).unwrap())
    }
}

impl ObjectTypeDefinition {
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

    pub fn implements_interface(&self, interface: &InterfaceTypeDefinition) -> bool {
        self.interface_implementations
            .iter()
            .any(|ii| ii.interface().get().name() == interface.name())
    }

    pub fn field_definition(&self, name: &str) -> Option<&FieldDefinition> {
        self.fields_definition.iter().find(|fd| fd.name() == name)
    }

    pub fn directives(&self) -> &Directives {
        &self.directives
    }
}

impl bluejay_core::definition::ObjectTypeDefinition for ObjectTypeDefinition {
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
        Some(&self.directives)
    }
}

pub fn init() -> Result<(), Error> {
    let class = root().define_class("ObjectTypeDefinition", Default::default())?;

    class.define_singleton_method("new", function!(ObjectTypeDefinition::new, 1))?;
    class.define_method("name", method!(ObjectTypeDefinition::name, 0))?;
    class.define_method(
        "field_definitions",
        method!(
            |otd: &ObjectTypeDefinition| -> RArray { (*otd.fields_definition()).into() },
            0
        ),
    )?;

    Ok(())
}
