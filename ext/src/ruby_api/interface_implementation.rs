use crate::helpers::WrappedDefinition;
use crate::ruby_api::{
    interface_type_definition::InterfaceTypeDefinition, root, HasVisibility, Visibility,
};
use magnus::{
    function, method,
    scan_args::{get_kwargs, KwArgs},
    typed_data::Obj,
    DataTypeFunctions, Error, Module, Object, RClass, RHash, TypedData,
};

#[derive(Debug, TypedData)]
#[magnus(class = "Bluejay::InterfaceImplementation", mark)]
pub struct InterfaceImplementation {
    interface: WrappedDefinition<InterfaceTypeDefinition>,
    visibility: Option<Visibility>,
}

impl InterfaceImplementation {
    fn new(kw: RHash) -> Result<Self, Error> {
        let args: KwArgs<(RClass,), (Option<Option<Visibility>>,), ()> =
            get_kwargs(kw, &["interface"], &["visibility"])?;
        let (interface,) = args.required;
        let (visibility,) = args.optional;
        WrappedDefinition::new(interface).map(|interface| Self {
            interface,
            visibility: visibility.flatten(),
        })
    }

    pub fn interface(&self) -> Obj<InterfaceTypeDefinition> {
        *self.interface.get()
    }
}

impl DataTypeFunctions for InterfaceImplementation {
    fn mark(&self) {
        self.interface.mark();
        self.visibility.as_ref().map(Visibility::mark);
    }
}

impl bluejay_core::definition::InterfaceImplementation for InterfaceImplementation {
    type InterfaceTypeDefinition = InterfaceTypeDefinition;

    fn interface(&self) -> &Self::InterfaceTypeDefinition {
        self.interface.as_ref()
    }
}

impl HasVisibility for InterfaceImplementation {
    fn visibility(&self) -> Option<&Visibility> {
        self.visibility.as_ref()
    }
}

pub fn init() -> Result<(), Error> {
    let class = root().define_class("InterfaceImplementation", Default::default())?;

    class.define_singleton_method("new", function!(InterfaceImplementation::new, 1))?;
    class.define_method(
        "interface",
        method!(|ii: &InterfaceImplementation| ii.interface.class(), 0),
    )?;

    Ok(())
}
