pub mod error;

use std::borrow::Cow;

pub use error::OBResult;

pub mod ext;
pub mod tool;

pub trait OBRespData<'de>: serde::de::Deserialize<'de> + serde::Serialize {}

impl<'de, T: serde::de::Deserialize<'de> + serde::Serialize> OBRespData<'de> for T {}

pub trait OBAction {
    const ACTION: Option<&'static str> = None;
    type Resp: OBRespData<'static>;

    fn action_name(&self) -> &str {
        Self::ACTION.expect("Action name not set")
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct ActionRaw<'a> {
    pub action: Cow<'a, str>,
    pub params: serde_value::Value,
    #[serde(flatten)]
    pub extra: serde_value::Value,
}
#[derive(serde::Deserialize, serde::Serialize)]
pub struct RespRaw(#[allow(dead_code)] serde_value::Value);

impl<'a> OBAction for ActionRaw<'a> {
    type Resp = RespRaw;
}
