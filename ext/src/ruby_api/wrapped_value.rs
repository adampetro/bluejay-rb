use bluejay_core::{
    BooleanValue, FloatValue, IntegerValue, ListValue as CoreListValue,
    ObjectValue as CoreObjectValue, Value as CoreValue,
};
use magnus::{
    exception, gc,
    r_hash::ForEach,
    value::{Qfalse, Qtrue},
    Error, Float, Integer, RArray, RHash, RString, Value,
};
use std::collections::HashMap;

pub type ValueInner =
    CoreValue<true, String, i32, f64, String, bool, (), String, ListValue, ObjectValue>;

#[derive(Debug)]
pub struct ListValue(Vec<ValueInner>);

impl AsRef<[ValueInner]> for ListValue {
    fn as_ref(&self) -> &[ValueInner] {
        &self.0
    }
}

impl From<Vec<ValueInner>> for ListValue {
    fn from(value: Vec<ValueInner>) -> Self {
        Self(value)
    }
}

impl CoreListValue<ValueInner> for ListValue {}

#[derive(Debug)]
pub struct ObjectValue(HashMap<String, ValueInner>);

impl CoreObjectValue<ValueInner> for ObjectValue {
    type Iterator<'a> = std::iter::Map<
        std::collections::hash_map::Iter<'a, String, ValueInner>,
        fn((&'a String, &'a ValueInner)) -> (&'a str, &'a ValueInner),
    >;

    fn iter(&self) -> Self::Iterator<'_> {
        self.0.iter().map(|(key, value)| (key.as_str(), value))
    }
}

impl From<HashMap<String, ValueInner>> for ObjectValue {
    fn from(value: HashMap<String, ValueInner>) -> Self {
        Self(value)
    }
}

#[derive(Debug)]
pub struct WrappedValue {
    r_value: Value,
    inner: ValueInner,
}

impl AsRef<ValueInner> for WrappedValue {
    fn as_ref(&self) -> &ValueInner {
        &self.inner
    }
}

impl From<WrappedValue> for ValueInner {
    fn from(val: WrappedValue) -> ValueInner {
        val.inner
    }
}

impl<'a> From<(Value, &bluejay_parser::ast::ConstValue<'a>)> for WrappedValue {
    fn from((r_value, inner): (Value, &bluejay_parser::ast::ConstValue)) -> Self {
        let inner = value_inner_from_parser_const_value(inner);
        Self { r_value, inner }
    }
}

impl From<(Value, ValueInner)> for WrappedValue {
    fn from((r_value, inner): (Value, ValueInner)) -> Self {
        Self { r_value, inner }
    }
}

impl TryFrom<Value> for WrappedValue {
    type Error = Error;

    fn try_from(r_value: Value) -> Result<Self, Self::Error> {
        let inner = value_inner_from_ruby_const_value(r_value)?;
        Ok(Self { r_value, inner })
    }
}

impl From<WrappedValue> for (Value, ValueInner) {
    fn from(value: WrappedValue) -> Self {
        (value.r_value, value.inner)
    }
}

impl From<WrappedValue> for Value {
    fn from(value: WrappedValue) -> Self {
        value.r_value
    }
}

impl From<&WrappedValue> for Value {
    fn from(value: &WrappedValue) -> Self {
        value.r_value
    }
}

fn value_inner_from_parser_const_value(value: &bluejay_parser::ast::ConstValue) -> ValueInner {
    match value {
        CoreValue::Boolean(b) => ValueInner::Boolean(b.to_bool()),
        CoreValue::Enum(e) => ValueInner::Enum(e.as_str().to_owned()),
        CoreValue::Float(f) => ValueInner::Float(f.to_f64()),
        CoreValue::Integer(i) => ValueInner::Integer(i.to_i32()),
        CoreValue::List(l) => ValueInner::List(ListValue(Vec::from_iter(
            l.as_ref().iter().map(value_inner_from_parser_const_value),
        ))),
        CoreValue::Null(_) => ValueInner::Null(()),
        CoreValue::Object(o) => ValueInner::Object(ObjectValue(
            o.iter()
                .map(|(name, value)| (name.to_string(), value_inner_from_parser_const_value(value)))
                .collect(),
        )),
        CoreValue::String(s) => ValueInner::String(s.to_string()),
        CoreValue::Variable(_) => unreachable!(),
    }
}

impl WrappedValue {
    pub(crate) fn mark(&self) {
        gc::mark(&self.r_value);
    }
}

pub fn value_inner_from_ruby_const_value(val: Value) -> Result<ValueInner, Error> {
    // TODO: support BigDecimal or even better, Numeric
    if let Some(i) = Integer::from_value(val) {
        // TODO: reconcile if we need to handle integers bigger than 32 bits
        // and if not, produce a better error for the user
        Ok(ValueInner::Integer(i.to_i32()?))
    } else if let Some(f) = Float::from_value(val) {
        Ok(ValueInner::Float(f.to_f64()))
    } else if Qtrue::from_value(val).is_some() {
        Ok(ValueInner::Boolean(true))
    } else if Qfalse::from_value(val).is_some() {
        Ok(ValueInner::Boolean(false))
    } else if let Some(s) = RString::from_value(val) {
        Ok(ValueInner::String(s.to_string()?))
    } else if val.is_nil() {
        Ok(ValueInner::Null(()))
    } else if let Some(arr) = RArray::from_value(val) {
        let v: Result<Vec<ValueInner>, Error> = arr
            .each()
            .map(|el| el.and_then(value_inner_from_ruby_const_value))
            .collect();
        Ok(ValueInner::List(ListValue(v?)))
    } else if let Some(r_hash) = RHash::from_value(val) {
        let mut h: HashMap<String, ValueInner> = HashMap::new();
        r_hash.foreach(|k, v| {
            let v = value_inner_from_ruby_const_value(v)?;
            h.insert(k, v);
            Ok(ForEach::Continue)
        })?;
        Ok(ValueInner::Object(h.into()))
    } else {
        Err(Error::new(
            exception::type_error(),
            format!("no implicit conversion of {} into Value", unsafe {
                val.classname()
            },),
        ))
    }
}
