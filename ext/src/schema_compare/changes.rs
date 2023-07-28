use bluejay_core::definition::{SchemaDefinition, TypeDefinitionReference, ObjectTypeDefinition, FieldDefinition, InputValueDefinition, InterfaceTypeDefinition, InputType};
use bluejay_core::Value;
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
    FieldTypeChanged{
        object_type: &'a S::ObjectTypeDefinition,
        old_field: &'a S::FieldDefinition,
        new_field: &'a S::FieldDefinition,
    },
    FieldArgumentAdded{
        object_type: &'a S::ObjectTypeDefinition,
        field: &'a S::FieldDefinition,
        argument: &'a S::InputValueDefinition,
    },
    FieldArgumentRemoved{
        object_type: &'a S::ObjectTypeDefinition,
        field: &'a S::FieldDefinition,
        argument: &'a S::InputValueDefinition,
    },
    FieldArgumentDescriptionChanged{
        object_type: &'a S::ObjectTypeDefinition,
        field: &'a S::FieldDefinition,
        old_argument: &'a S::InputValueDefinition,
        new_argument: &'a S::InputValueDefinition,
    },
    FieldArgumentDefaultValueChanged{
        object_type: &'a S::ObjectTypeDefinition,
        field: &'a S::FieldDefinition,
        old_argument: &'a S::InputValueDefinition,
        new_argument: &'a S::InputValueDefinition,
    },
    FieldArgumentTypeChanged{
        object_type: &'a S::ObjectTypeDefinition,
        field: &'a S::FieldDefinition,
        old_argument: &'a S::InputValueDefinition,
        new_argument: &'a S::InputValueDefinition,
    },
    ObjectInterfaceAddition{
        object_type: &'a S::ObjectTypeDefinition,
        interface: &'a S::InterfaceTypeDefinition,
    },
    ObjectInterfaceRemoval{
        object_type: &'a S::ObjectTypeDefinition,
        interface: &'a S::InterfaceTypeDefinition,
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
            Self::FieldTypeChanged{ object_type, old_field, new_field } => {
                // TODO: conditional criticality depending on safe type change
                Criticality::non_breaking(None)
            },
            Self::FieldArgumentAdded{ object_type, field, argument } => {
                // TODO conditional criticality
                Criticality::non_breaking(None)
            },
            Self::FieldArgumentRemoved{ object_type, field, argument } => {
                // TODO conditional criticality
                Criticality::breaking(Some("Removing a field argument is a breaking change because it will cause existing queries that use this argument to error.".to_string()))
            },
            Self::FieldArgumentDescriptionChanged{ object_type, field, old_argument, new_argument } => {
                Criticality::non_breaking(None)
            },
            Self::FieldArgumentDefaultValueChanged{ object_type, field, old_argument, new_argument } => {
                // TODO conditional criticality
                Criticality::dangerous(Some("Changing the default value for an argument may change the runtime behaviour of a field if it was never provided.".to_string()))
            },
            Self::FieldArgumentTypeChanged{ object_type, field, old_argument, new_argument } => {
                // TODO conditional criticality
                Criticality::dangerous(Some("Changing the type of a field's argument can cause existing queries that use this argument to error.".to_string()))
            },
            Self::ObjectInterfaceAddition{ object_type, interface } => {
                Criticality::dangerous(Some("Adding an interface to an object type may break existing clients that were not programming defensively against a new possible type.".to_string()))
            },
            Self::ObjectInterfaceRemoval{ object_type, interface } => {
                // TODO conditional criticality
                Criticality::breaking(Some("Changing the type of a field's argument can cause existing queries that use this argume.".to_string()))
            },
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
            Self::FieldTypeChanged{ object_type, old_field, new_field } => {
                //format!("Field `{}` changed type from `{}` to `{}`", self.path(), old_field.type().name(), new_field.type().name())
                format!("Field `{}.{}` type changed.", object_type.name(), old_field.name())
            },
            Self::FieldArgumentAdded{ object_type, field, argument } => {
                format!("Argument `{}` was added to field `{}.{}`", argument.name(), object_type.name(), field.name())
            },
            Self::FieldArgumentRemoved{ object_type, field, argument } => {
                format!("Argument `{}` was removed from field `{}.{}`", argument.name(), object_type.name(), field.name())
            },
            Self::FieldArgumentDescriptionChanged{ object_type, field, old_argument, new_argument } => {
                format!("Description for argument `{}` on field `{}.{}` changed from `{}` to `{}`", new_argument.name(), field.name(), object_type.name(), old_argument.description().unwrap_or(""), new_argument.description().unwrap_or(""))
            },
            Self::FieldArgumentDefaultValueChanged{ object_type, field, old_argument, new_argument } => {
                // TODO: exhaustive cases here are weird
                match (old_argument.default_value(), new_argument.default_value()) {
                    (Some(old_default_value), Some(new_default_value)) => {
                        if old_default_value.as_ref() != new_default_value.as_ref() {
                            format!("Default value for argument `{}` on field `{}.{}` was changed from `{} to `{}`", old_argument.name(), object_type.name(), field.name(), old_default_value.as_ref(), new_default_value.as_ref())
                        } else {
                            "".to_string()
                        }
                    },
                    (Some(old_default_value), None) => {
                        format!("Default value `{}` was removed from argument `{}` on field `{}.{}`", old_default_value.as_ref(), old_argument.name(), object_type.name(), field.name())
                    },
                    (None, Some(new_default_value)) => {
                        format!("Default value `{}` was added to argument `{}` on field `{}.{}`", new_default_value.as_ref(), new_argument.name(), object_type.name(), field.name())
                    },
                    (None, None) => { "".to_string() }
                }
            },
            Self::FieldArgumentTypeChanged{ object_type, field, old_argument, new_argument } => {
                format!("Type for argument `{}` on field `{}.{}` changed from `{}` to `{}`", new_argument.name(), field.name(), object_type.name(), old_argument.r#type().as_ref().display_name(), new_argument.r#type().as_ref().display_name())
            },
            Self::ObjectInterfaceAddition{ object_type, interface } => {
                format!("`{}` object implements `{}` interface", object_type.name(), interface.name())
            },
            Self::ObjectInterfaceRemoval{ object_type, interface } => {
                format!("`{}` object type no longer implements `{}` interface", object_type.name(), interface.name())
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
            Self::FieldTypeChanged{ object_type, old_field, new_field } => {
                vec![object_type.name(), old_field.name()].join(".")
            },
            Self::FieldArgumentAdded{ object_type, field, argument } => {
                vec![object_type.name(), field.name(), argument.name()].join(".")
            },
            Self::FieldArgumentRemoved{ object_type, field, argument } => {
                vec![object_type.name(), field.name(), argument.name()].join(".")
            },
            Self::FieldArgumentDescriptionChanged{ object_type, field, old_argument, new_argument } => {
                vec![object_type.name(), field.name(), old_argument.name()].join(".")
            },
            Self::FieldArgumentDefaultValueChanged{ object_type, field, old_argument, new_argument } => {
                vec![object_type.name(), field.name(), old_argument.name()].join(".")
            },
            Self::FieldArgumentTypeChanged{ object_type, field, old_argument, new_argument } => {
                vec![object_type.name(), field.name(), old_argument.name()].join(".")
            },
            Self::ObjectInterfaceAddition{ object_type, interface } => {
                object_type.name().to_string()
            },
            Self::ObjectInterfaceRemoval{ object_type, interface } => {
                object_type.name().to_string()
            },
        }
    }
}


