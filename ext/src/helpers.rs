mod wrapped_definition;
pub use wrapped_definition::{HasDefinitionWrapper, WrappedDefinition};
mod public_name;
pub use public_name::public_name;
mod typed_frozen_r_array;
pub use typed_frozen_r_array::TypedFrozenRArray;
mod variables;
pub use variables::Variables;

use bluejay_core::{AsIter, ObjectValue, Value as CoreValue, Variable};
use magnus::{RArray, RHash, Value, QNIL};

pub fn value_from_core_value<const CONST: bool>(
    value: &impl bluejay_core::AbstractValue<CONST>,
    variables: &impl Variables<CONST>,
) -> Value {
    match value.as_ref() {
        CoreValue::Boolean(b) => b.into(),
        CoreValue::Enum(e) => e.into(),
        CoreValue::Float(f) => f.into(),
        CoreValue::Integer(i) => i.into(),
        CoreValue::Null => *QNIL,
        CoreValue::String(s) => s.into(),
        CoreValue::Variable(var) => {
            if CONST {
                unreachable!()
            } else {
                variables.get(var.name()).unwrap_or(*QNIL)
            }
        }
        CoreValue::List(l) => {
            *RArray::from_iter(l.iter().map(|v| value_from_core_value(v, variables)))
        }
        CoreValue::Object(o) => *RHash::from_iter(
            o.iter()
                .map(|(k, v)| (k.as_ref(), value_from_core_value(v, variables))),
        ),
    }
}
