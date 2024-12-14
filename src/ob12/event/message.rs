use ob_types_macro::__data;

use crate::{
    ob12::{message::MessageChain, BotSelf},
    ValueMap,
};

use super::EventDetailed;

#[__data]
pub struct MessageArgs {
    pub message_id: String,
    pub user_id: String,
    pub sub_type: String,
    pub message: MessageChain,
    pub alt_message: Option<String>,
    pub extra: ValueMap,
}

#[__data]
pub struct GroupMessage {
    pub group_id: String,
    pub channel_id: String,
    #[serde(flatten)]
    pub args: MessageArgs,
}

#[__data]
pub struct ChannelMessage {
    pub guild_id: String,
    #[serde(flatten)]
    pub args: MessageArgs,
}

#[__data]
#[serde(tag = "detail_type")]
pub enum MessageKind {
    Private(MessageArgs),
    Group(GroupMessage),
    Channel(ChannelMessage),
    #[serde(untagged)]
    Other(EventDetailed),
}

#[__data]
pub struct MessageEvent {
    #[serde(rename = "self")]
    pub self_: BotSelf,
    #[serde(flatten)]
    pub kind: MessageKind,
}

impl From<MessageEvent> for super::EventType {
    fn from(value: MessageEvent) -> Self {
        super::EventType::Message(value)
    }
}
