use bluejay_core::definition::{SchemaDefinition, TypeDefinitionReference, ObjectTypeDefinition, FieldDefinition, InputValueDefinition, InputObjectTypeDefinition, InterfaceTypeDefinition, InputType, EnumTypeDefinition, EnumValueDefinition, UnionTypeDefinition, OutputType};
use bluejay_core::Value;
use super::helpers::{type_description, type_kind};
use super::diff::TypeWithFields;

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
        parent_type: &'a TypeWithFields<'a, S>,
    },
    FieldRemoved{
        removed_field: &'a S::FieldDefinition,
        parent_type: &'a TypeWithFields<'a, S>,
    },
    FieldDescriptionChanged{
        parent_type: &'a TypeWithFields<'a, S>,
        old_field: &'a S::FieldDefinition,
        new_field: &'a S::FieldDefinition,
    },
    FieldTypeChanged{
        parent_type: &'a TypeWithFields<'a, S>,
        old_field: &'a S::FieldDefinition,
        new_field: &'a S::FieldDefinition,
    },
    FieldArgumentAdded{
        parent_type: &'a TypeWithFields<'a, S>,
        field: &'a S::FieldDefinition,
        argument: &'a S::InputValueDefinition,
    },
    FieldArgumentRemoved{
        parent_type: &'a TypeWithFields<'a, S>,
        field: &'a S::FieldDefinition,
        argument: &'a S::InputValueDefinition,
    },
    FieldArgumentDescriptionChanged{
        parent_type: &'a TypeWithFields<'a, S>,
        field: &'a S::FieldDefinition,
        old_argument: &'a S::InputValueDefinition,
        new_argument: &'a S::InputValueDefinition,
    },
    FieldArgumentDefaultValueChanged{
        parent_type: &'a TypeWithFields<'a, S>,
        field: &'a S::FieldDefinition,
        old_argument: &'a S::InputValueDefinition,
        new_argument: &'a S::InputValueDefinition,
    },
    FieldArgumentTypeChanged{
        parent_type: &'a TypeWithFields<'a, S>,
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
    EnumValueAdded{
        enum_type: &'a S::EnumTypeDefinition,
        enum_value: &'a S::EnumValueDefinition,
    },
    EnumValueRemoved{
        enum_type: &'a S::EnumTypeDefinition,
        enum_value: &'a S::EnumValueDefinition,
    },
    EnumValueDescriptionChanged{
        enum_type: &'a S::EnumTypeDefinition,
        old_enum_value: &'a S::EnumValueDefinition,
        new_enum_value: &'a S::EnumValueDefinition,
    },
    UnionMemberAdded{
        union_type: &'a S::UnionTypeDefinition,
        union_member: &'a S::ObjectTypeDefinition,
    },
    UnionMemberRemoved{
        union_type: &'a S::UnionTypeDefinition,
        union_member: &'a S::ObjectTypeDefinition,
    },
    InputFieldAdded{
        input_object_type: &'a S::InputObjectTypeDefinition,
        added_field: &'a S::InputValueDefinition,
    },
    InputFieldRemoved{
        input_object_type: &'a S::InputObjectTypeDefinition,
        removed_field: &'a S::InputValueDefinition,
    },
    InputFieldDescriptionChanged{
        input_object_type: &'a S::InputObjectTypeDefinition,
        old_field: &'a S::InputValueDefinition,
        new_field: &'a S::InputValueDefinition,
    },
    InputFieldTypeChanged{
        input_object_type: &'a S::InputObjectTypeDefinition,
        old_field: &'a S::InputValueDefinition,
        new_field: &'a S::InputValueDefinition,
    },
    InputFieldDefaultValueChanged{
        input_object_type: &'a S::InputObjectTypeDefinition,
        old_field: &'a S::InputValueDefinition,
        new_field: &'a S::InputValueDefinition,
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
            Self::FieldAdded{ added_field, parent_type } => Criticality::non_breaking(None),
            Self::FieldRemoved{ removed_field, parent_type } => {
                // TODO: conditional criticality depending on deprecated or not
                Criticality::non_breaking(None)
            },
            Self::FieldDescriptionChanged{ parent_type, old_field, new_field } => Criticality::non_breaking(None),
            Self::FieldTypeChanged{ parent_type, old_field, new_field } => {
                // TODO: conditional criticality depending on safe type change
                Criticality::non_breaking(None)
            },
            Self::FieldArgumentAdded{ parent_type, field, argument } => {
                // TODO conditional criticality
                Criticality::non_breaking(None)
            },
            Self::FieldArgumentRemoved{ parent_type, field, argument } => {
                // TODO conditional criticality
                Criticality::breaking(Some("Removing a field argument is a breaking change because it will cause existing queries that use this argument to error.".to_string()))
            },
            Self::FieldArgumentDescriptionChanged{ parent_type, field, old_argument, new_argument } => {
                Criticality::non_breaking(None)
            },
            Self::FieldArgumentDefaultValueChanged{ parent_type, field, old_argument, new_argument } => {
                // TODO conditional criticality
                Criticality::dangerous(Some("Changing the default value for an argument may change the runtime behaviour of a field if it was never provided.".to_string()))
            },
            Self::FieldArgumentTypeChanged{ parent_type, field, old_argument, new_argument } => {
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
            Self::EnumValueAdded{ enum_type, enum_value } => {
                Criticality::dangerous(Some("Adding an enum value may break existing clients that were not programming defensively against an added case when querying an enum.".to_string()))
            },
            Self::EnumValueRemoved{ enum_type, enum_value } => {
                Criticality::breaking(Some("Removing an enum value will cause existing queries that use this enum value to error.".to_string()))
            },
            Self::EnumValueDescriptionChanged{ enum_type, old_enum_value, new_enum_value } => {
                Criticality::non_breaking(None)
            },
            Self::UnionMemberAdded{ union_type, union_member } => {
                Criticality::dangerous(Some("Adding a possible type to Unions may break existing clients that were not programming defensively against a new possible type..".to_string()))
            },
            Self::UnionMemberRemoved{ union_type, union_member } => {
                Criticality::breaking(Some("Removing a union member from a union can cause existing queries that use this union member in a fragment spread to error.".to_string()))
            },
            Self::InputFieldAdded{ input_object_type, added_field } => {
                // TODO: conditional criticality
                Criticality::breaking(Some("Adding a non-null input field without a default value to an existing input type will cause existing queries that use this input type to error because they will not provide a value for this new field.".to_string()))
            },
            Self::InputFieldRemoved{ input_object_type, removed_field } => {
                Criticality::breaking(Some("Removing an input field will cause existing queries that use this input field to error.".to_string()))
            },
            Self::InputFieldTypeChanged { input_object_type, old_field, new_field } => {
                // TODO: conditional criticality
                Criticality::dangerous(Some("TODO".to_string()))
            },
            Self::InputFieldDescriptionChanged { input_object_type, old_field, new_field } => {
                Criticality::non_breaking(None)
            },
            Self::InputFieldDefaultValueChanged { input_object_type, old_field, new_field } => {
                Criticality::dangerous(Some("Changing the default value for an argument may change the runtime behaviour of a field if it was never provided.".to_string()))
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
            Self::FieldAdded{ added_field, parent_type } => {
                format!("Field `{}` was added to object type `{}`", added_field.name(), parent_type.name())
            },
            Self::FieldRemoved{ removed_field, parent_type } => {
                format!("Field `{}` was removed from object type `{}`", removed_field.name(), parent_type.name())
            },
            Self::FieldDescriptionChanged{ parent_type, old_field, new_field } => {
                format!("Field `{}` description changed from `{} to `{}`", self.path(), old_field.description().unwrap_or(""), new_field.description().unwrap_or(""))
            },
            Self::FieldTypeChanged{ parent_type, old_field, new_field } => {
                format!("Field `{}.{}` changed type from `{}` to `{}`.", parent_type.name(), old_field.name(), old_field.r#type().as_ref().display_name(), new_field.r#type().as_ref().display_name())
            },
            Self::FieldArgumentAdded{ parent_type, field, argument } => {
                format!("Argument `{}` was added to field `{}.{}`", argument.name(), parent_type.name(), field.name())
            },
            Self::FieldArgumentRemoved{ parent_type, field, argument } => {
                format!("Argument `{}` was removed from field `{}.{}`", argument.name(), parent_type.name(), field.name())
            },
            Self::FieldArgumentDescriptionChanged{ parent_type, field, old_argument, new_argument } => {
                format!("Description for argument `{}` on field `{}.{}` changed from `{}` to `{}`", new_argument.name(), field.name(), parent_type.name(), old_argument.description().unwrap_or(""), new_argument.description().unwrap_or(""))
            },
            Self::FieldArgumentDefaultValueChanged{ parent_type, field, old_argument, new_argument } => {
                // TODO: exhaustive cases here are weird
                match (old_argument.default_value(), new_argument.default_value()) {
                    (Some(old_default_value), Some(new_default_value)) => {
                        if old_default_value.as_ref() != new_default_value.as_ref() {
                            format!("Default value for argument `{}` on field `{}.{}` was changed from `{} to `{}`", old_argument.name(), parent_type.name(), field.name(), old_default_value.as_ref(), new_default_value.as_ref())
                        } else {
                            "".to_string()
                        }
                    },
                    (Some(old_default_value), None) => {
                        format!("Default value `{}` was removed from argument `{}` on field `{}.{}`", old_default_value.as_ref(), old_argument.name(), parent_type.name(), field.name())
                    },
                    (None, Some(new_default_value)) => {
                        format!("Default value `{}` was added to argument `{}` on field `{}.{}`", new_default_value.as_ref(), new_argument.name(), parent_type.name(), field.name())
                    },
                    (None, None) => { "".to_string() }
                }
            },
            Self::FieldArgumentTypeChanged{ parent_type, field, old_argument, new_argument } => {
                format!("Type for argument `{}` on field `{}.{}` changed from `{}` to `{}`", new_argument.name(), field.name(), parent_type.name(), old_argument.r#type().as_ref().display_name(), new_argument.r#type().as_ref().display_name())
            },
            Self::ObjectInterfaceAddition{ object_type, interface } => {
                format!("`{}` object implements `{}` interface", object_type.name(), interface.name())
            },
            Self::ObjectInterfaceRemoval{ object_type, interface } => {
                format!("`{}` object type no longer implements `{}` interface", object_type.name(), interface.name())
            },
            Self::EnumValueAdded{ enum_type, enum_value } => {
                format!("Enum value `{}` was added to enum `{}`", enum_value.name(), enum_type.name())
            },
            Self::EnumValueRemoved{ enum_type, enum_value } => {
                format!("Enum value `{}` was removed from enum `{}`", enum_value.name(), enum_type.name())
            },
            Self::EnumValueDescriptionChanged{ enum_type, old_enum_value, new_enum_value } => {
                format!("Description for enum value `{}.{}` changed from `{}` to `{}`", enum_type.name(), new_enum_value.name(), old_enum_value.description().unwrap_or(""), new_enum_value.description().unwrap_or(""))
            },
            Self::UnionMemberAdded{ union_type, union_member } => {
                format!("Union member `{}` was added to from union type `{}`", union_member.name(), union_type.name())
            },
            Self::UnionMemberRemoved{ union_type, union_member } => {
                format!("Union member `{}` was removed from union type `{}`", union_member.name(), union_type.name())
            },
            Self::InputFieldAdded{ input_object_type, added_field } => {
                format!("Input field `{}` was added to input object type `{}`", added_field.name(), input_object_type.name())
            },
            Self::InputFieldRemoved{ input_object_type, removed_field } => {
                format!("Input field `{}` was removed from input object type `{}`", removed_field.name(), input_object_type.name())
            },
            Self::InputFieldDescriptionChanged{ input_object_type, old_field, new_field } => {
                format!("Input field `{}.{}` description changed from `{}` to `{}`", input_object_type.name(), old_field.name(), old_field.description().unwrap_or(""), new_field.description().unwrap_or(""))
            },
            Self::InputFieldTypeChanged{ input_object_type, old_field, new_field } => {
                format!("Input field `{}.{}` changed type from `{}` to `{}`", input_object_type.name(), new_field.name(), old_field.r#type().as_ref().display_name(), new_field.r#type().as_ref().display_name())
            },
            Self::InputFieldDefaultValueChanged { input_object_type, old_field, new_field } => {
                // TODO: exhaustive cases here are weird
                match (old_field.default_value(), new_field.default_value()) {
                    (Some(old_default_value), Some(new_default_value)) => {
                        if old_default_value.as_ref() != new_default_value.as_ref() {
                            format!("Input field `{}.{}` default valut changed from `{}` to `{}`", input_object_type.name(), new_field.name(), old_default_value.as_ref(), new_default_value.as_ref())
                        } else {
                            "".to_string()
                        }
                    },
                    (Some(old_default_value), None) => {
                        format!("Default value `{}` was removed from input field `{}.{}`", old_default_value.as_ref(), input_object_type.name(), old_field.name())
                    },
                    (None, Some(new_default_value)) => {
                        format!("Default value `{}` was added to input field `{}.{}`", new_default_value.as_ref(), input_object_type.name(), old_field.name())
                    },
                    (None, None) => { "".to_string() }
                }
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
            Self::FieldAdded{ added_field, parent_type } => {
                vec![parent_type.name(), added_field.name()].join(".")
            },
            Self::FieldRemoved{ removed_field, parent_type } => {
                vec![parent_type.name(), removed_field.name()].join(".")
            },
            Self::FieldDescriptionChanged{ parent_type, old_field, new_field} => {
                vec![parent_type.name(), old_field.name()].join(".")
            },
            Self::FieldTypeChanged{ parent_type, old_field, new_field } => {
                vec![parent_type.name(), old_field.name()].join(".")
            },
            Self::FieldArgumentAdded{ parent_type, field, argument } => {
                vec![parent_type.name(), field.name(), argument.name()].join(".")
            },
            Self::FieldArgumentRemoved{ parent_type, field, argument } => {
                vec![parent_type.name(), field.name(), argument.name()].join(".")
            },
            Self::FieldArgumentDescriptionChanged{ parent_type, field, old_argument, new_argument } => {
                vec![parent_type.name(), field.name(), old_argument.name()].join(".")
            },
            Self::FieldArgumentDefaultValueChanged{ parent_type, field, old_argument, new_argument } => {
                vec![parent_type.name(), field.name(), old_argument.name()].join(".")
            },
            Self::FieldArgumentTypeChanged{ parent_type, field, old_argument, new_argument } => {
                vec![parent_type.name(), field.name(), old_argument.name()].join(".")
            },
            Self::ObjectInterfaceAddition{ object_type, interface } => {
                object_type.name().to_string()
            },
            Self::ObjectInterfaceRemoval{ object_type, interface } => {
                object_type.name().to_string()
            },
            Self::EnumValueAdded{ enum_type, enum_value } => {
                vec![enum_type.name(), enum_value.name()].join(".")
            },
            Self::EnumValueRemoved{ enum_type, enum_value } => {
                vec![enum_type.name(), enum_value.name()].join(".")
            },
            Self::EnumValueDescriptionChanged{ enum_type, old_enum_value, new_enum_value } => {
                vec![enum_type.name(), old_enum_value.name()].join(".")
            },
            Self::UnionMemberAdded{ union_type, union_member } => {
                union_type.name().to_string()
            },
            Self::UnionMemberRemoved{ union_type, union_member } => {
                union_type.name().to_string()
            },
            Self::InputFieldAdded{ input_object_type, added_field } => {
                vec![input_object_type.name(), added_field.name()].join(".")
            },
            Self::InputFieldRemoved{ input_object_type, removed_field } => {
                vec![input_object_type.name(), removed_field.name()].join(".")
            },
            Self::InputFieldDescriptionChanged { input_object_type, old_field, new_field } => {
                vec![input_object_type.name(), old_field.name()].join(".")
            },
            Self::InputFieldTypeChanged { input_object_type, old_field, new_field } => {
                vec![input_object_type.name(), old_field.name()].join(".")
            },
            Self::InputFieldDefaultValueChanged { input_object_type, old_field, new_field } => {
                vec![input_object_type.name(), old_field.name()].join(".")
            },
        }
    }
}



