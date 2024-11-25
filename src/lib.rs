use std::collections::BTreeMap;
#[allow(unused)]
use std::collections::HashMap;

#[cfg(feature = "ob11")]
pub mod ob11;
#[cfg(feature = "ob12")]
pub mod ob12;

#[cfg(feature = "compat")]
pub mod compat;

pub(crate) type ValueMap = BTreeMap<serde_value::Value, serde_value::Value>;
#[allow(unused)]
pub(crate) type SerResult<T> = Result<T, serde_value::SerializerError>;
#[allow(unused)]
pub(crate) type DesResult<T> = Result<T, serde_value::DeserializerError>;
