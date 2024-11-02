use ob_types_base::{json::JSONValue, OBAction};
use ob_types_macro::json;

pub mod bot;
pub mod friend;
pub mod group;

pub type EmptyResp = ();

#[json]
pub struct OB11ActionRaw<'a, T: OBAction> {
    pub action: &'a str,
    pub params: T,
    pub echo: Option<String>,
}

#[derive(Copy)]
#[json(serde(rename_all = "lowercase"))]
pub enum OB11RespStatus {
    Ok,
    Async,
    Failed,
}

#[json]
pub struct OB11RespData {
    pub status: OB11RespStatus,
    pub retcode: i64,
    pub data: JSONValue,
    pub echo: Option<String>,
}
