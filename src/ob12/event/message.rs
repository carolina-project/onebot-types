use crate::{
    ob12::{message::MessageChain, BotSelf, ChatTarget},
    ValueMap,
};

#[derive(serde::Serialize, Debug, Clone)]
pub struct MessageEvent {
    #[serde(rename = "self")]
    pub self_: BotSelf,
    pub message_id: String,
    pub sub_type: String,
    pub message: MessageChain,
    pub alt_message: Option<String>,
    #[serde(flatten)]
    pub source: ChatTarget,
    #[serde(flatten)]
    pub extra: ValueMap,
}

impl From<MessageEvent> for super::EventType {
    fn from(value: MessageEvent) -> Self {
        super::EventType::Message(value)
    }
}

mod serde_impl {

    use crate::{
        ob12::{message::MessageChain, BotSelf, ChatTarget, CHAT_TARGET_FIELDS},
        ValueMap,
    };
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct DeHelper {
        #[serde(rename = "self")]
        pub self_: BotSelf,
        pub message_id: String,
        pub sub_type: String,
        pub message: MessageChain,
        pub alt_message: Option<String>,
        #[serde(flatten)]
        pub source: ChatTarget,
        #[serde(flatten)]
        pub extra: ValueMap,
    }

    use super::MessageEvent;
    impl<'de> Deserialize<'de> for MessageEvent {
        fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let mut helper = DeHelper::deserialize(deserializer)?;

            let extra = match helper.source {
                ChatTarget::Other { detail_type: _ } => {
                    helper.extra.remove_entry("detail_type");
                    helper.extra
                }
                _ => {
                    for ele in CHAT_TARGET_FIELDS {
                        helper.extra.remove(*ele);
                    }
                    helper.extra
                }
            };
            Ok(Self {
                self_: helper.self_,
                message_id: helper.message_id,
                sub_type: helper.sub_type,
                message: helper.message,
                alt_message: helper.alt_message,
                source: helper.source,
                extra,
            })
        }
    }
}
