use bluejay_core::definition::{SchemaDefinition, TypeDefinitionReference, ObjectTypeDefinition, FieldDefinition, FieldsDefinition, InputObjectTypeDefinition, InputFieldsDefinition, InputValueDefinition, OutputType, InterfaceImplementation, InterfaceTypeDefinition, InputType, EnumTypeDefinition, EnumValueDefinition, UnionTypeDefinition, UnionMemberTypes, UnionMemberType, DirectiveDefinition, DirectiveLocation};
use bluejay_core::{AsIter, Value};
use super::changes::*;
use super::helpers::{type_description, type_kind};

pub struct Schema<'a, S: SchemaDefinition> {
    old_schema: &'a S,
    new_schema: &'a S,
}

pub struct ObjectType<'a, S: SchemaDefinition> {
    old_type: &'a S::ObjectTypeDefinition,
    new_type: &'a S::ObjectTypeDefinition,
}

pub struct InputObjectType<'a, S: SchemaDefinition> {
    old_type: &'a S::InputObjectTypeDefinition,
    new_type: &'a S::InputObjectTypeDefinition,
}

pub struct Field<'a, S: SchemaDefinition> {
    type_name: &'a str,
    old_field: &'a S::FieldDefinition,
    new_field: &'a S::FieldDefinition,
}

pub struct InputField<'a, S: SchemaDefinition> {
    old_type: &'a S::InputObjectTypeDefinition,
    new_type: &'a S::InputObjectTypeDefinition,
    old_field: &'a S::InputValueDefinition,
    new_field: &'a S::InputValueDefinition,
}

pub struct Argument<'a, S: SchemaDefinition> {
    type_name: &'a str,
    field: &'a S::FieldDefinition,
    old_argument: &'a S::InputValueDefinition,
    new_argument: &'a S::InputValueDefinition,
}

pub struct Enum<'a, S: SchemaDefinition> {
    old_type: &'a S::EnumTypeDefinition,
    new_type: &'a S::EnumTypeDefinition,
}

pub struct Union<'a, S: SchemaDefinition> {
    old_type: &'a S::UnionTypeDefinition,
    new_type: &'a S::UnionTypeDefinition,
}

pub struct Interface<'a, S: SchemaDefinition> {
    old_interface: &'a S::InterfaceTypeDefinition,
    new_interface: &'a S::InterfaceTypeDefinition,
}

pub struct Directive<'a, S: SchemaDefinition> {
    old_directive: &'a S::DirectiveDefinition,
    new_directive: &'a S::DirectiveDefinition,
}

pub struct DirectiveArgument<'a, S: SchemaDefinition> {
    directive: &'a S::DirectiveDefinition,
    old_argument: &'a S::InputValueDefinition,
    new_argument: &'a S::InputValueDefinition,
}

impl<'a, S: SchemaDefinition> Schema<'a, S> {
    pub fn new(old_schema: &'a S, new_schema: &'a S) -> Self {
        Self {
            old_schema,
            new_schema,
        }
    }

    pub fn diff(&self) -> Vec<Change<S>> {
        let mut changes: Vec<Change<S>> = Vec::new();

        changes.extend(self.removed_types().into_iter()
            .map(|type_| Change::TypeRemoved {
                removed_type: type_,
            }));
        changes.extend(self.added_types().into_iter()
            .map(|type_| Change::TypeAdded {
                added_type: type_
            }));

        self.old_schema.type_definitions().for_each(|old_type| {
            let new_type = self.new_schema.get_type_definition(old_type.name());

            if new_type.is_some() {
                changes.extend(self.changes_in_type(old_type, new_type.unwrap()).into_iter());
            }
        });

        changes.extend(self.removed_directive_definitions().into_iter()
            .map(|directive| Change::DirectiveRemoved {
                directive: directive,
            }));
        changes.extend(self.added_directive_definitions().into_iter()
            .map(|directive| Change::DirectiveAdded {
                directive: directive,
            }));

        self.old_schema.directive_definitions().for_each(|old_directive| {
            let new_directive = self.new_schema.get_directive_definition(old_directive.name());

            if new_directive.is_some() {
                changes.extend(Directive::new(old_directive, new_directive.unwrap()).diff());
            }
        });

        changes
    }

