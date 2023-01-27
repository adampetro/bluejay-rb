use magnus::{Value, TryConvert, Error, Integer, Float, value::{Qtrue, Qfalse}, RString, RArray, RHash, r_hash::ForEach, exception, QNIL};

pub type JsonValueInner = bluejay_core::Value<true, String, i32, f64, String, bool, (), String, ListValue, ObjectValue>;

#[derive(Clone, Debug)]
pub struct ListValue(Vec<JsonValueInner>);

impl AsRef<[JsonValueInner]> for ListValue {
    fn as_ref(&self) -> &[JsonValueInner] {
        &self.0
    }
}

impl bluejay_core::ListValue<JsonValueInner> for ListValue {}

impl Into<Value> for ListValue {
    fn into(self) -> Value {
        *RArray::from_iter(self.0.iter().map(|v| core_value_to_value(v.clone())))
    }
}

#[derive(Clone, Debug)]
pub struct ObjectValue(Vec<(String, JsonValueInner)>);

impl bluejay_core::ObjectValue<JsonValueInner> for ObjectValue {
    type Key = String;

    fn fields(&self) -> &[(Self::Key, JsonValueInner)] {
        &self.0
    }
}

impl Into<Value> for ObjectValue {
    fn into(self) -> Value {
        *RHash::from_iter(self.0.iter().map(|(k, v)| (k.as_str(), core_value_to_value(v.clone()))))
    }
}

#[derive(Clone, Debug)]
pub struct JsonValue(JsonValueInner);

impl AsRef<JsonValueInner> for JsonValue {
    fn as_ref(&self) -> &JsonValueInner {
        &self.0
    }
}

impl TryConvert for JsonValue {
    fn try_convert(val: Value) -> Result<Self, Error> {
        // TODO: support BigDecimal or even better, Numeric
        if let Some(i) = Integer::from_value(val) {
            // TODO: reconcile if we need to handle integers bigger than 32 bits
            // and if not, produce a better error for the user
            Ok(JsonValue(JsonValueInner::Integer(i.to_i32()?)))
        } else if let Some(f) = Float::from_value(val) {
            Ok(JsonValue(JsonValueInner::Float(f.to_f64())))
        } else if Qtrue::from_value(val).is_some() {
            Ok(JsonValue(JsonValueInner::Boolean(true)))
        } else if Qfalse::from_value(val).is_some() {
            Ok(JsonValue(JsonValueInner::Boolean(false)))
        } else if let Some(s) = RString::from_value(val) {
            Ok(JsonValue(JsonValueInner::String(s.to_string()?)))
        } else if val.is_nil() {
            Ok(JsonValue(JsonValueInner::Null(())))
        } else if let Some(arr) = RArray::from_value(val) {
            let mut v: Vec<JsonValueInner> = Vec::new();
            for el in arr.each() {
                let json_value: Self = el?.try_convert()?;
                v.push(json_value.0);
            }
            Ok(JsonValue(JsonValueInner::List(ListValue(v))))
        } else if let Some(h) = RHash::from_value(val) {
            let mut v: Vec<(String, JsonValueInner)> = Vec::new();
            h.foreach(|key: String, value: Value| {
                let json_value: Self = value.try_convert()?;
                v.push((key, json_value.0));
                Ok(ForEach::Continue)
            })?;
            Ok(JsonValue(JsonValueInner::Object(ObjectValue(v))))
        } else {
            Err(Error::new(
                exception::type_error(),
                format!(
                    "no implicit conversion of {} into JsonValue",
                    unsafe { val.classname() },
                ),
            ))
        }
    }
}

// TODO: get rid of this once input coercion is done by core crate
impl Into<Value> for JsonValue {
    fn into(self) -> Value {
        core_value_to_value(self.0)
    }
}

fn core_value_to_value(core_value: JsonValueInner) -> Value {
    match core_value {
        JsonValueInner::Boolean(b) => b.into(),
        JsonValueInner::String(s) => s.into(),
        JsonValueInner::Integer(i) => i.into(),
        JsonValueInner::Float(f) => f.into(),
        JsonValueInner::Enum(e) => e.into(),
        JsonValueInner::List(l) => l.into(),
        JsonValueInner::Object(o) => o.into(),
        JsonValueInner::Variable(_) => unreachable!(),
        JsonValueInner::Null(_) => *QNIL,
    }
}
