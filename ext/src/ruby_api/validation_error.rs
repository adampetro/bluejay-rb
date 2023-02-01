use super::root;
use crate::helpers::WrappedStruct;
use crate::ruby_api::SchemaDefinition;
use bluejay_core::validation::executable::Error as CoreError;
use bluejay_parser::ast::executable::ExecutableDocument;
use magnus::{function, method, rb_sys::AsRawValue, Error, Module, Object, Value};

#[derive(Clone, Debug, PartialEq)]
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

    pub fn eql(&self, other: Value) -> bool {
        if let Ok(other) = other.try_convert::<&Self>() {
            self == other
        } else {
            false
        }
    }

    fn inspect(rb_self: WrappedStruct<Self>) -> Result<String, Error> {
        let rs_self = rb_self.get();

        Ok(format!(
            "#<Bluejay::ValidationError:0x{:016x} @message={:?}>",
            rb_self.to_value().as_raw(),
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
                field.name(),
                r#type.name()
            )),
            CoreError::FieldSelectionsDoNotMerge { selection_set: _ } => {
                Self::new("Field selections do not merge".to_string())
            }
        }
    }
}

pub fn init() -> Result<(), Error> {
    let class = root().define_class("ValidationError", Default::default())?;

    class.define_singleton_method("new", function!(ValidationError::new, 1))?;
    class.define_method("message", method!(ValidationError::message, 0))?;
    class.define_method("==", method!(ValidationError::eql, 1))?;
    class.define_method("inspect", method!(ValidationError::inspect, 0))?;

    Ok(())
}