    fn changes_in_type(&self, old_type: TypeDefinitionReference<'a, S::TypeDefinition>, new_type: TypeDefinitionReference<'a, S::TypeDefinition>) -> Vec<Change<S>> {
        let mut changes: Vec<Change<S>> = Vec::new();

        if type_kind(&old_type) != type_kind(&new_type) {
            changes.push(Change::TypeKindChanged {
                old_type,
                new_type,
            });
        } else {
            match (old_type, new_type) {
                (TypeDefinitionReference::Object(old_type), TypeDefinitionReference::Object(new_type)) => {
                    changes.extend(ObjectType::new(old_type, new_type).diff());
                },
                (TypeDefinitionReference::Enum(old_type), TypeDefinitionReference::Enum(new_type)) => {
                    changes.extend(Enum::new(old_type, new_type).diff());
                },
                (TypeDefinitionReference::Union(old_type), TypeDefinitionReference::Union(new_type)) => {
                    changes.extend(Union::new(old_type, new_type).diff());
                },
                (TypeDefinitionReference::Interface(old_type), TypeDefinitionReference::Interface(new_type)) => {
                    changes.extend(Interface::new(old_type, new_type).diff());
                },
                (TypeDefinitionReference::InputObject(old_type), TypeDefinitionReference::InputObject(new_type)) => {
                    changes.extend(InputObjectType::new(old_type, new_type).diff());
                },
                _ => { }
            }
        }

        if type_description(&old_type) != type_description(&new_type) {
            changes.push(Change::TypeDescriptionChanged {
                old_type,
                new_type,
            });
        }

        changes
    }

    fn chages_in_directive(&self, old_directive: &'a S::DirectiveDefinition, new_directive: &'a S::DirectiveDefinition) -> Vec<Change<S>> {
        let mut changes: Vec<Change<S>> = Vec::new();
        changes
    }

    fn removed_types(&self) -> Vec<TypeDefinitionReference<'_, S::TypeDefinition>> {
        let mut removed_types: Vec<TypeDefinitionReference<'_, S::TypeDefinition>> = Vec::new();

        self.old_schema.type_definitions().for_each(|old_type| {
            if self.new_schema.get_type_definition(old_type.name()).is_none() {
                removed_types.push(old_type);
            }
        });

        removed_types
    }

    fn added_types(&self) -> Vec<TypeDefinitionReference<'_, S::TypeDefinition>> {
        let mut added_types: Vec<TypeDefinitionReference<'_, S::TypeDefinition>> = Vec::new();

        self.new_schema.type_definitions().for_each(|new_type| {
            if self.old_schema.get_type_definition(new_type.name()).is_none() {
                added_types.push(new_type);
            }
        });

        added_types
    }

    fn removed_directive_definitions(&self) -> Vec<&'a S::DirectiveDefinition> {
        let mut removed_directive_definitions = Vec::new();

        self.old_schema.directive_definitions().for_each(|old_directive| {
            if self.new_schema.get_directive_definition(old_directive.name()).is_none() {
                removed_directive_definitions.push(old_directive);
            }
        });

        removed_directive_definitions
    }

    fn added_directive_definitions(&self) -> Vec<&'a S::DirectiveDefinition> {
        let mut added_directive_definitions = Vec::new();

        self.new_schema.directive_definitions().for_each(|new_directive| {
            if self.old_schema.get_directive_definition(new_directive.name()).is_none() {
                added_directive_definitions.push(new_directive);
            }
        });

        added_directive_definitions
    }
}

