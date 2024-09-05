use crate::cross::Data;

#[derive(Clone, Debug)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct OB11Action {
    pub action: String,
    pub params: Data,
}

#[derive(Clone, Copy, Debug)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "lowercase")
)]
pub enum OB11RespStatus {
    Ok,
    Async,
    Failed,
}

#[derive(Clone, Debug)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct OB11Resp {
    pub status: OB11RespStatus,
    pub retcode: i64,
    pub data: Data,
}
