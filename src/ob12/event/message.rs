use ob_types_macro::{OBEvent, __data};

use crate::{
    base::MessageChain,
    ob12::{BotSelf, ChatTarget},
    ValueMap,
};

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

impl MessageEvent {
    pub fn messages(&self) -> Option<&MessageChain> {
        Some(match self {
            MessageEvent::Private(private) => &private.0.message,
            MessageEvent::Group(group) => &group.args.message,
            MessageEvent::Channel(channel) => &channel.args.message,
            MessageEvent::Other(_) => return None,
        })
    }

    pub fn messages_mut(&mut self) -> Option<&mut MessageChain> {
        Some(match self {
            MessageEvent::Private(private) => &mut private.0.message,
            MessageEvent::Group(group) => &mut group.args.message,
            MessageEvent::Channel(channel) => &mut channel.args.message,
            MessageEvent::Other(_) => return None,
        })
    }

    pub fn get_chat_target(&self) -> Option<ChatTarget> {
        Some(match self {
            MessageEvent::Private(Private(args)) => ChatTarget::Private {
                user_id: args.user_id.clone(),
            },
            MessageEvent::Group(Group { group_id, .. }) => ChatTarget::Group {
                group_id: group_id.clone(),
            },
            MessageEvent::Channel(Channel {
                guild_id,
                channel_id,
                ..
            }) => ChatTarget::Channel {
                guild_id: guild_id.clone(),
                channel_id: channel_id.clone(),
            },
            MessageEvent::Other(_) => return None,
        })
    }

    pub fn get_self(&self) -> Option<&BotSelf> {
        match self {
            MessageEvent::Private(private) => Some(&private.0.self_),
            MessageEvent::Group(group) => Some(&group.args.self_),
            MessageEvent::Channel(channel) => Some(&channel.args.self_),
            MessageEvent::Other(_) => None,
        }
    }
}

impl_from_into!(MessageEvent, EventType::Message);
