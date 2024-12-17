use std::collections::BTreeMap;
#[allow(unused)]
use std::collections::HashMap;

pub mod base;

#[cfg(feature = "ob11")]
pub mod ob11;
#[cfg(feature = "ob12")]
pub mod ob12;

#[cfg(feature = "compat")]
pub mod compat;

#[allow(unused)]
pub type ValueMap = BTreeMap<String, serde_value::Value>;
#[allow(unused)]
pub(crate) type SerResult<T> = Result<T, serde_value::SerializerError>;
#[allow(unused)]
pub(crate) type DesResult<T> = Result<T, serde_value::DeserializerError>;

pub use base::{OBAction, OBEvent, OBMessage};

pub use ob_types_macro::{OBAction, OBEvent, OBMessage};

#[cfg(feature = "ob12")]
pub use {base::OBEventSelector, ob_types_macro::OBEventSelector};
