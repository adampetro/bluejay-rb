use super::root;
use bluejay_core::{
    definition::{
        DirectiveDefinition, FieldDefinition, InputType, InputValueDefinition, OutputType,
        SchemaDefinition,
    },
    executable::{OperationDefinition, VariableType},
    AsIter, OperationType,
};
use bluejay_parser::ast::{executable::ExecutableDocument, Value as ParserValue};
use bluejay_validator::executable::{ArgumentError, DirectiveError, Error as CoreError};
use bluejay_validator::value::input_coercion::Error as InputCoercionError;
use itertools::Itertools;
use magnus::{
    function, method,
    rb_sys::AsRawValue,
    typed_data::{self, Obj},
    Error, Module, Object,
};
use std::borrow::Cow;

#[derive(Clone, Debug, PartialEq, Eq)]
#[magnus::wrap(class = "Bluejay::ValidationError")]
pub struct ValidationError {
    message: Cow<'static, str>,
}

impl ValidationError {
    pub fn new(message: impl Into<Cow<'static, str>>) -> Self {
        Self {
            message: message.into(),
        }
    }

    pub fn message(&self) -> &str {
        self.message.as_ref()
    }

    fn inspect(rb_self: Obj<Self>) -> Result<String, Error> {
        let rs_self = rb_self.get();

        Ok(format!(
            "#<Bluejay::ValidationError:0x{:016x} @message={:?}>",
            rb_self.as_raw(),
            rs_self.message,
        ))
    }
}

