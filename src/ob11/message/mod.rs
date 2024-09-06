use crate::cross::Data;

pub mod types;

#[derive(Clone, Debug)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct OB11MessageSegRaw {
    pub r#type: String,
    pub data: Data,
}
