use bluejay_core::{
    AsIter, ListValue as CoreListValue, ObjectValue as CoreObjectValue, Value as CoreValue,
    ValueReference,
};
use bluejay_parser::ast::ConstValue as ParserConstValue;
use magnus::{
    exception, gc,
    r_hash::ForEach,
    value::{Qfalse, Qtrue},
    Error, Float, Integer, RArray, RHash, RString, Value,
};
use std::collections::HashMap;

#[derive(Debug)]
pub enum ValueInner {
    Integer(i32),
    Float(f64),
    String(String),
    Boolean(bool),
    Null,
    Enum(String),
    List(ListValue),
    Object(ObjectValue),
}

pub enum Never {}

impl bluejay_core::Variable for Never {
    fn name(&self) -> &str {
        unreachable!()
    }
}

impl CoreValue<true> for ValueInner {
    type List = ListValue;
    type Object = ObjectValue;
    type Variable = Never;

    fn as_ref(&self) -> ValueReference<'_, true, Self> {
        match self {
            Self::Integer(i) => ValueReference::Integer(*i),
            Self::Float(f) => ValueReference::Float(*f),
            Self::String(s) => ValueReference::String(s),
            Self::Boolean(b) => ValueReference::Boolean(*b),
            Self::Null => ValueReference::Null,
            Self::Enum(e) => ValueReference::Enum(e),
            Self::List(l) => ValueReference::List(l),
            Self::Object(o) => ValueReference::Object(o),
        }
    }
}

#[derive(Debug)]
pub struct ListValue(Vec<ValueInner>);

impl AsIter for ListValue {
    type Item = ValueInner;
    type Iterator<'a> = std::slice::Iter<'a, Self::Item>;

    fn iter(&self) -> Self::Iterator<'_> {
        self.0.iter()
    }
}

impl From<Vec<ValueInner>> for ListValue {
    fn from(value: Vec<ValueInner>) -> Self {
        Self(value)
    }
}

impl CoreListValue<true> for ListValue {
    type Value = ValueInner;
}

#[derive(Debug)]
pub struct ObjectValue(HashMap<String, ValueInner>);

impl CoreObjectValue<true> for ObjectValue {
    type Key = String;
    type Value = ValueInner;
    type Iterator<'a> = std::collections::hash_map::Iter<'a, String, ValueInner>;

    fn iter(&self) -> Self::Iterator<'_> {
        self.0.iter()
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

impl WrappedValue {
    pub fn to_value(&self) -> Value {
        self.r_value
    }
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

fn value_inner_from_parser_const_value(value: &ParserConstValue) -> ValueInner {
    match value.as_ref() {
        ValueReference::Boolean(b) => ValueInner::Boolean(b),
        ValueReference::Enum(e) => ValueInner::Enum(e.to_owned()),
        ValueReference::Float(f) => ValueInner::Float(f),
        ValueReference::Integer(i) => ValueInner::Integer(i),
        ValueReference::List(l) => ValueInner::List(ListValue(Vec::from_iter(
            l.iter().map(value_inner_from_parser_const_value),
        ))),
        ValueReference::Null => ValueInner::Null,
        ValueReference::Object(o) => ValueInner::Object(ObjectValue(
            CoreObjectValue::iter(o)
                .map(|(name, value)| {
                    (
                        name.as_ref().to_string(),
                        value_inner_from_parser_const_value(value),
                    )
                })
                .collect(),
        )),
        ValueReference::String(s) => ValueInner::String(s.to_string()),
        ValueReference::Variable(_) => unreachable!(),
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
        Ok(ValueInner::Null)
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
