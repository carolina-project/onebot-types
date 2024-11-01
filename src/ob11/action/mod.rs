use ob_types_base::json::JSONValue;
use ob_types_macro::json;

pub mod bot;
pub mod friend;
pub mod group;

pub type EmptyResp = ();

#[derive(Clone)]
#[json]
pub struct OB11ActionRaw {
    pub action: String,
    pub params: JSONValue,
    pub echo: Option<String>,
}

#[derive(Clone, Copy)]
#[json(serde(rename_all = "lowercase"))]
pub enum OB11RespStatus {
    Ok,
    Async,
    Failed,
}

#[derive(Clone)]
#[json]
pub struct OB11RespData {
    pub status: OB11RespStatus,
    pub retcode: i64,
    pub data: JSONValue,
    pub echo: Option<String>,
}
