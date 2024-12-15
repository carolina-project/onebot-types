use ob_types_macro::{OBEvent, __data};

use crate::{base::MessageChain, ob12::BotSelf, ValueMap};

use super::EventDetailed;

#[__data]
pub struct MessageArgs {
    #[serde(rename = "self")]
    pub self_: BotSelf,
    pub message_id: String,
    pub user_id: String,
    pub sub_type: String,
    pub message: MessageChain,
    pub alt_message: Option<String>,
    pub extra: ValueMap,
}

#[__data]
#[derive(OBEvent)]
#[event(__crate_path = crate, type = "message")]
pub struct Private(pub MessageArgs);

#[__data]
#[derive(OBEvent)]
#[event(__crate_path = crate, type = "message")]
pub struct Group {
    pub group_id: String,
    #[serde(flatten)]
    pub args: MessageArgs,
}

#[__data]
#[derive(OBEvent)]
#[event(__crate_path = crate, type = "message")]
pub struct Channel {
    pub guild_id: String,
    pub channel_id: String,
    #[serde(flatten)]
    pub args: MessageArgs,
}

#[__data]
#[serde(tag = "detail_type")]
pub enum MessageEvent {
    Private(Private),
    Group(Group),
    Channel(Channel),
    #[serde(untagged)]
    Other(EventDetailed),
}

impl From<MessageEvent> for super::EventKind {
    fn from(value: MessageEvent) -> Self {
        super::EventKind::Message(value)
    }
}
