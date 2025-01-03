use crate::{base::MessageChain, ob12::ChatTarget, scalable_struct};

use super::EmptyResp;

scalable_struct! {
    SendMessageResp = {
        message_id: String,
        time: f64,
    }
}

scalable_struct! {
    #[resp(SendMessageResp)]
    SendMessage = {
        #[serde(flatten)]
        target: ChatTarget,
        message: MessageChain,
    },
    #[resp(EmptyResp)]
    DeleteMessage = {
        message_id: String
    }
}
