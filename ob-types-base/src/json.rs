use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum JSONValue {
    Object(HashMap<String, JSONValue>),
    Array(Vec<JSONValue>),
    Int(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Null,
}

#[cfg(feature = "json")]
mod serde_impl {
    use super::JSONValue;
    use serde::{
        ser::{SerializeMap, SerializeSeq},
        Deserialize,
    };
    use serde_json::Value;

    impl<'de> Deserialize<'de> for JSONValue {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let value = Value::deserialize(deserializer)?;
            Ok(value.into())
        }
    }

    impl serde::Serialize for JSONValue {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            match self {
                JSONValue::Object(map) => {
                    let mut obj = serializer.serialize_map(Some(map.len()))?;
                    for (k, v) in map {
                        obj.serialize_entry(k, v)?;
                    }
                    obj.end()
                }
                JSONValue::Array(arr) => {
                    let mut o_arr = serializer.serialize_seq(Some(arr.len()))?;
                    for ele in arr {
                        o_arr.serialize_element(ele)?;
                    }
                    o_arr.end()
                }
                JSONValue::Int(i) => serializer.serialize_i64(*i),
                JSONValue::Float(f) => serializer.serialize_f64(*f),
                JSONValue::String(s) => serializer.serialize_str(s),
                JSONValue::Boolean(b) => serializer.serialize_bool(*b),
                JSONValue::Null => serializer.serialize_none(),
            }
        }
    }

    impl From<Value> for JSONValue {
        fn from(value: Value) -> Self {
            match value {
                Value::Null => JSONValue::Null,
                Value::Bool(r) => JSONValue::Boolean(r),
                Value::Number(r) => {
                    if let Some(int) = r.as_i64() {
                        JSONValue::Int(int)
                    } else {
                        JSONValue::Float(r.as_f64().unwrap())
                    }
                }
                Value::String(r) => JSONValue::String(r),
                Value::Array(r) => JSONValue::Array(r.into_iter().map(Value::into).collect()),
                Value::Object(r) => {
                    JSONValue::Object(r.into_iter().map(|(k, v)| (k, v.into())).collect())
                }
            }
        }
    }
}