impl<'a, S: SchemaDefinition+'a> ObjectType<'a, S> {
    pub fn new(old_type: &'a S::ObjectTypeDefinition, new_type: &'a S::ObjectTypeDefinition) -> Self {
        Self {
            old_type,
            new_type,
        }
    }

    pub fn diff(&self) -> Vec<Change<'a, S>> {
        let mut changes = Vec::new();

        changes.extend(self.interface_additions().into_iter()
            .map(|interface| Change::ObjectInterfaceAddition { object_type: self.old_type, interface: interface }));
        changes.extend(self.interface_removals().into_iter()
            .map(|interface| Change::ObjectInterfaceRemoval { object_type: self.old_type, interface: interface }));

        changes.extend(self.field_additions().into_iter()
            .map(|field| Change::FieldAdded { added_field: field, type_name: self.new_type.name() }));
        changes.extend(self.field_removals().into_iter()
            .map(|field| Change::FieldRemoved { removed_field: field, type_name: self.new_type.name() }));

        self.old_type.fields_definition().iter().for_each(|old_field: &'a<S as SchemaDefinition>::FieldDefinition| {
            let new_field: Option<&'a<S as SchemaDefinition>::FieldDefinition> = self.new_type.fields_definition().get(old_field.name());

            if new_field.is_some() {
                changes.extend(Field::new(self.new_type.name(), old_field, new_field.unwrap()).diff());
            }
        });

        changes
    }

    fn field_additions(&self) -> Vec<&'a S::FieldDefinition> {
        let mut added_fields: Vec<&'a<S as SchemaDefinition>::FieldDefinition> = Vec::new();

        self.new_type.fields_definition().iter().for_each(|field: &'a<S as SchemaDefinition>::FieldDefinition| {
            if !self.old_type.fields_definition().contains_field(field.name()) {
                added_fields.push(field);
            }
        });

        added_fields
    }

    fn field_removals(&self) -> Vec<&'a S::FieldDefinition> {
        let mut removed_fields: Vec<&'a<S as SchemaDefinition>::FieldDefinition> = Vec::new();

        self.old_type.fields_definition().iter().for_each(|field: &'a<S as SchemaDefinition>::FieldDefinition| {
            if !self.new_type.fields_definition().contains_field(field.name()) {
                removed_fields.push(field);
            }
        });

        removed_fields
    }

    fn interface_additions(&self) -> Vec<&'a S::InterfaceTypeDefinition> {
        let mut added_interfaces = Vec::new();

        self.new_type.interface_implementations().map(|ii| ii.iter()).into_iter().flatten().for_each(|new_interface_impl: &'a<S as SchemaDefinition>::InterfaceImplementation| {
            if self.old_type.interface_implementations().map(|ii| ii.iter()).into_iter().flatten().find(|old_interface_impl| {
                old_interface_impl.interface().name() == new_interface_impl.interface().name()
            }).is_none() {
                added_interfaces.push(new_interface_impl.interface());
            }
        });

        added_interfaces
    }

    fn interface_removals(&self) -> Vec<&'a S::InterfaceTypeDefinition> {
        let mut removed_interfaces = Vec::new();

        self.old_type.interface_implementations().map(|ii| ii.iter()).into_iter().flatten().for_each(|old_interface_impl: &'a<S as SchemaDefinition>::InterfaceImplementation| {
            if self.new_type.interface_implementations().map(|ii| ii.iter()).into_iter().flatten().find(|new_interface_impl| {
                old_interface_impl.interface().name() == new_interface_impl.interface().name()
            }).is_none() {
                removed_interfaces.push(old_interface_impl.interface());
            }
        });

        removed_interfaces
    }
}

