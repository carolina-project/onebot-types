use ob_types_macro::json;

pub mod action;
pub mod event;
pub mod message;

#[derive(Debug, Clone)]
#[json]
pub struct BotSelf {
    pub platform: String,
    pub user_id: String,
}