impl From<String> for ValidationError {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl<'a, S: SchemaDefinition> From<CoreError<'a, ExecutableDocument<'a>, S>> for ValidationError {
    fn from(value: CoreError<'a, ExecutableDocument<'a>, S>) -> Self {
        match value {
            CoreError::NotLoneAnonymousOperation { .. } => Self::new(
                "Anonymous operations are not allowed when there is more than one operation",
            ),
            CoreError::NonUniqueOperationNames { name, .. } => {
                Self::new(format!("More than one operation named `{name}`"))
            }
            CoreError::SubscriptionRootNotSingleField { .. } => {
                Self::new("Subscription operations can only select one field at the root")
            }
            CoreError::FieldDoesNotExistOnType { field, r#type } => Self::new(format!(
                "Field `{}` does not exist on `{}`",
                field.name().as_ref(),
                r#type.name()
            )),
            CoreError::OperationTypeNotDefined { operation } => Self::new(format!(
                "Schema does not define a {} root",
                OperationType::from(operation.operation_type()),
            )),
            CoreError::LeafFieldSelectionNotEmpty { r#type, .. } => Self::new(format!(
                "Selection on field of leaf type `{}` was not empty",
                r#type.as_ref().display_name()
            )),
            CoreError::NonLeafFieldSelectionEmpty { r#type, .. } => Self::new(format!(
                "No selection on field of non-leaf type `{}`",
                r#type.as_ref().display_name()
            )),
            CoreError::NonUniqueFragmentDefinitionNames { name, .. } => {
                Self::new(format!("Multiple fragment definitions named `{name}`"))
            }
            CoreError::FragmentDefinitionTargetTypeDoesNotExist {
                fragment_definition,
            } => Self::new(format!(
                "No type definition with name `{}`",
                fragment_definition.type_condition().named_type().as_ref()
            )),
            CoreError::InlineFragmentTargetTypeDoesNotExist { inline_fragment } => {
                Self::new(format!(
                    "No type definition with name `{}`",
                    inline_fragment
                        .type_condition()
                        .map(|tc| tc.named_type().as_ref())
                        .unwrap_or_default()
                ))
            }
            CoreError::FragmentDefinitionTargetTypeNotComposite {
                fragment_definition,
            } => Self::new(format!(
                "`{}` is not a composite type",
                fragment_definition.type_condition().named_type().as_ref()
            )),
            CoreError::InlineFragmentTargetTypeNotComposite { inline_fragment } => {
                Self::new(format!(
                    "`{}` is not a composite type",
                    inline_fragment
                        .type_condition()
                        .map(|tc| tc.named_type().as_ref())
                        .unwrap_or_default()
                ))
            }
            CoreError::FragmentDefinitionUnused {
                fragment_definition,
            } => Self::new(format!(
                "Fragment definition `{}` is unused",
                fragment_definition.name().as_ref()
            )),
            CoreError::FragmentSpreadTargetUndefined { fragment_spread } => Self::new(format!(
                "No fragment defined with name `{}`",
                fragment_spread.name().as_ref()
            )),
            CoreError::FragmentSpreadCycle {
                fragment_definition,
                ..
            } => Self::new(format!(
                "Cycle detected in fragment `{}`",
                fragment_definition.name().as_ref()
            )),
            CoreError::FieldSelectionsDoNotMergeDifferingArguments { .. } => {
                Self::new("Fields in selection set do not merge due to unequal arguments")
            }
            CoreError::FieldSelectionsDoNotMergeDifferingNames { .. } => {
                Self::new("Fields in selection set do not merge due to unequal field names")
            }
            CoreError::FieldSelectionsDoNotMergeIncompatibleTypes { .. } => {
                Self::new("Fields in selection set do not merge due to incompatible types")
            }
            CoreError::FragmentSpreadIsNotPossible {
                fragment_spread,
                parent_type,
            } => Self::new(format!(
                "Fragment `{}` cannot be spread for type {}",
                fragment_spread.name().as_ref(),
                parent_type.name()
            )),
            CoreError::InlineFragmentSpreadIsNotPossible {
                inline_fragment,
                parent_type,
            } => Self::new(format!(
                "Fragment targeting type {} cannot be spread for type {}",
                inline_fragment
                    .type_condition()
                    .map(|type_condition| type_condition.named_type().as_ref())
                    .unwrap_or_else(|| parent_type.name()),
                parent_type.name(),
            )),
            CoreError::InvalidConstValue(error) => Self::from(error),
            CoreError::InvalidVariableValue(error) => Self::from(error),
            CoreError::InvalidConstArgument(error) => Self::from(error),
            CoreError::InvalidVariableArgument(error) => Self::from(error),
            CoreError::InvalidConstDirective(error) => Self::from(error),
            CoreError::InvalidVariableDirective(error) => Self::from(error),
            CoreError::NonUniqueVariableDefinitionNames { name, .. } => {
                Self::new(format!("Multiple variable definitions named ${name}"))
            }
            CoreError::VariableDefinitionTypeNotInput {
                variable_definition,
            } => Self::new(format!(
                "Type of variable ${}, {}, is not an input type",
                variable_definition.variable().name(),
                variable_definition.r#type().as_ref().name()
            )),
            CoreError::VariableNotDefined {
                variable,
                operation_definition,
            } => {
                let operation_name = match operation_definition.as_ref().name() {
                    Some(name) => Cow::Owned(format!("operation {name}")),
                    None => Cow::Borrowed("anonymous operation"),
                };
                Self::new(format!(
                    "Variable ${} not defined in {operation_name}",
                    variable.name(),
                ))
            }
            CoreError::VariableDefinitionUnused {
                variable_definition,
            } => Self::new(format!(
                "Variable definition ${} not used",
                variable_definition.variable().name(),
            )),
            CoreError::InvalidVariableUsage {
                variable,
                variable_type,
                location_type,
            } => Self::new(format!(
                "Variable ${} of type {} cannot be used here, where {} is expected",
                variable.name(),
                variable_type.as_ref().display_name(),
                location_type.as_ref().display_name(),
            )),
        }
    }
}