impl<'a, S: SchemaDefinition+'a> InputObjectType<'a, S> {
    pub fn new(old_type: &'a S::InputObjectTypeDefinition, new_type: &'a S::InputObjectTypeDefinition) -> Self {
        Self {
            old_type,
            new_type,
        }
    }

    pub fn diff(&self) -> Vec<Change<'a, S>> {
        let mut changes = Vec::new();

        changes.extend(self.field_additions().into_iter()
            .map(|field| Change::InputFieldAdded { added_field: field, input_object_type: self.new_type }));
        changes.extend(self.field_removals().into_iter()
            .map(|field| Change::InputFieldRemoved { removed_field: field, input_object_type: self.old_type }));

        self.old_type.input_field_definitions().iter().for_each(|old_field: &'a<S as SchemaDefinition>::InputValueDefinition| {
            let new_field: Option<&'a<S as SchemaDefinition>::InputValueDefinition> = self.new_type.input_field_definitions().get(old_field.name());

            if new_field.is_some() {
                changes.extend(InputField::new(self.old_type, self.new_type, old_field, new_field.unwrap()).diff());
            }
        });

        changes
    }

    fn field_additions(&self) -> Vec<&'a S::InputValueDefinition> {
        let mut added_fields: Vec<&'a<S as SchemaDefinition>::InputValueDefinition> = Vec::new();

        self.new_type.input_field_definitions().iter().for_each(|new_field: &'a<S as SchemaDefinition>::InputValueDefinition| {
            let old_field: Option<&'a<S as SchemaDefinition>::InputValueDefinition> = self.old_type.input_field_definitions().get(new_field.name());

            if old_field.is_none() {
                added_fields.push(new_field)
            }
        });

        added_fields
    }

    fn field_removals(&self) -> Vec<&'a S::InputValueDefinition> {
        let mut removed_fields: Vec<&'a<S as SchemaDefinition>::InputValueDefinition> = Vec::new();

        self.old_type.input_field_definitions().iter().for_each(|old_field: &'a<S as SchemaDefinition>::InputValueDefinition| {
            let new_field: Option<&'a<S as SchemaDefinition>::InputValueDefinition> = self.new_type.input_field_definitions().get(old_field.name());

            if new_field.is_none() {
                removed_fields.push(old_field)
            }
        });

        removed_fields
    }
}

impl<'a, S: SchemaDefinition+'a> Field<'a, S> {
    pub fn new(type_name: &'a str, old_field: &'a S::FieldDefinition, new_field: &'a S::FieldDefinition) -> Self {
        Self {
            type_name,
            old_field,
            new_field
        }
    }

    pub fn diff(&self) -> Vec<Change<'a, S>> {
        let mut changes = Vec::new();

        if self.old_field.description() != self.new_field.description() {
            changes.push(Change::FieldDescriptionChanged {
                type_name: self.type_name,
                old_field: self.old_field,
                new_field: self.new_field,
            });
        }

        if self.old_field.r#type().as_ref().display_name() != self.new_field.r#type().as_ref().display_name() {
            changes.push(Change::FieldTypeChanged {
                type_name: self.type_name,
                old_field: self.old_field,
                new_field: self.new_field,
            });
        }

        changes.extend(self.argument_additions().into_iter()
            .map(|arg| Change::FieldArgumentAdded { type_name: self.type_name, field: self.new_field, argument: arg }));

        changes.extend(self.argument_removals().into_iter()
            .map(|arg| Change::FieldArgumentRemoved { type_name: self.type_name, field: self.old_field, argument: arg }));

        changes
    }

    fn argument_additions(&self) -> Vec<&'a S::InputValueDefinition> {
        let mut added_arguments = Vec::new();

        self.new_field.arguments_definition().map(|ii| ii.iter()).into_iter().flatten().for_each(|new_arg: &'a<S as SchemaDefinition>::InputValueDefinition| {
            let old_arg = self.old_field.arguments_definition().map(|ii| ii.iter()).into_iter().flatten().find(|old_arg| {
                old_arg.name() == new_arg.name()
            });

            if old_arg.is_none() {
                added_arguments.push(new_arg);
            }
        });

        added_arguments
    }

    fn argument_removals(&self) -> Vec<&'a S::InputValueDefinition> {
        let mut removed_arguments: Vec<&<S as SchemaDefinition>::InputValueDefinition> = Vec::new();

        self.old_field.arguments_definition().map(|ii| ii.iter()).into_iter().flatten().for_each(|old_arg: &'a<S as SchemaDefinition>::InputValueDefinition| {
            let new_arg = self.new_field.arguments_definition().map(|ii| ii.iter()).into_iter().flatten().find(|new_arg| {
                old_arg.name() == new_arg.name()
            });

            if new_arg.is_none() {
                removed_arguments.push(old_arg);
            }
        });

        removed_arguments
    }
}

