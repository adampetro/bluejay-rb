use crate::helpers::HasDefinitionWrapper;
use crate::ruby_api::{
    base, introspection, root, Directives, FieldDefinition, FieldsDefinition, HasVisibility,
    InterfaceImplementations, Visibility,
};
use crate::visibility_scoped::{ScopedInterfaceTypeDefinition, ScopedObjectTypeDefinition};
use bluejay_core::{definition::prelude::*, AsIter};
use magnus::{
    function, gc, memoize, method, scan_args::get_kwargs, scan_args::KwArgs, DataTypeFunctions,
    Error, Module, Object, RArray, RClass, RHash, RModule, TypedData, Value,
};

#[derive(Debug, TypedData)]
#[magnus(class = "Bluejay::ObjectTypeDefinition", mark)]
pub struct ObjectTypeDefinition {
    name: String,
    description: Option<String>,
    fields_definition: FieldsDefinition,
    directives: Directives,
    interface_implementations: InterfaceImplementations,
    is_builtin: bool,
    visibility: Option<Visibility>,
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
                "ruby_class",
                "visibility",
            ],
            &[],
        )?;
        let (
            name,
            fields_definition,
            interface_implementations,
            description,
            directives,
            ruby_class,
            visibility,
        ): (
            String,
            FieldsDefinition,
            InterfaceImplementations,
            Option<String>,
            RArray,
            RClass,
            Option<Visibility>,
        ) = args.required;
        let directives = directives.try_into()?;
        let is_builtin = unsafe { ruby_class.name() }.starts_with("Bluejay::Builtin::ObjectTypes");
        Ok(Self {
            name,
            description,
            fields_definition,
            directives,
            interface_implementations,
            is_builtin,
            visibility,
        })
    }
}

impl DataTypeFunctions for ObjectTypeDefinition {
    fn mark(&self) {
        gc::mark(self.fields_definition);
        gc::mark(self.interface_implementations);
        self.directives.mark();
        self.visibility.as_ref().map(Visibility::mark);
    }
}

impl HasDefinitionWrapper for ObjectTypeDefinition {
    type Wrapper = Value;

    fn required_module() -> RModule {
        *memoize!(RModule: base().define_module("ObjectType").unwrap())
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

    pub fn implements_interface(
        scoped_self: &ScopedObjectTypeDefinition,
        interface: &ScopedInterfaceTypeDefinition,
    ) -> bool {
        scoped_self
            .interface_implementations()
            .map_or(false, |interface_implementations| {
                interface_implementations
                    .iter()
                    .any(|ii| ii.interface().name() == interface.name())
            })
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
        self.directives.to_option()
    }

    fn is_builtin(&self) -> bool {
        self.is_builtin
    }
}

impl introspection::Type for ObjectTypeDefinition {
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
        introspection::TypeKind::Object
    }

    fn name(&self) -> Option<&str> {
        Some(&self.name)
    }
}

impl HasVisibility for ObjectTypeDefinition {
    fn visibility(&self) -> Option<&Visibility> {
        self.visibility.as_ref()
    }
}

pub fn init() -> Result<(), Error> {
    let class = root().define_class("ObjectTypeDefinition", Default::default())?;

    class.define_singleton_method("new", function!(ObjectTypeDefinition::new, 1))?;
    class.define_method(
        "field_definitions",
        method!(
            |otd: &ObjectTypeDefinition| RArray::from(*otd.fields_definition()),
            0
        ),
    )?;
    introspection::implement_type!(ObjectTypeDefinition, class);

    Ok(())
}
