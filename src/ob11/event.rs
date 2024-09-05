use crate::cross::Data;

#[derive(Clone, Copy, Debug)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
pub enum OB11PostType {
    MetaEvent,
    Message,
    Notice,
    Request,
}

#[derive(Clone, Debug)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct OB11Event {
    time: i64,
    self_id: i64,
    post_type: OB11PostType,
    #[cfg_attr(not(target_arch = "wasm32"), serde(flatten))]
    pub extra: Data,
}
