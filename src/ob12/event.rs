use crate::cross::Data;

#[derive(Clone, Copy, Debug)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "lowercase")
)]
pub enum OB12EventType {
    Meta,
    Message,
    Notice,
    Request,
}

#[derive(Clone, Debug)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct OB12Event {
    pub id: String,
    pub time: f64,
    #[cfg_attr(not(target_arch = "wasm32"), serde(rename = "self"))]
    pub self_: super::BotSelf,
    pub r#type: OB12EventType,
    pub detail_type: String,
    pub sub_type: String,
    #[cfg_attr(not(target_arch = "wasm32"), serde(flatten))]
    pub extra: Data,
}
