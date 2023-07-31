use bluejay_core::definition::{SchemaDefinition, TypeDefinitionReference, ObjectTypeDefinition, FieldDefinition, FieldsDefinition, ArgumentsDefinition, InputValueDefinition, OutputType, InterfaceImplementation, InterfaceTypeDefinition, InputType, EnumTypeDefinition, EnumValueDefinition};
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

pub struct Field<'a, S: SchemaDefinition> {
    old_type: &'a S::ObjectTypeDefinition,
    new_type: &'a S::ObjectTypeDefinition,
    old_field: &'a S::FieldDefinition,
    new_field: &'a S::FieldDefinition,
}

pub struct Argument<'a, S: SchemaDefinition> {
    object_type: &'a S::ObjectTypeDefinition,
    field: &'a S::FieldDefinition,
    old_argument: &'a S::InputValueDefinition,
    new_argument: &'a S::InputValueDefinition,
}

pub struct Enum<'a, S: SchemaDefinition> {
    old_type: &'a S::EnumTypeDefinition,
    new_type: &'a S::EnumTypeDefinition,
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
            .map(|field| Change::FieldAdded { added_field: field, object_type: self.old_type }));
        changes.extend(self.field_removals().into_iter()
            .map(|field| Change::FieldRemoved { removed_field: field, object_type: self.new_type }));

        self.old_type.fields_definition().iter().for_each(|old_field: &'a<S as SchemaDefinition>::FieldDefinition| {
            let new_field: Option<&'a<S as SchemaDefinition>::FieldDefinition> = self.new_type.fields_definition().get(old_field.name());

            if new_field.is_some() {
                changes.extend(Field::new(self.old_type, self.new_type, old_field, new_field.unwrap()).diff());
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

impl<'a, S: SchemaDefinition+'a> Field<'a, S> {
    pub fn new(old_type: &'a S::ObjectTypeDefinition, new_type: &'a S::ObjectTypeDefinition, old_field: &'a S::FieldDefinition, new_field: &'a S::FieldDefinition) -> Self {
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
            changes.push(Change::FieldDescriptionChanged {
                object_type: self.old_type,
                old_field: self.old_field,
                new_field: self.new_field,
            });
        }

        if self.old_field.r#type().as_ref().display_name() != self.new_field.r#type().as_ref().display_name() {
            changes.push(Change::FieldTypeChanged {
                object_type: self.old_type,
                old_field: self.old_field,
                new_field: self.new_field,
            });
        }

        changes.extend(self.argument_additions().into_iter()
            .map(|arg| Change::FieldArgumentAdded { object_type: self.new_type, field: self.new_field, argument: arg }));

        changes.extend(self.argument_removals().into_iter()
            .map(|arg| Change::FieldArgumentRemoved { object_type: self.new_type, field: self.old_field, argument: arg }));

        changes
    }

    fn argument_additions(&self) -> Vec<&'a S::InputValueDefinition> {
        let mut added_arguments = Vec::new();

        self.new_field.arguments_definition().map(|ii| ii.iter()).into_iter().flatten().for_each(|new_arg: &'a<S as SchemaDefinition>::InputValueDefinition| {
            if self.old_field.arguments_definition().unwrap().get(new_arg.name()).is_some() {
                added_arguments.push(new_arg);
            }
        });

        added_arguments
    }

    fn argument_removals(&self) -> Vec<&'a S::InputValueDefinition> {
        let mut removed_arguments: Vec<&<S as SchemaDefinition>::InputValueDefinition> = Vec::new();

        self.old_field.arguments_definition().map(|ii| ii.iter()).into_iter().flatten().for_each(|old_arg: &'a<S as SchemaDefinition>::InputValueDefinition| {
            if self.new_field.arguments_definition().map(|ii| ii.iter()).into_iter().flatten().find(|new_arg| {
                old_arg.name() == new_arg.name()
            }).is_none() {
                removed_arguments.push(old_arg);
            }
        });

        removed_arguments
    }
}

impl<'a, S: SchemaDefinition+'a> Argument<'a, S> {
    pub fn new(object_type: &'a S::ObjectTypeDefinition, field: &'a S::FieldDefinition, old_argument: &'a S::InputValueDefinition, new_argument: &'a S::InputValueDefinition) -> Self {
        Self {
            object_type,
            field,
            old_argument,
            new_argument
        }
    }

    pub fn diff(&self) -> Vec<Change<'a, S>> {
        let mut changes = Vec::new();

        if self.old_argument.description() != self.new_argument.description() {
            changes.push(Change::FieldArgumentDescriptionChanged {
                object_type: self.object_type,
                field: self.field,
                old_argument: self.old_argument,
                new_argument: self.new_argument,
            });
        }

        if self.old_argument.r#type().as_ref().display_name() != self.new_argument.r#type().as_ref().display_name() {
            changes.push(Change::FieldArgumentTypeChanged {
                object_type: self.object_type,
                field: self.field,
                old_argument: self.old_argument,
                new_argument: self.new_argument,
            });
        }

        match (self.old_argument.default_value(), self.new_argument.default_value()) {
            (Some(old_default), Some(new_default)) => {
                if old_default.as_ref() != new_default.as_ref() {
                    changes.push(Change::FieldArgumentDefaultValueChanged {
                        object_type: self.object_type,
                        field: self.field,
                        old_argument: self.old_argument,
                        new_argument: self.new_argument,
                    });
                }
            },
            (Some(_), None) => {
                changes.push(Change::FieldArgumentDefaultValueChanged {
                    object_type: self.object_type,
                    field: self.field,
                    old_argument: self.old_argument,
                    new_argument: self.new_argument,
                });
            },
            (None, Some(_)) => {
                changes.push(Change::FieldArgumentDefaultValueChanged {
                    object_type: self.object_type,
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
