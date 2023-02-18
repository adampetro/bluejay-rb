mod wrapped_definition;
pub use wrapped_definition::{HasDefinitionWrapper, WrappedDefinition};
mod public_name;
pub use public_name::public_name;
mod typed_frozen_r_array;
pub use typed_frozen_r_array::TypedFrozenRArray;
mod wrapped_struct;
pub use wrapped_struct::WrappedStruct;
mod variables;
pub use variables::Variables;

use bluejay_core::{
    BooleanValue, FloatValue, IntegerValue, ObjectValue, Value as CoreValue,
    Variable as CoreVariable,
};
use magnus::{RArray, RHash, Value, QNIL};

pub fn value_from_core_value<const CONST: bool>(
    value: &impl bluejay_core::AbstractValue<CONST>,
    variables: &impl Variables<CONST>,
) -> Value {
    match value.as_ref() {
        CoreValue::Boolean(b) => b.to_bool().into(),
        CoreValue::Enum(e) => e.as_ref().into(),
        CoreValue::Float(f) => f.to_f64().into(),
        CoreValue::Integer(i) => i.to_i32().into(),
        CoreValue::Null(_) => *QNIL,
        CoreValue::String(s) => s.as_ref().into(),
        CoreValue::Variable(var) => {
            if CONST {
                unreachable!()
            } else {
                variables.get(var.name()).unwrap_or(*QNIL)
            }
        }
        CoreValue::List(l) => *RArray::from_iter(
            l.as_ref()
                .iter()
                .map(|v| value_from_core_value(v, variables)),
        ),
        CoreValue::Object(o) => *RHash::from_iter(
            o.iter()
                .map(|(k, v)| (k, value_from_core_value(v, variables))),
        ),
    }
}
