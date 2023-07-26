use magnus::{define_module, function, memoize, Error, Module, RModule};

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
mod errors;
mod execution_error;
mod execution_result;
mod field_definition;
mod fields_definition;
mod input_fields_definition;
mod input_object_type_definition;
mod input_type;
mod input_value_definition;
mod interface_implementation;
mod interface_implementations;
mod interface_type_definition;
mod introspection;
mod object_type_definition;
mod output_type;
mod r_result;
mod scalar;
mod schema_definition;
mod type_definition;
mod union_member_type;
mod union_member_types;
mod union_type_definition;
mod validation_error;
mod visibility;
mod wrapped_value;

pub use arguments_definition::ArgumentsDefinition;
pub use coerce_input::CoerceInput;
pub use coercion_error::CoercionError;
pub use custom_scalar_type_definition::CustomScalarTypeDefinition;
pub use directive::Directive;
pub use directive_definition::DirectiveDefinition;
pub use directive_location::DirectiveLocation;
pub use directives::Directives;
pub use enum_type_definition::EnumTypeDefinition;
pub use enum_value_definition::EnumValueDefinition;
pub use enum_value_definitions::EnumValueDefinitions;
pub use errors::{base_error, non_unique_definition_name_error};
pub use execution_error::ExecutionError;
pub use execution_result::ExecutionResult;
pub use field_definition::{ExtraResolverArg, FieldDefinition};
pub use fields_definition::FieldsDefinition;
pub use input_fields_definition::InputFieldsDefinition;
pub use input_object_type_definition::InputObjectTypeDefinition;
pub use input_type::{BaseInputType, InputType};
pub use input_value_definition::InputValueDefinition;
pub use interface_implementation::InterfaceImplementation;
pub use interface_implementations::InterfaceImplementations;
pub use interface_type_definition::InterfaceTypeDefinition;
pub use object_type_definition::ObjectTypeDefinition;
pub use output_type::{BaseOutputType, OutputType};
pub use r_result::RResult;
pub use scalar::Scalar;
pub use schema_definition::SchemaDefinition;
pub use type_definition::TypeDefinition;
pub use union_member_type::UnionMemberType;
pub use union_member_types::UnionMemberTypes;
pub use union_type_definition::UnionTypeDefinition;
pub use validation_error::ValidationError;
pub use visibility::{HasVisibility, Visibility};
pub use wrapped_value::WrappedValue;

pub fn root() -> RModule {
    *memoize!(RModule: define_module("Bluejay").unwrap())
}

pub fn base() -> RModule {
    *memoize!(RModule: root().define_module("Base").unwrap())
}

pub fn errors() -> RModule {
    *memoize!(RModule: root().define_module("Errors").unwrap())
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
    input_type::init()?;
    input_value_definition::init()?;
    interface_implementation::init()?;
    interface_type_definition::init()?;
    object_type_definition::init()?;
    output_type::init()?;
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
                bluejay_parser::ast::executable::ExecutableDocument::parse(s.as_str())
                    .map(|doc| {
                        (doc.operation_definitions().len() + doc.fragment_definitions().len()) > 0
                    })
                    .unwrap_or(false)
            },
            1
        ),
    )?;

    Ok(())
}
