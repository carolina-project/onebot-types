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
#[cfg_attr(feature = "json", serde(from = "serde_json::Value"))]
pub enum JSONValue {
    Object(JSONMap),
    Array(Vec<JSONValue>),
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
    use serde_json::Value;

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