impl<'a, S: SchemaDefinition+'a> InputField<'a, S> {
    pub fn new(old_type: &'a S::InputObjectTypeDefinition, new_type: &'a S::InputObjectTypeDefinition, old_field: &'a S::InputValueDefinition, new_field: &'a S::InputValueDefinition) -> Self {
        Self {
            old_type,
            new_type,
            old_field,
            new_field
        }
    }

    pub fn diff(&self) -> Vec<Change<'a, S>> {
        let mut changes = Vec::new();

        if self.old_field.description() != self.new_field.description() {
            changes.push(Change::InputFieldDescriptionChanged {
                input_object_type: self.old_type,
                old_field: self.old_field,
                new_field: self.new_field,
            });
        }

        if self.old_field.r#type().as_ref().display_name() != self.new_field.r#type().as_ref().display_name() {
            changes.push(Change::InputFieldTypeChanged {
                input_object_type: self.old_type,
                old_field: self.old_field,
                new_field: self.new_field,
            });
        }

        match (self.old_field.default_value(), self.new_field.default_value()) {
            (Some(old_default), Some(new_default)) => {
                if old_default.as_ref() != new_default.as_ref() {
                    changes.push(Change::InputFieldDefaultValueChanged {
                        input_object_type: self.old_type,
                        old_field: self.old_field,
                        new_field: self.new_field,
                    });
                }
            },
            (Some(_), None) => {
                changes.push(Change::InputFieldDefaultValueChanged {
                    input_object_type: self.old_type,
                    old_field: self.old_field,
                    new_field: self.new_field,
                });
            },
            (None, Some(_)) => {
                changes.push(Change::InputFieldDefaultValueChanged {
                    input_object_type: self.old_type,
                    old_field: self.old_field,
                    new_field: self.new_field,
                });
            },
            (None, None) => { }
        }

        // TODO: directives

        changes
    }
}

impl<'a, S: SchemaDefinition+'a> Argument<'a, S> {
    pub fn new(type_name: &'a str, field: &'a S::FieldDefinition, old_argument: &'a S::InputValueDefinition, new_argument: &'a S::InputValueDefinition) -> Self {
        Self {
            type_name,
            field,
            old_argument,
            new_argument
        }
    }

    pub fn diff(&self) -> Vec<Change<'a, S>> {
        let mut changes = Vec::new();

        if self.old_argument.description() != self.new_argument.description() {
            changes.push(Change::FieldArgumentDescriptionChanged {
                type_name: self.type_name,
                field: self.field,
                old_argument: self.old_argument,
                new_argument: self.new_argument,
            });
        }

        if self.old_argument.r#type().as_ref().display_name() != self.new_argument.r#type().as_ref().display_name() {
            changes.push(Change::FieldArgumentTypeChanged {
                type_name: self.type_name,
                field: self.field,
                old_argument: self.old_argument,
                new_argument: self.new_argument,
            });
        }

        match (self.old_argument.default_value(), self.new_argument.default_value()) {
            (Some(old_default), Some(new_default)) => {
                if old_default.as_ref() != new_default.as_ref() {
                    changes.push(Change::FieldArgumentDefaultValueChanged {
                        type_name: self.type_name,
                        field: self.field,
                        old_argument: self.old_argument,
                        new_argument: self.new_argument,
                    });
                }
            },
            (Some(_), None) => {
                changes.push(Change::FieldArgumentDefaultValueChanged {
                    type_name: self.type_name,
                    field: self.field,
                    old_argument: self.old_argument,
                    new_argument: self.new_argument,
                });
            },
            (None, Some(_)) => {
                changes.push(Change::FieldArgumentDefaultValueChanged {
                    type_name: self.type_name,
                    field: self.field,
                    old_argument: self.old_argument,
                    new_argument: self.new_argument,
                });
            },
            (None, None) => { }
        }

        // TODO: directives
        changes
    }
}

