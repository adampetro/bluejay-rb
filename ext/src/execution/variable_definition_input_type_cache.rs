use crate::ruby_api::{BaseInputType, InputType, SchemaDefinition};
use bluejay_core::executable::VariableType as _;
use bluejay_parser::ast::executable::VariableType;
use magnus::{typed_data::Obj, RArray};

pub struct VariableDefinitionInputTypeCache {
    rarray: RArray,
}

impl VariableDefinitionInputTypeCache {
    pub(crate) fn new() -> Self {
        Self {
            rarray: RArray::new(),
        }
    }

    pub(crate) fn input_type_for_variable_definition<'a, 'b: 'a>(
        &'a self,
        schema_definition: &'b SchemaDefinition,
        variable_type: &VariableType,
    ) -> &'a InputType {
        let variable_base_type = schema_definition
            .r#type(variable_type.as_ref().name())
            .unwrap();
        let base_input_type_reference: BaseInputType = variable_base_type.try_into().unwrap();
        let variable_type =
            InputType::from_parser_variable_type(variable_type, base_input_type_reference);
        let wrapped = Obj::wrap(variable_type);
        self.rarray.push(wrapped).unwrap();
        self.rarray.entry(self.rarray.len() as isize - 1).unwrap()
    }
}
