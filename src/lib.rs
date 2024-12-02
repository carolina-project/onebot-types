use std::collections::BTreeMap;
#[allow(unused)]
use std::collections::HashMap;

#[cfg(feature = "ob11")]
pub mod ob11;
#[cfg(feature = "ob12")]
pub mod ob12;

#[cfg(feature = "compat")]
pub mod compat;

#[allow(unused)]
pub(crate) type ValueMap = BTreeMap<String, serde_value::Value>;
#[allow(unused)]
pub(crate) type SerResult<T> = Result<T, serde_value::SerializerError>;
#[allow(unused)]
pub(crate) type DesResult<T> = Result<T, serde_value::DeserializerError>;

pub mod base {
    pub use ob_types_base::error::OBError;
    pub use ob_types_base::error::TypeMismatchError;
    pub use ob_types_base::ext::IntoValue;
    pub use ob_types_base::ext::VTryFrom;
    pub use ob_types_base::ext::ValueExt;
    pub use ob_types_base::OBAction;
    pub use ob_types_base::OBRespData;
}
