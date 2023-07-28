use crate::ruby_api::{HasVisibility, SchemaDefinition};
use bluejay_core::definition::SchemaDefinition as CoreSchemaDefinition;
use magnus::{Error, Value};
use std::cell::RefCell;
use std::collections::HashMap;

pub struct Warden {
    context: Value,
    cache: RefCell<HashMap<String, bool>>,
    visibility_error: RefCell<Option<Error>>,
}

impl Warden {
    pub(crate) fn new(context: Value) -> Self {
        Self {
            context,
            cache: Default::default(),
            visibility_error: Default::default(),
        }
    }

    fn evaluate_visibility(&self, item: &impl HasVisibility) -> bool {
        item.visibility().map_or(true, |visibility| {
            let cache_key = match visibility.cache_key() {
                Ok(s) => s,
                Err(error) => {
                    self.report_error(error);
                    return false;
                }
            };

            if let Some(cached) = self.cache.borrow().get(cache_key).cloned() {
                return cached;
            }

            let is_visible = match visibility.is_visible(self.context) {
                Ok(is_visible) => is_visible,
                Err(error) => {
                    self.report_error(error);
                    return false;
                }
            };

            self.cache
                .borrow_mut()
                .insert(cache_key.to_string(), is_visible);

            is_visible
        })
    }

    pub(crate) fn to_result(&self) -> Result<(), Error> {
        self.visibility_error
            .borrow_mut()
            .take()
            .map_or(Ok(()), Err)
    }

    pub(crate) fn report_error(&self, error: Error) {
        self.visibility_error.borrow_mut().get_or_insert(error);
    }

    pub(crate) fn context(&self) -> Value {
        self.context
    }
}

impl bluejay_visibility::Warden for Warden {
    type SchemaDefinition = SchemaDefinition;

    fn is_field_definition_visible(
        &self,
        field_definition: &<Self::SchemaDefinition as CoreSchemaDefinition>::FieldDefinition,
    ) -> bool {
        self.evaluate_visibility(field_definition)
    }

    fn is_input_value_definition_visible(
        &self,
        input_value_definition: &<Self::SchemaDefinition as CoreSchemaDefinition>::InputValueDefinition,
    ) -> bool {
        self.evaluate_visibility(input_value_definition)
    }

    fn is_enum_value_definition_visible(
        &self,
        enum_value_definition: &<Self::SchemaDefinition as CoreSchemaDefinition>::EnumValueDefinition,
    ) -> bool {
        self.evaluate_visibility(enum_value_definition)
    }

    fn is_union_member_type_visible(
        &self,
        union_member_type: &<Self::SchemaDefinition as CoreSchemaDefinition>::UnionMemberType,
    ) -> bool {
        self.evaluate_visibility(union_member_type)
    }

    fn is_interface_implementation_visible(
        &self,
        interface_implementation: &<Self::SchemaDefinition as CoreSchemaDefinition>::InterfaceImplementation,
    ) -> bool {
        self.evaluate_visibility(interface_implementation)
    }

    fn is_directive_definition_visible(
        &self,
        directive_definition: &<Self::SchemaDefinition as CoreSchemaDefinition>::DirectiveDefinition,
    ) -> bool {
        self.evaluate_visibility(directive_definition)
    }

    fn is_custom_scalar_type_definition_visible(
        &self,
        custom_scalar_type_definition: &<Self::SchemaDefinition as CoreSchemaDefinition>::CustomScalarTypeDefinition,
    ) -> bool {
        self.evaluate_visibility(custom_scalar_type_definition)
    }

    fn is_enum_type_definition_visible(
        &self,
        enum_type_definition: &<Self::SchemaDefinition as CoreSchemaDefinition>::EnumTypeDefinition,
    ) -> bool {
        self.evaluate_visibility(enum_type_definition)
    }

    fn is_input_object_type_definition_visible(
        &self,
        input_object_type_definition: &<Self::SchemaDefinition as CoreSchemaDefinition>::InputObjectTypeDefinition,
    ) -> bool {
        self.evaluate_visibility(input_object_type_definition)
    }

    fn is_interface_type_definition_visible(
        &self,
        interface_type_definition: &<Self::SchemaDefinition as CoreSchemaDefinition>::InterfaceTypeDefinition,
    ) -> bool {
        self.evaluate_visibility(interface_type_definition)
    }

    fn is_object_type_definition_visible(
        &self,
        object_type_definition: &<Self::SchemaDefinition as CoreSchemaDefinition>::ObjectTypeDefinition,
    ) -> bool {
        self.evaluate_visibility(object_type_definition)
    }

    fn is_union_type_definition_visible(
        &self,
        union_type_definition: &<Self::SchemaDefinition as CoreSchemaDefinition>::UnionTypeDefinition,
    ) -> bool {
        self.evaluate_visibility(union_type_definition)
    }
}
