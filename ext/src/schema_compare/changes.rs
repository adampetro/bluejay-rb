use bluejay_core::definition::{SchemaDefinition, TypeDefinitionReference, ObjectTypeDefinition, FieldDefinition, InputValueDefinition, InputObjectTypeDefinition, InterfaceTypeDefinition, InputType, EnumTypeDefinition, EnumValueDefinition, UnionTypeDefinition, OutputType, DirectiveDefinition, DirectiveLocation};
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
        type_name: &'a str,
    },
    FieldRemoved{
        removed_field: &'a S::FieldDefinition,
        type_name: &'a str,
    },
    FieldDescriptionChanged{
        type_name: &'a str,
        old_field: &'a S::FieldDefinition,
        new_field: &'a S::FieldDefinition,
    },
    FieldTypeChanged{
        type_name: &'a str,
        old_field: &'a S::FieldDefinition,
        new_field: &'a S::FieldDefinition,
    },
    FieldArgumentAdded{
        type_name: &'a str,
        field: &'a S::FieldDefinition,
        argument: &'a S::InputValueDefinition,
    },
    FieldArgumentRemoved{
        type_name: &'a str,
        field: &'a S::FieldDefinition,
        argument: &'a S::InputValueDefinition,
    },
    FieldArgumentDescriptionChanged{
        type_name: &'a str,
        field: &'a S::FieldDefinition,
        old_argument: &'a S::InputValueDefinition,
        new_argument: &'a S::InputValueDefinition,
    },
    FieldArgumentDefaultValueChanged{
        type_name: &'a str,
        field: &'a S::FieldDefinition,
        old_argument: &'a S::InputValueDefinition,
        new_argument: &'a S::InputValueDefinition,
    },
    FieldArgumentTypeChanged{
        type_name: &'a str,
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
    DirectiveAdded{
        directive: &'a S::DirectiveDefinition,
    },
    DirectiveRemoved{
        directive: &'a S::DirectiveDefinition,
    },
    DirectiveLocationAdded{
        directive: &'a S::DirectiveDefinition,
        location: &'a DirectiveLocation,
    },
    DirectiveLocationRemoved{
        directive: &'a S::DirectiveDefinition,
        location: &'a DirectiveLocation,
    },
    DirectiveDescriptionChanged{
        old_directive: &'a S::DirectiveDefinition,
        new_directive: &'a S::DirectiveDefinition,
    },
    DirectiveArgumentAdded{
        directive: &'a S::DirectiveDefinition,
        argument: &'a S::InputValueDefinition,
    },
    DirectiveArgumentRemoved{
        directive: &'a S::DirectiveDefinition,
        argument: &'a S::InputValueDefinition,
    },
    DirectiveArgumentDescriptionChanged{
        directive: &'a S::DirectiveDefinition,
        old_argument: &'a S::InputValueDefinition,
        new_argument: &'a S::InputValueDefinition,
    },
    DirectiveArgumentDefaultValueChanged{
        directive: &'a S::DirectiveDefinition,
        old_argument: &'a S::InputValueDefinition,
        new_argument: &'a S::InputValueDefinition,
    },
    DirectiveArgumentTypeChanged{
        directive: &'a S::DirectiveDefinition,
        old_argument: &'a S::InputValueDefinition,
        new_argument: &'a S::InputValueDefinition,
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
            Self::FieldAdded{ added_field, type_name } => Criticality::non_breaking(None),
            Self::FieldRemoved{ removed_field, type_name } => {
                // TODO: conditional criticality depending on deprecated or not
                Criticality::non_breaking(None)
            },
            Self::FieldDescriptionChanged{ type_name, old_field, new_field } => Criticality::non_breaking(None),
            Self::FieldTypeChanged{ type_name, old_field, new_field } => {
                // TODO: conditional criticality depending on safe type change
                Criticality::non_breaking(None)
            },
            Self::FieldArgumentAdded{ type_name, field, argument } => {
                // TODO conditional criticality
                Criticality::non_breaking(None)
            },
            Self::FieldArgumentRemoved{ type_name, field, argument } => {
                // TODO conditional criticality
                Criticality::breaking(Some("Removing a field argument is a breaking change because it will cause existing queries that use this argument to error.".to_string()))
            },
            Self::FieldArgumentDescriptionChanged{ type_name, field, old_argument, new_argument } => {
                Criticality::non_breaking(None)
            },
            Self::FieldArgumentDefaultValueChanged{ type_name, field, old_argument, new_argument } => {
                // TODO conditional criticality
                Criticality::dangerous(Some("Changing the default value for an argument may change the runtime behaviour of a field if it was never provided.".to_string()))
            },
            Self::FieldArgumentTypeChanged{ type_name, field, old_argument, new_argument } => {
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
            Self::DirectiveAdded{ directive } => {
                Criticality::non_breaking(None)
            },
            Self::DirectiveRemoved{ directive } => {
                Criticality::breaking(None)
            },
            Self::DirectiveLocationAdded{ directive, location } => {
                Criticality::non_breaking(None)
            },
            Self::DirectiveLocationRemoved{ directive, location } => {
                Criticality::breaking(None)
            },
            Self::DirectiveDescriptionChanged{ old_directive, new_directive } => {
                Criticality::non_breaking(None)
            },
            Self::DirectiveArgumentAdded{ directive, argument } => {
                // TODO: conditional criticality

                if argument.is_required() {
                    Criticality::breaking(None)
                } else {
                    Criticality::non_breaking(None)
                }
            },
            Self::DirectiveArgumentRemoved{ directive, argument } => {
                Criticality::breaking(None)
            },
            Self::DirectiveArgumentDescriptionChanged { directive, old_argument, new_argument } => {
                Criticality::non_breaking(None)
            },
            Self::DirectiveArgumentTypeChanged { directive, old_argument, new_argument } => {
                // TODO: conditional criticality
                Criticality::breaking(None)
            },
            Self::DirectiveArgumentDefaultValueChanged { directive, old_argument, new_argument } => {
                Criticality::dangerous(Some("Changing the default value for an argument may change the runtime behaviour of a field if it was never provided.".to_string()))
            }
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
            Self::FieldAdded{ added_field, type_name } => {
                format!("Field `{}` was added to object type `{}`", added_field.name(), type_name)
            },
            Self::FieldRemoved{ removed_field, type_name } => {
                format!("Field `{}` was removed from object type `{}`", removed_field.name(), type_name)
            },
            Self::FieldDescriptionChanged{ type_name, old_field, new_field } => {
                format!("Field `{}` description changed from `{} to `{}`", self.path(), old_field.description().unwrap_or(""), new_field.description().unwrap_or(""))
            },
            Self::FieldTypeChanged{ type_name, old_field, new_field } => {
                format!("Field `{}.{}` changed type from `{}` to `{}`.", type_name, old_field.name(), old_field.r#type().as_ref().display_name(), new_field.r#type().as_ref().display_name())
            },
            Self::FieldArgumentAdded{ type_name, field, argument } => {
                format!("Argument `{}` was added to field `{}.{}`", argument.name(), type_name, field.name())
            },
            Self::FieldArgumentRemoved{ type_name, field, argument } => {
                format!("Argument `{}` was removed from field `{}.{}`", argument.name(), type_name, field.name())
            },
            Self::FieldArgumentDescriptionChanged{ type_name, field, old_argument, new_argument } => {
                format!("Description for argument `{}` on field `{}.{}` changed from `{}` to `{}`", new_argument.name(), field.name(), type_name, old_argument.description().unwrap_or(""), new_argument.description().unwrap_or(""))
            },
            Self::FieldArgumentDefaultValueChanged{ type_name, field, old_argument, new_argument } => {
                // TODO: exhaustive cases here are weird
                match (old_argument.default_value(), new_argument.default_value()) {
                    (Some(old_default_value), Some(new_default_value)) => {
                        if old_default_value.as_ref() != new_default_value.as_ref() {
                            format!("Default value for argument `{}` on field `{}.{}` was changed from `{} to `{}`", old_argument.name(), type_name, field.name(), old_default_value.as_ref(), new_default_value.as_ref())
                        } else {
                            "".to_string()
                        }
                    },
                    (Some(old_default_value), None) => {
                        format!("Default value `{}` was removed from argument `{}` on field `{}.{}`", old_default_value.as_ref(), old_argument.name(), type_name, field.name())
                    },
                    (None, Some(new_default_value)) => {
                        format!("Default value `{}` was added to argument `{}` on field `{}.{}`", new_default_value.as_ref(), new_argument.name(), type_name, field.name())
                    },
                    (None, None) => { "".to_string() }
                }
            },
            Self::FieldArgumentTypeChanged{ type_name, field, old_argument, new_argument } => {
                format!("Type for argument `{}` on field `{}.{}` changed from `{}` to `{}`", new_argument.name(), field.name(), type_name, old_argument.r#type().as_ref().display_name(), new_argument.r#type().as_ref().display_name())
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
            Self::DirectiveAdded{ directive } => {
                format!("Directive `{}` was added", directive.name())
            },
            Self::DirectiveRemoved{ directive } => {
                format!("Directive `{}` was removed", directive.name())
            },
            Self::DirectiveLocationAdded{ directive, location } => {
                format!("Location `{}` was added to directive `{}`", location, directive.name())
            },
            Self::DirectiveLocationRemoved{ directive, location } => {
                format!("Location `{}` was removed from directive `{}`", location, directive.name())
            },
            Self::DirectiveDescriptionChanged { old_directive, new_directive } => {
                format!("Directive `{}` description changed from `{}` to `{}`", new_directive.name(), old_directive.description().unwrap_or(""), new_directive.description().unwrap_or(""))
            },
            Self::DirectiveArgumentAdded{ directive, argument } => {
                format!("Argument `{}` was added to directive `{}`", argument.name(), directive.name())
            },
            Self::DirectiveArgumentRemoved{ directive, argument } => {
                format!("Argument `{}` was removed from directive `{}`", argument.name(), directive.name())
            },
            Self::DirectiveArgumentDescriptionChanged { directive, old_argument, new_argument } => {
                format!("Description for argument `{}` on directive `{}.{}` changed from `{}` to `{}`", new_argument.name(), directive.name(), old_argument.name(), old_argument.description().unwrap_or(""), new_argument.description().unwrap_or(""))
            },
            Self::DirectiveArgumentTypeChanged { directive, old_argument, new_argument } => {
                format!("Type for argument `{}` on directive `{}.{}` changed from `{}` to `{}`", new_argument.name(), directive.name(), old_argument.name(), old_argument.r#type().as_ref().display_name(), new_argument.r#type().as_ref().display_name())
            },
            Self::DirectiveArgumentDefaultValueChanged { directive, old_argument, new_argument } => {
                // TODO: exhaustive cases here are weird
                match (old_argument.default_value(), new_argument.default_value()) {
                    (Some(old_default_value), Some(new_default_value)) => {
                        if old_default_value.as_ref() != new_default_value.as_ref() {
                            format!("Directive argument `{}.{}` default valut changed from `{}` to `{}`", directive.name(), new_argument.name(), old_default_value.as_ref(), new_default_value.as_ref())
                        } else {
                            "".to_string()
                        }
                    },
                    (Some(old_default_value), None) => {
                        format!("Default value `{}` was removed from directive argument `{}.{}`", old_default_value.as_ref(), directive.name(), old_argument.name())
                    },
                    (None, Some(new_default_value)) => {
                        format!("Default value `{}` was added to directive argument `{}.{}`", new_default_value.as_ref(), directive.name(), old_argument.name())
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
            Self::FieldAdded{ added_field, type_name } => {
                vec![type_name, added_field.name()].join(".")
            },
            Self::FieldRemoved{ removed_field, type_name } => {
                vec![type_name, removed_field.name()].join(".")
            },
            Self::FieldDescriptionChanged{ type_name, old_field, new_field} => {
                vec![type_name, old_field.name()].join(".")
            },
            Self::FieldTypeChanged{ type_name, old_field, new_field } => {
                vec![type_name, old_field.name()].join(".")
            },
            Self::FieldArgumentAdded{ type_name, field, argument } => {
                vec![type_name, field.name(), argument.name()].join(".")
            },
            Self::FieldArgumentRemoved{ type_name, field, argument } => {
                vec![type_name, field.name(), argument.name()].join(".")
            },
            Self::FieldArgumentDescriptionChanged{ type_name, field, old_argument, new_argument } => {
                vec![type_name, field.name(), old_argument.name()].join(".")
            },
            Self::FieldArgumentDefaultValueChanged{ type_name, field, old_argument, new_argument } => {
                vec![type_name, field.name(), old_argument.name()].join(".")
            },
            Self::FieldArgumentTypeChanged{ type_name, field, old_argument, new_argument } => {
                vec![type_name, field.name(), old_argument.name()].join(".")
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
            Self::DirectiveAdded{ directive } => {
                directive.name().to_string()
            },
            Self::DirectiveRemoved{ directive } => {
                format!("@{}", directive.name())
            },
            Self::DirectiveLocationAdded{ directive, location } => {
                format!("@{}", directive.name())
            },
            Self::DirectiveLocationRemoved{ directive, location } => {
                format!("@{}", directive.name())
            },
            Self::DirectiveDescriptionChanged { old_directive, new_directive } => {
                format!("@{}", new_directive.name())
            },
            Self::DirectiveArgumentAdded{ directive, argument } => {
                format!("@{}.{}", directive.name(), argument.name())
            },
            Self::DirectiveArgumentRemoved{ directive, argument } => {
                format!("@{}.{}", directive.name(), argument.name())
            },
            Self::DirectiveArgumentDescriptionChanged { directive, old_argument, new_argument } => {
                format!("@{}.{}", directive.name(), old_argument.name())
            },
            Self::DirectiveArgumentTypeChanged { directive, old_argument, new_argument } => {
                format!("@{}.{}", directive.name(), old_argument.name())
            },
            Self::DirectiveArgumentDefaultValueChanged { directive, old_argument, new_argument } => {
                format!("@{}.{}", directive.name(), old_argument.name())
            },
        }
    }
}



