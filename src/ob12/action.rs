use crate::cross::Data;

#[derive(Clone, Debug)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct OB12ActionData {
    pub action: String,
    pub echo: Option<String>,
    #[cfg_attr(not(target_arch = "wasm32"), serde(rename = "self"))]
    pub self_: super::BotSelf,
    pub params: Data,
}

#[derive(Clone, Copy, Debug)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "lowercase")
)]
pub enum OB12RespStatus {
    Ok,
    Failed,
}

#[derive(Clone, Debug)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct OB12RespData {
    pub status: OB12RespStatus,
    pub retcode: i64,
    pub data: Data,
    pub message: String,
    pub echo: Option<String>,
}
