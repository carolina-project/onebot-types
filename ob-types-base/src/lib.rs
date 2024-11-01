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

#[cfg(feature = "json")]
mod serde_impl {
    impl<T: serde::de::DeserializeOwned + serde::Serialize> super::OBRespData for T {
        fn from_json_raw(data: serde_json::Value) -> super::OBResult<Self>
        where
            Self: Sized,
        {
            Ok(serde_json::from_value(data)?)
        }
    }
}
