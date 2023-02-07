use crate::ruby_api::InputObject;
use magnus::{
    exception,
    value::{Qfalse, Qtrue},
    Error, Float, Integer, RArray, RString, TryConvert, Value as RubyValue,
};

pub type ValueInner =
    bluejay_core::Value<true, String, i32, f64, String, bool, (), String, ListValue, InputObject>;

#[derive(Debug)]
pub struct ListValue(Vec<ValueInner>);

impl AsRef<[ValueInner]> for ListValue {
    fn as_ref(&self) -> &[ValueInner] {
        &self.0
    }
}

impl bluejay_core::ListValue<ValueInner> for ListValue {}

#[derive(Debug)]
pub struct Value(ValueInner);

impl AsRef<ValueInner> for Value {
    fn as_ref(&self) -> &ValueInner {
        &self.0
    }
}

impl From<Value> for ValueInner {
    fn from(val: Value) -> ValueInner {
        val.0
    }
}

impl Value {
    pub(crate) fn mark(&self) {
        mark_inner(&self.0)
    }
}

pub(crate) fn mark_inner(inner: &ValueInner) {
    match inner {
        ValueInner::Object(o) => o.mark(),
        ValueInner::List(l) => l.as_ref().iter().for_each(mark_inner),
        _ => {}
    }
}

impl TryConvert for Value {
    fn try_convert(val: RubyValue) -> Result<Self, Error> {
        // TODO: see if it is poosible to reuse input coercion in some way here
        // TODO: support BigDecimal or even better, Numeric
        if let Some(i) = Integer::from_value(val) {
            // TODO: reconcile if we need to handle integers bigger than 32 bits
            // and if not, produce a better error for the user
            Ok(Value(ValueInner::Integer(i.to_i32()?)))
        } else if let Some(f) = Float::from_value(val) {
            Ok(Value(ValueInner::Float(f.to_f64())))
        } else if Qtrue::from_value(val).is_some() {
            Ok(Value(ValueInner::Boolean(true)))
        } else if Qfalse::from_value(val).is_some() {
            Ok(Value(ValueInner::Boolean(false)))
        } else if let Some(s) = RString::from_value(val) {
            Ok(Value(ValueInner::String(s.to_string()?)))
        } else if val.is_nil() {
            Ok(Value(ValueInner::Null(())))
        } else if let Some(arr) = RArray::from_value(val) {
            let mut v: Vec<ValueInner> = Vec::new();
            for el in arr.each() {
                let json_value: Self = el?.try_convert()?;
                v.push(json_value.0);
            }
            Ok(Value(ValueInner::List(ListValue(v))))
        } else if let Ok(input_object) = val.try_convert() {
            Ok(Value(ValueInner::Object(input_object)))
        } else {
            Err(Error::new(
                exception::type_error(),
                format!("no implicit conversion of {} into Value", unsafe {
                    val.classname()
                },),
            ))
        }
    }
}
