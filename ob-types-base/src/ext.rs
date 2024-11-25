use std::collections::BTreeMap;

use serde_value::Value;

use crate::error::TypeMismatchError;

pub trait ValueExt {
    fn is_number(&self) -> bool;
    fn as_i64(&self) -> Option<i64>;
    fn as_u64(&self) -> Option<u64>;
    fn as_f64(&self) -> Option<f64>;
    fn as_bool(&self) -> Option<bool>;
    fn try_into_string(self) -> Option<String>;
    fn get_by_str(&self, key: impl AsRef<str>) -> Option<&Value>;

    fn from_map(map: BTreeMap<String, Value>) -> Value {
        Value::Map(map.into_iter().map(|(k, v)| (k.into_value(), v)).collect())
    }
}

pub trait IntoValue {
    fn into_value(self) -> Value;
}

pub trait VTryFrom {
    fn value_try_from(value: Value) -> Result<Self, TypeMismatchError>
    where
        Self: Sized;
}

macro_rules! from_value {
    ($($typ:ty : $enum_ty:ident),*) => {
        $(impl VTryFrom for $typ {
            fn value_try_from(value: Value) -> Result<Self, TypeMismatchError> {
                match value {
                    Value::$enum_ty(v) => Ok(v),
                    _ => Err(TypeMismatchError {
                        expected: stringify!($typ).into(),
                        got: format!("{:?}", value),
                    }),
                }
            }
        })*
    };
}

from_value!(
    u8: U8,
    u16: U16,
    u32: U32,
    u64: U64,
    i8: I8,
    i16: I16,
    i32: I32,
    i64: I64,
    f32: F32,
    f64: F64,
    String: String,
    bool: Bool,
    BTreeMap<Value, Value>: Map
);

macro_rules! into_value {
    ($($typ:ty : $enum_ty:ident),*) => {
        $(impl IntoValue for $typ {
            #[inline]
            fn into_value(self) -> Value {
            Value::$enum_ty(self)
            }
        })*
    };
}

into_value!(
    u8: U8,
    u16: U16,
    u32: U32,
    u64: U64,
    i8: I8,
    i16: I16,
    i32: I32,
    i64: I64,
    f32: F32,
    f64: F64,
    String: String,
    bool: Bool,
    BTreeMap<Value, Value>: Map
);

macro_rules! as_type {
    ($self:ident, $into:ty | $($typ:ident),* |) => {
        match $self {
            $(Value::$typ(v) => Some(*v as $into),)*
            _ => None,
        }
    };
}

impl ValueExt for Value {
    fn is_number(&self) -> bool {
        matches!(
            self,
            Value::I8(_)
                | Value::I16(_)
                | Value::I32(_)
                | Value::I64(_)
                | Value::U8(_)
                | Value::U16(_)
                | Value::U32(_)
                | Value::U64(_)
                | Value::F32(_)
                | Value::F64(_)
        )
    }

    fn as_i64(&self) -> Option<i64> {
        as_type!(self, i64 |U8, U16, U32, U64, I8, I16, I32, I64|)
    }

    fn as_u64(&self) -> Option<u64> {
        as_type!(self, u64 |U8, U16, U32, U64, I8, I16, I32, I64|)
    }

    fn as_f64(&self) -> Option<f64> {
        as_type!(self, f64 |F32, F64|)
    }

    fn get_by_str(&self, key: impl AsRef<str>) -> Option<&Value> {
        match self {
            Value::Map(map) => map.get(&key.as_ref().to_owned().into_value()),
            _ => None,
        }
    }

    fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(b) => Some(*b),
            _ => None,
        }
    }

    fn try_into_string(self) -> Option<String> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }
}
