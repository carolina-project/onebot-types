use ob_types_macro::{OBEvent, __data};

use crate::{base::MessageChain, ob12::BotSelf, ValueMap};

use super::{impl_from_into, EventDetailed, EventType};

#[__data]
pub struct MessageArgs {
    #[serde(rename = "self")]
    pub self_: BotSelf,
    pub message_id: String,
    pub user_id: String,
    pub sub_type: String,
    pub message: MessageChain,
    pub alt_message: Option<String>,
    #[serde(flatten)]
    pub extra: ValueMap,
}

#[__data]
#[derive(OBEvent)]
#[event(__crate_path = crate, type = "message")]
#[serde(transparent)]
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
#[serde(tag = "detail_type", rename_all = "snake_case")]
pub enum MessageEvent {
    Private(Private),
    Group(Group),
    Channel(Channel),
    #[serde(untagged)]
    Other(EventDetailed),
}

impl_from_into!(MessageEvent, EventType::Message);
