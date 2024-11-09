use std::time::Duration;

use ob_types_macro::{json, onebot_action};

use crate::{
    ob12::{message::MessageChain, ChatTarget},
    scalable_struct,
};

use super::EmptyResp;

scalable_struct! {
    #[json(resp)]
    SendMessageResp = {
        message_id: String,
        #[serde(with = "ob_types_base::tool::duration_f64")]
        time: Duration,
    }
}

scalable_struct! {
    #[onebot_action(SendMessageResp)]
    SendMessage = {
        #[serde(flatten)]
        target: ChatTarget,
        message: MessageChain,
    },
    #[onebot_action(EmptyResp)]
    DeleteMessage = {
        message_id: String
    }
}
