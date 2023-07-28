use bluejay_core::definition::{SchemaDefinition, TypeDefinitionReference, prelude::*};
use super::helpers::{type_description, type_kind};

#[derive(Eq, Ord, PartialEq, PartialOrd)]
pub enum Criticality {
    Breaking { reason: String },
    Dangerous { reason: String },
    NonBreaking { reason: String },
}

impl Criticality {
    pub fn breaking(reason: Option<String>) -> Self {
        Self::Breaking { reason: reason.unwrap_or(String::from("This change is a breaking change")) }
    }

    pub fn dangerous(reason: Option<String>) -> Self {
        Self::Dangerous { reason: reason.unwrap_or(String::from("This change is dangerous")) }
    }

    pub fn non_breaking(reason: Option<String>) -> Self {
        Self::NonBreaking { reason: reason.unwrap_or(String::from("This change is safe")) }
    }
}

pub enum Change<'a, S: SchemaDefinition> {
    TypeRemoved{
        removed_type: TypeDefinitionReference<'a, S::TypeDefinition>,
    },
    TypeAdded{
        added_type: TypeDefinitionReference<'a, S::TypeDefinition>,
    },
    TypeKindChanged{
        old_type: TypeDefinitionReference<'a, S::TypeDefinition>,
        new_type: TypeDefinitionReference<'a, S::TypeDefinition>,
    },
    TypeDescriptionChanged{
        old_type: TypeDefinitionReference<'a, S::TypeDefinition>,
        new_type: TypeDefinitionReference<'a, S::TypeDefinition>,
    },
    FieldAdded{
        added_field: &'a S::FieldDefinition,
        object_type: &'a S::ObjectTypeDefinition,
    },
    FieldRemoved{
        removed_field: &'a S::FieldDefinition,
        object_type: &'a S::ObjectTypeDefinition,
    },
    FieldDescriptionChanged{
        object_type: &'a S::ObjectTypeDefinition,
        old_field: &'a S::FieldDefinition,
        new_field: &'a S::FieldDefinition,
    },
}

impl<'a, S: SchemaDefinition>  Change<'a, S> {
    pub fn breaking(&self) -> bool {
        matches!(self.criticality(), Criticality::Breaking { .. })
    }

    pub fn non_breaking(&self) -> bool {
        matches!(self.criticality(), Criticality::NonBreaking { .. })
    }

    pub fn dangerous(&self) -> bool {
        matches!(self.criticality(), Criticality::Dangerous { .. })
    }

    pub fn criticality(&self) -> Criticality {
        match self {
            Self::TypeRemoved{ removed_type } => Criticality::breaking(
                Some("Removing a type is a breaking change. It is preferable to deprecate and remove all references to this type first.".to_string())
            ),
            Self::TypeAdded{ added_type } => Criticality::non_breaking(None),
            Self::TypeKindChanged{ old_type, new_type } => Criticality::non_breaking(None),
            Self::TypeDescriptionChanged{ old_type, new_type } => Criticality::non_breaking(None),
            Self::FieldAdded{ added_field, object_type } => Criticality::non_breaking(None),
            Self::FieldRemoved{ removed_field, object_type } => {
                // TODO: conditional criticality depending on deprecated or not
                Criticality::non_breaking(None)
            },
            Self::FieldDescriptionChanged{ object_type, old_field, new_field } => Criticality::non_breaking(None),
        }
    }

    pub fn message(&self) -> String {
        match self {
            Self::TypeRemoved{ removed_type } => {
                format!("Type `{}` was removed", removed_type.name())
            },
            Self::TypeAdded{ added_type } => {
                format!("Type `{}` was added", added_type.name())
            },
            Self::TypeKindChanged{ old_type, new_type } => {
                format!(
                    "`{}` kind changed from `{}` to `{}`",
                    old_type.name(),
                    type_kind(old_type),
                    type_kind(new_type)
                )
            },
            Self::TypeDescriptionChanged{ old_type, new_type } => {
                format!(
                    "Description `{}` on type `{}` has changed to `{}`",
                    type_description(old_type).unwrap_or(""),
                    old_type.name(),
                    type_description(new_type).unwrap_or("")
                )
            },
            Self::FieldAdded{ added_field, object_type } => {
                format!("Field `{}` was added to object type `{}`", added_field.name(), object_type.name())
            },
            Self::FieldRemoved{ removed_field, object_type } => {
                format!("Field `{}` was removed from object type `{}`", removed_field.name(), object_type.name())
            },
            Self::FieldDescriptionChanged{ object_type, old_field, new_field } => {
                format!("Field `{}` description changed from `{} to `{}`", self.path(), old_field.description().unwrap_or(""), new_field.description().unwrap_or(""))
            },
        }
    }

    pub fn path(&self) -> String {
        match self {
            Self::TypeRemoved{ removed_type} => {
                removed_type.name().to_string()
            },
            Self::TypeAdded{ added_type } => {
                added_type.name().to_string()
            },
            Self::TypeKindChanged{ old_type, new_type } => {
                old_type.name().to_string()
            },
            Self::TypeDescriptionChanged{ old_type, new_type } => {
                old_type.name().to_string()
            },
            Self::FieldAdded{ added_field, object_type } => {
                vec![object_type.name(), added_field.name()].join(".")
            },
            Self::FieldRemoved{ removed_field, object_type } => {
                vec![object_type.name(), removed_field.name()].join(".")
            },
            Self::FieldDescriptionChanged{ object_type, old_field, new_field} => {
                vec![object_type.name(), old_field.name()].join(".")
            },
        }
    }
}



