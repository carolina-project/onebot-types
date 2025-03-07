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

impl SendMessage {
    pub fn new(target: ChatTarget, message: impl Into<MessageChain>) -> Self {
        Self {
            target,
            message: message.into(),
            extra: Default::default(),
        }
    }
}

impl DeleteMessage {
    pub fn new(message_id: impl Into<String>) -> Self {
        Self {
            message_id: message_id.into(),
            extra: Default::default(),
        }
    }
}
