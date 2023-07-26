use crate::ruby_api::{SchemaDefinition, Visibility};
use bluejay_core::definition::SchemaDefinition as CoreSchemaDefinition;
use magnus::Value;
use std::cell::RefCell;
use std::collections::HashMap;

pub struct Warden {
    context: Value,
    cache: RefCell<HashMap<String, bool>>,
}

impl Warden {
    pub(crate) fn new(context: Value) -> Self {
        Self {
            context,
            cache: Default::default(),
        }
    }

    fn evaluate_visibility(&self, visibility: Option<&Visibility>) -> bool {
        visibility.map_or(true, |visibility| {
            if let Some(cached) = self.cache.borrow().get(visibility.cache_key()).cloned() {
                return cached;
            }

            // TODO: error handling
            let is_visible = visibility.is_visible(self.context).unwrap();

            self.cache
                .borrow_mut()
                .insert(visibility.cache_key().to_string(), is_visible);

            is_visible
        })
    }
}

impl bluejay_visibility::Warden for Warden {
    type SchemaDefinition = SchemaDefinition;

    fn is_field_definition_visible(
        &self,
        field_definition: &<Self::SchemaDefinition as CoreSchemaDefinition>::FieldDefinition,
    ) -> bool {
        self.evaluate_visibility(field_definition.visibility())
    }

    fn is_input_value_definition_visible(
        &self,
        input_value_definition: &<Self::SchemaDefinition as CoreSchemaDefinition>::InputValueDefinition,
    ) -> bool {
        self.evaluate_visibility(input_value_definition.visibility())
    }

    fn is_enum_value_definition_visible(
        &self,
        _: &<Self::SchemaDefinition as CoreSchemaDefinition>::EnumValueDefinition,
    ) -> bool {
        true
    }

    fn is_union_member_type_visible(
        &self,
        _: &<Self::SchemaDefinition as CoreSchemaDefinition>::UnionMemberType,
    ) -> bool {
        true
    }

    fn is_interface_implementation_visible(
        &self,
        _: &<Self::SchemaDefinition as CoreSchemaDefinition>::InterfaceImplementation,
    ) -> bool {
        true
    }

    fn is_directive_definition_visible(
        &self,
        _: &<Self::SchemaDefinition as CoreSchemaDefinition>::DirectiveDefinition,
    ) -> bool {
        true
    }

    fn is_custom_scalar_type_definition_visible(
        &self,
        _: &<Self::SchemaDefinition as CoreSchemaDefinition>::CustomScalarTypeDefinition,
    ) -> bool {
        true
    }

    fn is_enum_type_definition_visible(
        &self,
        _: &<Self::SchemaDefinition as CoreSchemaDefinition>::EnumTypeDefinition,
    ) -> bool {
        true
    }

    fn is_input_object_type_definition_visible(
        &self,
        _: &<Self::SchemaDefinition as CoreSchemaDefinition>::InputObjectTypeDefinition,
    ) -> bool {
        true
    }

    fn is_interface_type_definition_visible(
        &self,
        _: &<Self::SchemaDefinition as CoreSchemaDefinition>::InterfaceTypeDefinition,
    ) -> bool {
        true
    }

    fn is_object_type_definition_visible(
        &self,
        _: &<Self::SchemaDefinition as CoreSchemaDefinition>::ObjectTypeDefinition,
    ) -> bool {
        true
    }

    fn is_union_type_definition_visible(
        &self,
        _: &<Self::SchemaDefinition as CoreSchemaDefinition>::UnionTypeDefinition,
    ) -> bool {
        true
    }
}
