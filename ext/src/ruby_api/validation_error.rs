use super::root;
use crate::ruby_api::SchemaDefinition;
use bluejay_core::{
    call_const_wrapper_method,
    definition::{AbstractOutputTypeReference, InputValueDefinition},
    ArgumentWrapper, Directive, DirectiveWrapper, OperationType,
};
use bluejay_parser::ast::executable::ExecutableDocument;
use bluejay_validator::executable::Error as CoreError;
use itertools::join;
use magnus::{
    function, method,
    rb_sys::AsRawValue,
    typed_data::{self, Obj},
    Error, Module, Object,
};
use std::borrow::Cow;

#[derive(Clone, Debug, PartialEq, Eq)]
#[magnus::wrap(class = "Bluejay::ValidationError", mark)]
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

impl<'a> From<CoreError<'a, ExecutableDocument<'a>, SchemaDefinition>> for ValidationError {
    fn from(value: CoreError<'a, ExecutableDocument<'a>, SchemaDefinition>) -> Self {
        match value {
            CoreError::NotLoneAnonymousOperation {
                anonymous_operations: _,
            } => Self::new(
                "Anonymous operations are not allowed when there is more than one operation",
            ),
            CoreError::NonUniqueOperationNames {
                name,
                operations: _,
            } => Self::new(format!("More than one operation named `{name}`")),
            CoreError::SubscriptionRootNotSingleField { operation: _ } => {
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
            CoreError::LeafFieldSelectionNotEmpty {
                selection_set: _,
                r#type,
            } => Self::new(format!(
                "Selection on field of leaf type `{}` was not empty",
                r#type.as_ref().display_name()
            )),
            CoreError::NonLeafFieldSelectionEmpty { field: _, r#type } => Self::new(format!(
                "No selection on field of non-leaf type `{}`",
                r#type.as_ref().display_name()
            )),
            CoreError::ArgumentDoesNotExistOnField {
                argument,
                field_definition,
            } => Self::new(format!(
                "Field `{}` does not define an argument named `{}`",
                field_definition.name(),
                argument.name().as_ref(),
            )),
            CoreError::ArgumentDoesNotExistOnDirective {
                argument,
                directive_definition,
            } => {
                let name = call_const_wrapper_method!(ArgumentWrapper, argument, name);
                Self::new(format!(
                    "Directive `{}` does not define an argument named `{}`",
                    directive_definition.name(),
                    name.as_ref(),
                ))
            }
            CoreError::NonUniqueArgumentNames { arguments: _, name } => {
                Self::new(format!("Multiple arguments with name `{name}`"))
            }
            CoreError::FieldMissingRequiredArguments {
                field,
                field_definition: _,
                missing_argument_definitions,
                arguments_with_null_values: _,
            } => {
                let missing_argument_names = join(
                    missing_argument_definitions
                        .into_iter()
                        .map(InputValueDefinition::name),
                    ", ",
                );
                Self::new(format!(
                    "Field `{}` missing argument(s): {missing_argument_names}",
                    field.response_key()
                ))
            }
            CoreError::DirectiveMissingRequiredArguments {
                directive,
                directive_definition: _,
                missing_argument_definitions,
                arguments_with_null_values: _,
            } => {
                let missing_argument_names = join(
                    missing_argument_definitions
                        .into_iter()
                        .map(InputValueDefinition::name),
                    ", ",
                );
                let directive_name = call_const_wrapper_method!(DirectiveWrapper, directive, name);
                Self::new(format!(
                    "Directive `{directive_name}` missing argument(s): {missing_argument_names}",
                ))
            }
            CoreError::NonUniqueFragmentDefinitionNames {
                name,
                fragment_definitions: _,
            } => Self::new(format!("Multiple fragment definitions named `{name}`")),
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
                fragment_spread: _,
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
