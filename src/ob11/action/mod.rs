use ob_types_base::cross::Data;
use ob_types_macro::native_data;

pub mod bot;
pub mod friend;
pub mod group;

#[derive(Clone, Debug)]
#[native_data]
pub struct OB11ActionRaw {
    pub action: String,
    pub params: Data,
    pub echo: Option<String>,
}

#[derive(Clone, Copy, Debug)]
#[native_data(
    serde(rename_all = "lowercase")
)]
pub enum OB11RespStatus {
    Ok,
    Async,
    Failed,
}

#[derive(Clone, Debug)]
#[native_data]
pub struct OB11RespData {
    pub status: OB11RespStatus,
    pub retcode: i64,
    pub data: Data,
    pub echo: Option<String>,
}
