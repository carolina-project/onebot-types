use std::time::Duration;

use ob_types_macro::{json, onebot_action};

use crate::ob12::{message::MessageChain, ChatTarget};
use ob_types_base::tool::duration_f64;

#[onebot_action(SendMessageResp)]
pub struct SendMessage {
    #[serde(flatten)]
    pub target: ChatTarget,
    pub message: MessageChain,
}

#[json(resp)]
pub struct SendMessageResp {
    pub message_id: String,
    #[serde(with = "duration_f64")]
    pub time: Duration
}
