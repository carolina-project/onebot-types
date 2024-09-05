pub mod action;
pub mod event;
pub mod message;

#[derive(Debug, Clone)]
#[cfg_attr(not(target_arch = "wasm32"), derive(serde::Serialize, serde::Deserialize))]
pub struct BotSelf {
    pub platform: String,
    pub user_id: String,
}
