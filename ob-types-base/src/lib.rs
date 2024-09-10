use cross::Data;
use error::OBResult;
use serde::Serialize;

pub mod cross;
pub mod error;

pub trait OBRespData {
    fn from_data(data: Data) -> OBResult<Self>
    where
        Self: Sized;
}

pub trait ToData {
    fn to_data(self) -> OBResult<Data>;
}

pub trait OBAction {
    type Resp: OBRespData;

    fn action(&self) -> &str;
}

#[cfg(not(target_arch = "wasm32"))]
impl<T: Serialize> ToData for T {
    fn to_data(self) -> OBResult<Data> {
        Ok(serde_json::to_value(self)?)
    }
}
