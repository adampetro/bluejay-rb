use super::root;
use crate::ruby_api::SchemaDefinition;
use bluejay_core::OperationType;
use bluejay_parser::ast::executable::ExecutableDocument;
use bluejay_validator::executable::Error as CoreError;
use magnus::{
    function, method,
    rb_sys::AsRawValue,
    typed_data::{self, Obj},
    Error, Module, Object,
};

#[derive(Clone, Debug, PartialEq, Eq)]
#[magnus::wrap(class = "Bluejay::ValidationError", mark)]
pub struct ValidationError {
    message: String,
}

impl ValidationError {
    pub fn new(message: String) -> Self {
        Self { message }
    }

    pub fn message(&self) -> &str {
        self.message.as_str()
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

impl<'a> From<CoreError<'a, ExecutableDocument<'a>, SchemaDefinition>> for ValidationError {
    fn from(value: CoreError<'a, ExecutableDocument<'a>, SchemaDefinition>) -> Self {
        match value {
            CoreError::NotLoneAnonymousOperation {
                anonymous_operations: _,
            } => Self::new(
                "Anonymous operations are not allowed when there is more than one operation"
                    .to_string(),
            ),
            CoreError::NonUniqueOperationNames {
                name,
                operations: _,
            } => Self::new(format!("More than one operation named `{name}`")),
            CoreError::SubscriptionRootNotSingleField { operation: _ } => Self::new(
                "Subscription operations can only select one field at the root".to_string(),
            ),
            CoreError::FieldDoesNotExistOnType { field, r#type } => Self::new(format!(
                "Field `{}` does not exist on `{}`",
                field.name().as_ref(),
                r#type.name()
            )),
            CoreError::FieldSelectionsDoNotMerge { selection_set: _ } => {
                Self::new("Field selections do not merge".to_string())
            }
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
        }
    }
}

pub fn init() -> Result<(), Error> {
    let class = root().define_class("ValidationError", Default::default())?;

    class.define_singleton_method("new", function!(ValidationError::new, 1))?;
    class.define_method("message", method!(ValidationError::message, 0))?;
    class.define_method(
        "==",
        method!(<ValidationError as typed_data::IsEql>::is_eql, 1),
    )?;
    class.define_method("inspect", method!(ValidationError::inspect, 0))?;

    Ok(())
}