impl<'a, S: SchemaDefinition+'a> Enum<'a, S> {
    pub fn new(old_type: &'a S::EnumTypeDefinition, new_type: &'a S::EnumTypeDefinition) -> Self {
        Self {
            old_type,
            new_type,

        }
    }

    pub fn diff(&self) -> Vec<Change<'a, S>> {
        let mut changes = Vec::new();

        changes.extend(self.value_additions().into_iter()
            .map(|arg| Change::EnumValueAdded { enum_type: self.new_type, enum_value: arg }));

        changes.extend(self.value_removals().into_iter()
            .map(|arg| Change::EnumValueRemoved { enum_type: self.old_type, enum_value: arg }));

        self.old_type.enum_value_definitions().iter().for_each(|old_enum_value| {
            let new_enum_value = self.new_type.enum_value_definitions().iter().find(|new_enum_value| {
                old_enum_value.name() == new_enum_value.name()
            });

            if new_enum_value.is_some() {
                if old_enum_value.description() != new_enum_value.unwrap().description() {
                    changes.push(Change::EnumValueDescriptionChanged {
                        enum_type: self.old_type,
                        old_enum_value,
                        new_enum_value: new_enum_value.unwrap(),
                    });
                }

                // TODO: directives
            }
        });

        changes
    }

    fn value_additions(&self) -> Vec<&'a S::EnumValueDefinition> {
        let mut added_values: Vec<&<S as SchemaDefinition>::EnumValueDefinition>= Vec::new();

        self.new_type.enum_value_definitions().iter().for_each(|new_enum_value| {
            if self.old_type.enum_value_definitions().iter().find(|old_enum_value| {
                old_enum_value.name() == new_enum_value.name()
            }).is_none() {
                added_values.push(new_enum_value);
            }
        });

        added_values
    }

    fn value_removals(&self) -> Vec<&'a S::EnumValueDefinition> {
        let mut removed_values: Vec<&<S as SchemaDefinition>::EnumValueDefinition> = Vec::new();

        self.old_type.enum_value_definitions().iter().for_each(|old_enum_value: &'a<S as SchemaDefinition>::EnumValueDefinition| {
            if self.new_type.enum_value_definitions().iter().find(|new_enum_value| {
                old_enum_value.name() == new_enum_value.name()
            }).is_none() {
                removed_values.push(old_enum_value);
            }
        });

        removed_values
    }
}

impl<'a, S: SchemaDefinition+'a> Union<'a, S> {
    pub fn new(old_type: &'a S::UnionTypeDefinition, new_type: &'a S::UnionTypeDefinition) -> Self {
        Self {
            old_type,
            new_type,

        }
    }

    pub fn diff(&self) -> Vec<Change<'a, S>> {
        let mut changes = Vec::new();

        changes.extend(self.member_additions().into_iter()
            .map(|arg| Change::UnionMemberAdded { union_type: self.new_type, union_member: arg }));

        changes.extend(self.member_removals().into_iter()
            .map(|arg| Change::UnionMemberRemoved { union_type: self.new_type, union_member: arg }));


        changes
    }

    fn member_removals(&self) -> Vec<&'a S::ObjectTypeDefinition> {
        let mut member_removals: Vec<&<S as SchemaDefinition>::ObjectTypeDefinition> = Vec::new();

        self.old_type.union_member_types().iter().for_each(|old_member_type| {
            if !self.new_type.union_member_types().contains_type(old_member_type.member_type().name()) {
                member_removals.push(old_member_type.member_type());
            }
        });

        member_removals
    }

    fn member_additions(&self) -> Vec<&'a S::ObjectTypeDefinition> {
        let mut member_additions: Vec<&<S as SchemaDefinition>::ObjectTypeDefinition> = Vec::new();

        self.new_type.union_member_types().iter().for_each(|new_member_type| {
            if !self.old_type.union_member_types().contains_type(new_member_type.member_type().name()) {
                member_additions.push(new_member_type.member_type());
            }
        });

        member_additions
    }
}

