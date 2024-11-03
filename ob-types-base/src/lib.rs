pub mod error;
pub mod json;

use std::borrow::Cow;

pub use error::OBResult;
pub use json::JSONValue;

pub mod tool;

pub trait OBRespData {
    #[cfg(feature = "json")]
    fn from_json_raw(data: serde_json::Value) -> OBResult<Self>
    where
        Self: Sized;
}

macro_rules! impl_ob_resp_data {
    ($($types:ty),*) => {
        #[cfg(not(feature = "json"))]
        mod impl_ob_resp {
            $(
                impl super::OBRespData for $types {}
            )*
        }
    };
}

macro_rules! impl_ob_resp_gene {
    ($($types:ty)*) => {
        #[cfg(not(feature = "json"))]
        mod impl_ob_resp_gene {$(
            impl<T> super::OBRespData for $types {}
        )*}
    };
}

impl_ob_resp_data!(
    (),
    bool,
    u8,
    u16,
    u32,
    u64,
    i8,
    i16,
    i32,
    i64,
    f32,
    f64,
    String
);

impl_ob_resp_gene!(Vec<T>);

pub trait OBAction {
    type Resp: OBRespData;

    fn action(&self) -> &str;
}

#[cfg_attr(feature = "json", derive(serde::Deserialize, serde::Serialize))]
pub struct ActionRaw<'a> {
    pub action: Cow<'a, str>,
    pub params: JSONValue,
    #[cfg_attr(feature = "json", serde(flatten))]
    pub extra: JSONValue,
}
#[cfg_attr(feature = "json", derive(serde::Deserialize, serde::Serialize))]
pub struct RespRaw(#[allow(dead_code)] JSONValue);

#[cfg(feature = "json")]
mod serde_impl {
    use crate::{ActionRaw, OBAction};

    impl<T: serde::de::DeserializeOwned + serde::Serialize> super::OBRespData for T {
        fn from_json_raw(data: serde_json::Value) -> super::OBResult<Self>
        where
            Self: Sized,
        {
            Ok(serde_json::from_value(data)?)
        }
    }

    impl<'a> OBAction for ActionRaw<'a> {
        type Resp = super::RespRaw;
        fn action(&self) -> &str {
            self.action.as_ref()
        }
    }
}
