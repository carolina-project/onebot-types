use ob_types_base::{json::JSONValue, OBAction};
use ob_types_macro::json;

mod bot;
mod friend;
mod group;

pub use bot::*;
pub use friend::*;
pub use group::*;

pub type EmptyResp = ();

#[json]
pub struct ActionRaw<'a, T: OBAction> {
    pub action: &'a str,
    pub params: T,
    pub echo: Option<String>,
}

#[derive(Copy)]
#[json(serde(rename_all = "lowercase"))]
pub enum RespStatus {
    Ok,
    Async,
    Failed,
}

#[json]
pub struct RespData {
    pub status: RespStatus,
    pub retcode: i64,
    pub data: JSONValue,
    pub echo: Option<String>,
}