impl<'a, const CONST: bool> From<InputCoercionError<'a, CONST, ParserValue<'a, CONST>>>
    for ValidationError
{
    fn from(value: InputCoercionError<'a, CONST, ParserValue<'a, CONST>>) -> Self {
        match value {
            InputCoercionError::NullValueForRequiredType {
                input_type_name, ..
            } => Self::new(format!(
                "Got null when non-null value of type {input_type_name} was expected"
            )),
            InputCoercionError::NoImplicitConversion {
                value,
                input_type_name,
                ..
            } => Self::new(format!(
                "No implicit conversion of {value} to {input_type_name}"
            )),
            InputCoercionError::NoEnumMemberWithName {
                name,
                enum_type_name,
                ..
            } => Self::new(format!("No member `{name}` on enum {enum_type_name}")),
            InputCoercionError::NoValueForRequiredFields {
                field_names,
                input_object_type_name,
                ..
            } => Self::new(format!(
                "No value for required fields on input type {input_object_type_name}: {}",
                field_names.into_iter().join(", "),
            )),
            InputCoercionError::NonUniqueFieldNames { field_name, .. } => Self::new(format!(
                "Object with multiple entries for field {field_name}"
            )),
            InputCoercionError::NoInputFieldWithName {
                field,
                input_object_type_name,
                ..
            } => Self::new(format!(
                "No field with name {} on input type {input_object_type_name}",
                field.as_ref()
            )),
            InputCoercionError::CustomScalarInvalidValue { message, .. } => Self::new(message),
            InputCoercionError::OneOfInputNullValues {
                input_object_type_name,
                ..
            } => Self::new(format!(
                "Multiple entries with null values for oneOf input object {input_object_type_name}"
            )),
            InputCoercionError::OneOfInputNotSingleNonNullValue { input_object_type_name, non_null_entries, .. } => Self::new(
                format!("Got {} entries with non-null values for oneOf input object {input_object_type_name}", non_null_entries.len())
            )
        }
    }
}

impl<'a, const CONST: bool, S: SchemaDefinition>
    From<DirectiveError<'a, CONST, ExecutableDocument<'a>, S>> for ValidationError
{
    fn from(value: DirectiveError<'a, CONST, ExecutableDocument<'a>, S>) -> Self {
        match value {
            DirectiveError::DirectiveDoesNotExist { directive } => Self::new(format!(
                "No directive definition with name `@{}`",
                directive.name().as_ref()
            )),
            DirectiveError::DirectivesNotUniquePerLocation { directive_definition, .. } => Self::new(
                format!(
                    "Directive @{} is not repeatable but was used multiple times in the same location",
                    directive_definition.name(),
                )
            ),
            DirectiveError::DirectiveInInvalidLocation { directive, directive_definition, location } => Self::new(
                format!(
                    "Directive @{} cannot be used at location {location}. It is only allowed at the following locations: {}",
                    directive.name().as_ref(),
                    directive_definition.locations().iter().join(", "),
                )
            )
        }
    }
}

impl<'a, const CONST: bool, S: SchemaDefinition>
    From<ArgumentError<'a, CONST, ExecutableDocument<'a>, S>> for ValidationError
{
    fn from(value: ArgumentError<'a, CONST, ExecutableDocument<'a>, S>) -> Self {
        match value {
            ArgumentError::DirectiveMissingRequiredArguments {
                directive,
                missing_argument_definitions,
                ..
            } => {
                let missing_argument_names = missing_argument_definitions
                    .into_iter()
                    .map(InputValueDefinition::name)
                    .join(", ");
                Self::new(format!(
                    "Directive `{}` missing argument(s): {missing_argument_names}",
                    directive.name().as_ref(),
                ))
            }
            ArgumentError::ArgumentDoesNotExistOnDirective {
                argument,
                directive_definition,
            } => Self::new(format!(
                "Directive `{}` does not define an argument named `{}`",
                directive_definition.name(),
                argument.name().as_ref(),
            )),
            ArgumentError::ArgumentDoesNotExistOnField {
                argument,
                field_definition,
            } => Self::new(format!(
                "Field `{}` does not define an argument named `{}`",
                field_definition.name(),
                argument.name().as_ref(),
            )),
            ArgumentError::NonUniqueArgumentNames { name, .. } => {
                Self::new(format!("Multiple arguments with name `{name}`"))
            }
            ArgumentError::FieldMissingRequiredArguments {
                field,
                missing_argument_definitions,
                ..
            } => {
                let missing_argument_names = missing_argument_definitions
                    .into_iter()
                    .map(InputValueDefinition::name)
                    .join(", ");
                Self::new(format!(
                    "Field `{}` missing argument(s): {missing_argument_names}",
                    field.response_key()
                ))
            }
        }
    }
}

pub fn init() -> Result<(), Error> {
    let class = root().define_class("ValidationError", Default::default())?;

    class.define_singleton_method("new", function!(<ValidationError as From<String>>::from, 1))?;
    class.define_method("message", method!(ValidationError::message, 0))?;
    class.define_method(
        "==",
        method!(<ValidationError as typed_data::IsEql>::is_eql, 1),
    )?;
    class.define_method("inspect", method!(ValidationError::inspect, 0))?;

    Ok(())
}