impl<'a, S: SchemaDefinition+'a> Interface<'a, S> {
    pub fn new(old_interface: &'a S::InterfaceTypeDefinition, new_interface: &'a S::InterfaceTypeDefinition) -> Self {
        Self {
            old_interface,
            new_interface,

        }
    }

    pub fn diff(&self) -> Vec<Change<'a, S>> {
        let mut changes: Vec<Change<'a, S>> = Vec::new();

        changes.extend(self.field_additions().into_iter()
            .map(|field| Change::FieldAdded { added_field: field, type_name: self.old_interface.name() }));
        changes.extend(self.field_removals().into_iter()
            .map(|field| Change::FieldRemoved { removed_field: field, type_name: self.new_interface.name() }));

        self.old_interface.fields_definition().iter().for_each(|old_field: &'a<S as SchemaDefinition>::FieldDefinition| {
            let new_field: Option<&'a<S as SchemaDefinition>::FieldDefinition> = self.new_interface.fields_definition().get(old_field.name());

            if new_field.is_some() {
                changes.extend(Field::new(self.new_interface.name(), old_field, new_field.unwrap()).diff());
            }
        });

        changes
    }

    fn field_additions(&self) -> Vec<&'a S::FieldDefinition> {
        let mut added_fields: Vec<&'a<S as SchemaDefinition>::FieldDefinition> = Vec::new();

        self.new_interface.fields_definition().iter().for_each(|field: &'a<S as SchemaDefinition>::FieldDefinition| {
            if !self.old_interface.fields_definition().contains_field(field.name()) {
                added_fields.push(field);
            }
        });

        added_fields
    }

    fn field_removals(&self) -> Vec<&'a S::FieldDefinition> {
        let mut removed_fields: Vec<&'a<S as SchemaDefinition>::FieldDefinition> = Vec::new();

        self.old_interface.fields_definition().iter().for_each(|field: &'a<S as SchemaDefinition>::FieldDefinition| {
            if !self.new_interface.fields_definition().contains_field(field.name()) {
                removed_fields.push(field);
            }
        });

        removed_fields
    }
}

