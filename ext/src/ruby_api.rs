use magnus::{define_module, function, memoize, Error, RModule};

mod arguments_definition;
mod coerce_input;
mod coercion_error;
mod custom_scalar_type_definition;
mod directive;
mod directive_definition;
mod directive_location;
mod directives;
mod enum_type_definition;
mod enum_value_definition;
mod enum_value_definitions;
mod execution_error;
mod execution_result;
mod field_definition;
mod fields_definition;
mod input_fields_definition;
mod input_object_type_definition;
mod input_type_reference;
mod input_value_definition;
mod interface_implementation;
mod interface_implementations;
mod interface_type_definition;
mod object_type_definition;
mod output_type_reference;
mod r_result;
mod scalar;
mod schema_definition;
mod union_member_type;
mod union_member_types;
mod union_type_definition;
mod validation_error;
mod wrapped_value;

pub use arguments_definition::ArgumentsDefinition;
pub use coerce_input::CoerceInput;
pub use coercion_error::CoercionError;
pub use directive::Directive;
pub use directive_definition::DirectiveDefinition;
pub use directive_location::DirectiveLocation;
pub use directives::Directives;
pub use execution_error::ExecutionError;
pub use execution_result::ExecutionResult;
pub use field_definition::FieldDefinition;
pub use input_object_type_definition::InputObjectTypeDefinition;
pub use input_type_reference::{BaseInputTypeReference, InputTypeReference};
pub use input_value_definition::InputValueDefinition;
pub use interface_type_definition::InterfaceTypeDefinition;
pub use object_type_definition::ObjectTypeDefinition;
pub use output_type_reference::{BaseOutputTypeReference, OutputTypeReference};
pub use r_result::RResult;
pub use schema_definition::{SchemaDefinition, TypeDefinitionReference};
pub use union_type_definition::UnionTypeDefinition;
pub use wrapped_value::WrappedValue;

pub fn root() -> RModule {
    *memoize!(RModule: define_module("Bluejay").unwrap())
}

pub fn init() -> Result<(), Error> {
    let r = root();

    coercion_error::init()?;
    custom_scalar_type_definition::init()?;
    directive_definition::init()?;
    directive_location::init()?;
    enum_type_definition::init()?;
    enum_value_definition::init()?;
    execution_error::init()?;
    execution_result::init()?;
    field_definition::init()?;
    input_object_type_definition::init()?;
    input_type_reference::init()?;
    input_value_definition::init()?;
    interface_implementation::init()?;
    interface_type_definition::init()?;
    object_type_definition::init()?;
    output_type_reference::init()?;
    r_result::init()?;
    scalar::init()?;
    schema_definition::init()?;
    union_member_type::init()?;
    union_type_definition::init()?;
    validation_error::init()?;
    r.define_module_function(
        "parse",
        function!(
            |s: String| {
                let (doc, errs) =
                    bluejay_parser::ast::executable::ExecutableDocument::parse(s.as_str());
                (doc.operation_definitions().len() + doc.fragment_definitions().len()) > 0
                    && errs.is_empty()
            },
            1
        ),
    )?;

    Ok(())
}
