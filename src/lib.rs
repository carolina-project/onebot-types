use std::collections::BTreeMap;
#[allow(unused)]
use std::collections::HashMap;

#[cfg(feature = "ob11")]
pub mod ob11;
#[cfg(feature = "ob12")]
pub mod ob12;

#[cfg(all(feature = "ob11", feature = "ob12"))]
pub mod compat;

pub(crate) type ValueMap = BTreeMap<serde_value::Value, serde_value::Value>;
