pub mod error;
pub mod json;

pub use error::OBResult;
pub use json::JSONValue;

pub mod tool;

pub trait OBRespData {
    #[cfg(feature = "json")]
    fn from_json_raw(data: serde_json::Value) -> OBResult<Self>
    where
        Self: Sized;
}

pub trait OBAction {
    type Resp: OBRespData;

    fn action(&self) -> &str;
}

#[cfg(feature = "json")]
mod serde_impl {
    impl<T: serde::de::DeserializeOwned> super::OBRespData for T {
        fn from_json_raw(data: serde_json::Value) -> super::OBResult<Self>
        where
            Self: Sized,
        {
            Ok(serde_json::from_value(data)?)
        }
    }
}
