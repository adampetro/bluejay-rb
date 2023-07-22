use crate::helpers::HasDefinitionWrapper;
use crate::ruby_api::{base, introspection, root, Directives, FieldsDefinition, UnionMemberTypes};
use crate::visibility_scoped::{ScopedObjectTypeDefinition, ScopedUnionTypeDefinition};
use bluejay_core::{definition::prelude::*, AsIter};
use magnus::{
    function, gc, memoize, scan_args::get_kwargs, scan_args::KwArgs, DataTypeFunctions, Error,
    Module, Object, RArray, RHash, RModule, TypedData,
};

#[derive(Debug, TypedData)]
#[magnus(class = "Bluejay::UnionTypeDefinition", mark)]
pub struct UnionTypeDefinition {
    name: String,
    description: Option<String>,
    directives: Directives,
    member_types: UnionMemberTypes,
    fields_definition: FieldsDefinition,
}

impl UnionTypeDefinition {
    fn new(kw: RHash) -> Result<Self, Error> {
        let args: KwArgs<_, (), ()> = get_kwargs(
            kw,
            &[
                "name",
                "member_types",
                "description",
                "directives",
                "field_definitions",
            ],
            &[],
        )?;
        let (name, member_types, description, directives, field_definitions): (
            String,
            RArray,
            Option<String>,
            RArray,
            RArray,
        ) = args.required;
        let member_types = UnionMemberTypes::new(member_types)?;
        let directives = directives.try_into()?;
        let fields_definition = FieldsDefinition::new(field_definitions)?;
        Ok(Self {
            name,
            description,
            directives,
            member_types,
            fields_definition,
        })
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn member_types(&self) -> &UnionMemberTypes {
        &self.member_types
    }

    pub fn directives(&self) -> &Directives {
        &self.directives
    }

    pub fn contains_type(
        scoped_self: &ScopedUnionTypeDefinition,
        t: &ScopedObjectTypeDefinition,
    ) -> bool {
        scoped_self
            .union_member_types()
            .iter()
            .any(|mt| mt.member_type().name() == t.name())
    }

    pub fn sorbet_type(&self) -> String {
        format!(
            "T.any({})",
            itertools::join(
                self.member_types
                    .iter()
                    .map(|member_type| member_type.r#type().fully_qualified_name()),
                ", ",
            ),
        )
    }
}

impl DataTypeFunctions for UnionTypeDefinition {
    fn mark(&self) {
        gc::mark(self.member_types);
        self.directives.mark();
        gc::mark(self.fields_definition);
    }
}

impl HasDefinitionWrapper for UnionTypeDefinition {
    fn required_module() -> RModule {
        *memoize!(RModule: base().define_module("UnionType").unwrap())
    }
}

impl bluejay_core::definition::UnionTypeDefinition for UnionTypeDefinition {
    type UnionMemberTypes = UnionMemberTypes;
    type Directives = Directives;
    type FieldsDefinition = FieldsDefinition;

    fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn union_member_types(&self) -> &Self::UnionMemberTypes {
        &self.member_types
    }

    fn directives(&self) -> Option<&Self::Directives> {
        self.directives.to_option()
    }

    fn fields_definition(&self) -> &Self::FieldsDefinition {
        &self.fields_definition
    }
}

impl introspection::Type for UnionTypeDefinition {
    type OfType = introspection::Never;

    fn description(&self) -> Option<&str> {
        self.description()
    }

    fn kind(&self) -> introspection::TypeKind {
        introspection::TypeKind::Union
    }

    fn name(&self) -> Option<&str> {
        Some(&self.name)
    }

    fn possible_types(&self) -> Option<UnionMemberTypes> {
        Some(self.member_types)
    }
}

pub fn init() -> Result<(), Error> {
    let class = root().define_class("UnionTypeDefinition", Default::default())?;

    class.define_singleton_method("new", function!(UnionTypeDefinition::new, 1))?;
    introspection::implement_type!(UnionTypeDefinition, class);

    Ok(())
}
