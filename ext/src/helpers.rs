mod funcall_kw;
mod public_name;
mod typed_frozen_r_array;
mod variables;
mod warden;
mod wrapped_definition;

pub use funcall_kw::{FuncallKw, NewInstanceKw};
pub use public_name::public_name;
pub use typed_frozen_r_array::TypedFrozenRArray;
pub use variables::Variables;
pub use warden::Warden;
pub use wrapped_definition::{HasDefinitionWrapper, WrappedDefinition, Wrapper};

use bluejay_core::{AsIter, ObjectValue, Value as CoreValue, ValueReference, Variable};
use magnus::{RArray, RHash, TryConvert, Value, QNIL};
use std::marker::PhantomData;

pub fn value_from_core_value<const CONST: bool>(
    value: &impl CoreValue<CONST>,
    variables: &impl Variables<CONST>,
) -> Value {
    match value.as_ref() {
        ValueReference::Boolean(b) => b.into(),
        ValueReference::Enum(e) => e.into(),
        ValueReference::Float(f) => f.into(),
        ValueReference::Integer(i) => i.into(),
        ValueReference::Null => *QNIL,
        ValueReference::String(s) => s.into(),
        ValueReference::Variable(var) => {
            if CONST {
                unreachable!()
            } else {
                variables.get(var.name()).unwrap_or(*QNIL)
            }
        }
        ValueReference::List(l) => {
            *RArray::from_iter(l.iter().map(|v| value_from_core_value(v, variables)))
        }
        ValueReference::Object(o) => *RHash::from_iter(
            o.iter()
                .map(|(k, v)| (k.as_ref(), value_from_core_value(v, variables))),
        ),
    }
}

pub struct RArrayIter<'a, T: TryConvert> {
    data: &'a RArray,
    idx: usize,
    item_type: PhantomData<T>,
}

impl<'a, T: TryConvert> From<&'a RArray> for RArrayIter<'a, T> {
    fn from(value: &'a RArray) -> Self {
        Self {
            data: value,
            idx: 0,
            item_type: Default::default(),
        }
    }
}

impl<T: TryConvert> Iterator for RArrayIter<'_, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.data.len() {
            None
        } else {
            let value = self.data.entry(self.idx as isize).ok();
            self.idx += 1;
            value
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.data.len() - self.idx;
        (remaining, Some(remaining))
    }
}

#[cfg(ruby_gte_3_2)]
pub(crate) fn rhash_with_capacity(cap: usize) -> RHash {
    RHash::with_capacity(cap)
}

#[cfg(not(ruby_gte_3_2))]
pub(crate) fn rhash_with_capacity(_: usize) -> RHash {
    RHash::new()
}
