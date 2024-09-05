use crate::cross::Data;

#[derive(Clone, Debug)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct OB12MessageSeg {
    pub r#type: String,
    pub data: Data,
    pub alt_message: Option<String>,
}
