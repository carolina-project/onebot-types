use std::borrow::Cow;

use ob_types_base::{json::JSONValue, OBAction};
use ob_types_macro::json;

#[json]
pub struct ActionRaw<'a, T: OBAction> {
    pub action: Cow<'a, str>,
    pub params: T,
    pub echo: Option<String>,
    #[serde(rename = "self")]
    pub self_: super::BotSelf,
}

#[derive(Copy)]
#[json(serde(rename_all = "lowercase"))]
pub enum RespStatus {
    Ok,
    Failed,
}

#[json]
pub struct RespData {
    pub status: RespStatus,
    pub retcode: i64,
    pub data: JSONValue,
    pub message: String,
    pub echo: Option<String>,
}
