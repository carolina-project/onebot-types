use std::collections::BTreeMap;

macro_rules! json_from {
    ($typ: ty, $into: ident) => {
        impl From<$typ> for JSONValue {
            fn from(value: $typ) -> Self {
                Self::$into(value.into())
            }
        }
    };
}

pub type JSONMap = BTreeMap<String, JSONValue>;


#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "json", serde(untagged))]
#[cfg_attr(
    feature = "zerocpy",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
)]
#[cfg_attr(
    feature = "zerocpy", 
    rkyv(serialize_bounds(
        __S: rkyv::ser::Writer + rkyv::ser::Allocator,
        __S::Error: rkyv::rancor::Source,
    ))
)]
#[cfg_attr(
    feature = "zerocpy", 
    rkyv(deserialize_bounds(__D::Error: rkyv::rancor::Source))
)]
pub enum JSONValue {
    Object(#[cfg_attr(feature = "zerocpy", rkyv(omit_bounds))] JSONMap),
    Array(#[cfg_attr(feature = "zerocpy", rkyv(omit_bounds))] Vec<JSONValue>),
    Int(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    #[default]
    Null,
}

json_from!(String, String);
json_from!(u16, Int);
json_from!(i64, Int);
json_from!(JSONMap, Object);
json_from!(&str, String);

#[cfg(feature = "json")]
mod serde_impl {
    use super::JSONValue;
    use serde_json::{Number, Value};

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

    impl From<JSONValue> for Value {
        fn from(value: JSONValue) -> Self {
            match value {
                JSONValue::Null => Value::Null,
                JSONValue::Boolean(r) => Value::Bool(r),
                JSONValue::Int(r) => Value::Number(r.into()),
                JSONValue::Float(r) => Number::from_f64(r)
                    .map(Value::Number)
                    .unwrap_or(Value::Null),
                JSONValue::String(r) => Value::String(r),
                JSONValue::Array(r) => Value::Array(r.into_iter().map(Value::from).collect()),
                JSONValue::Object(r) => {
                    Value::Object(r.into_iter().map(|(k, v)| (k, Value::from(v))).collect())
                }
            }
        }
    }
}