impl<'a, S: SchemaDefinition+'a> Directive<'a, S> {
    pub fn new(old_directive: &'a S::DirectiveDefinition, new_directive: &'a S::DirectiveDefinition) -> Self {
        Self {
            old_directive,
            new_directive,
        }
    }

    pub fn diff(&self) -> Vec<Change<'a, S>> {
        let mut changes: Vec<Change<'a, S>> = Vec::new();

        if self.old_directive.description() != self.new_directive.description() {
            changes.push(Change::DirectiveDescriptionChanged {
                old_directive: self.old_directive,
                new_directive: self.new_directive,
            });
        }

        changes.extend(self.added_locations().into_iter()
            .map(|location| Change::DirectiveLocationAdded {
                directive: self.new_directive,
                location: location,
            }));

        changes.extend(self.removed_locations().into_iter()
            .map(|location| Change::DirectiveLocationRemoved {
                directive: self.new_directive,
                location: location,
            }));

        changes.extend(self.added_arguments().into_iter()
            .map(|argument| Change::DirectiveArgumentAdded {
                directive: self.new_directive,
                argument: argument,
            }));

        changes.extend(self.removed_arguments().into_iter()
            .map(|argument| Change::DirectiveArgumentRemoved {
                directive: self.new_directive,
                argument: argument,
            }));

        self.old_directive.arguments_definition().map(|ii| ii.iter()).into_iter().flatten().for_each(|old_argument: &'a<S as SchemaDefinition>::InputValueDefinition| {
            let new_argument: Option<&'a<S as SchemaDefinition>::InputValueDefinition> = self.new_directive.arguments_definition().map(|ii| ii.iter()).into_iter().flatten().find(|new_argument| {
                old_argument.name() == new_argument.name()
            });

            if new_argument.is_some() {
                changes.extend(DirectiveArgument::new(self.new_directive, old_argument, new_argument.unwrap()).diff());
            }
        });

        changes
    }

    fn removed_locations(&self) -> Vec<&'a DirectiveLocation> {
        let mut removed_locations = Vec::new();

        self.old_directive.locations().iter().for_each(|old_location| {
            if self.new_directive.locations().iter().find(|new_location| {
                *new_location == old_location
            }).is_none() {
                removed_locations.push(old_location);
            }
        });

        removed_locations
    }

    fn added_locations(&self) -> Vec<&'a DirectiveLocation> {
        let mut added_locations = Vec::new();

        self.new_directive.locations().iter().for_each(|new_location: &DirectiveLocation| {
            if self.old_directive.locations().iter().find(|old_location| {
                *old_location == new_location
            }).is_none() {
                added_locations.push(new_location);
            }
        });

        added_locations
    }

    fn removed_arguments(&self) -> Vec<&'a S::InputValueDefinition> {
        let mut removed_arguments = Vec::new();

        self.old_directive.arguments_definition().map(|ii| ii.iter()).into_iter().flatten().for_each(|old_argument: &'a<S as SchemaDefinition>::InputValueDefinition| {
            if self.new_directive.arguments_definition().map(|ii| ii.iter()).into_iter().flatten().find(|new_argument| {
                old_argument.name() == new_argument.name()
            }).is_none() {
                removed_arguments.push(old_argument);
            }
        });

        removed_arguments
    }

    fn added_arguments(&self) -> Vec<&'a S::InputValueDefinition> {
        let mut added_arguments = Vec::new();

        self.new_directive.arguments_definition().map(|ii| ii.iter()).into_iter().flatten().for_each(|new_argument: &'a<S as SchemaDefinition>::InputValueDefinition| {
            if self.old_directive.arguments_definition().map(|ii| ii.iter()).into_iter().flatten().find(|old_argument| {
                old_argument.name() == new_argument.name()
            }).is_none() {
                added_arguments.push(new_argument);
            }
        });

        added_arguments
    }
}

impl<'a, S: SchemaDefinition+'a> DirectiveArgument<'a, S> {
    pub fn new(directive: &'a S::DirectiveDefinition, old_argument: &'a S::InputValueDefinition, new_argument: &'a S::InputValueDefinition) -> Self {
        Self {
            directive,
            old_argument,
            new_argument
        }
    }

    pub fn diff(&self) -> Vec<Change<'a, S>> {
        let mut changes = Vec::new();

        if self.old_argument.description() != self.new_argument.description() {
            changes.push(Change::DirectiveArgumentDescriptionChanged {
                directive: self.directive,
                old_argument: self.old_argument,
                new_argument: self.new_argument,
            });
        }

        if self.old_argument.r#type().as_ref().display_name() != self.new_argument.r#type().as_ref().display_name() {
            changes.push(Change::DirectiveArgumentTypeChanged {
                directive: self.directive,
                old_argument: self.old_argument,
                new_argument: self.new_argument,
            });
        }

        match (self.old_argument.default_value(), self.new_argument.default_value()) {
            (Some(old_default), Some(new_default)) => {
                if old_default.as_ref() != new_default.as_ref() {
                    changes.push(Change::DirectiveArgumentDefaultValueChanged {
                        directive: self.directive,
                        old_argument: self.old_argument,
                        new_argument: self.new_argument,
                    });
                }
            },
            (Some(_), None) => {
                changes.push(Change::DirectiveArgumentDefaultValueChanged {
                    directive: self.directive,
                    old_argument: self.old_argument,
                    new_argument: self.new_argument,
                });
            },
            (None, Some(_)) => {
                changes.push(Change::DirectiveArgumentDefaultValueChanged {
                    directive: self.directive,
                    old_argument: self.old_argument,
                    new_argument: self.new_argument,
                });
            },
            (None, None) => { }
        }

        changes
    }
}
