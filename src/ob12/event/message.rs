use crate::ob12::{message::MessageChain, BotSelf, ChatTarget};

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
    #[serde(flatten, deserialize_with = "de_extra")]
    pub extra: serde_value::Value,
}

impl From<MessageEvent> for super::EventType {
    fn from(value: MessageEvent) -> Self {
        super::EventType::Message(value)
    }
}

mod serde_impl {
    use std::collections::BTreeMap;

    use crate::ob12::{message::MessageChain, BotSelf, ChatTarget, CHAT_TARGET_FIELDS};
    use serde::Deserialize;
    use serde_value::Value;

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
        pub extra: BTreeMap<Value, Value>,
    }

    use super::MessageEvent;
    impl<'de> Deserialize<'de> for MessageEvent {
        fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let mut helper = DeHelper::deserialize(deserializer)?;

            let extra: Value = match helper.source {
                ChatTarget::Other { detail_type: _ } => {
                    helper
                        .extra
                        .remove_entry(&Value::String("detail_type".into()));
                    Value::Map(helper.extra)
                }
                _ => {
                    for ele in CHAT_TARGET_FIELDS {
                        helper.extra.remove(&Value::String((*ele).to_owned()));
                    }
                    Value::Map(helper.extra)
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
