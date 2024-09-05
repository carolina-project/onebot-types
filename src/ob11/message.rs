use crate::cross::Data;

#[derive(Clone, Debug)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct OB11MessageSeg {
    pub r#type: String,
    pub data: Data,
}

impl OB11MessageSeg {
    pub fn from_cq_string(cq_string: &str) -> Vec<OB11MessageSeg> {
        let mut segments = vec![];

        let mut current_text = String::new();
        todo!()
    }
}
