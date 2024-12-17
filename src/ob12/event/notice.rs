use ob_types_macro::{OBEvent, __data};

use crate::ob12::BotSelf;

use super::{impl_from_into, EventType};

#[__data]
#[serde(rename_all = "snake_case")]
pub enum IncreaseType {
    Join,
    Invite,
    #[serde(untagged)]
    Other(String),
}

#[__data]
#[serde(rename_all = "snake_case")]
pub enum DecreaseType {
    Kick,
    Leave,
    #[serde(untagged)]
    Other(String),
}

#[__data]
#[serde(rename_all = "snake_case")]
pub enum MessageDeleteType {
    Delete,
    Recall,
    #[serde(untagged)]
    Other(String),
}

macro_rules! notice_kinds {
    {$(
        $kind:ident {
            $($field:ident: $ty:ty),* $(,)?
        },
    )*} => {
        $(
            #[__data]
            #[derive(OBEvent)]
            #[event(__crate_path = crate, type = "notice")]
            pub struct $kind {
                #[serde(rename = "self")]
                pub self_: BotSelf,
                $(pub $field: $ty,)*
                #[serde(flatten)]
                pub extra: crate::ValueMap,
            }

            impl From<$kind> for NoticeEvent {
                fn from(kind: $kind) -> Self {
                    Self::$kind(kind)
                }
            }
        )*

        #[__data]
        #[serde(tag = "detail_type", rename_all = "snake_case")]
        pub enum NoticeEvent {
            $(
            $kind($kind),
            )*
            #[serde(untagged)]
            Other(super::EventDetailed),
        }
    };
}

notice_kinds! {
    FriendIncrease {
        sub_type: String,
        user_id: String,
    },
    FriendDecrease {
        sub_type: String,
        user_id: String,
    },
    PrivateMessageDelete {
        sub_type: String,
        message_id: String,
        user_id: String,
    },
    GroupMemberIncrease {
        sub_type: IncreaseType,
        group_id: String,
        user_id: String,
        operator_id: String,
    },
    GroupMemberDecrease {
        sub_type: DecreaseType,
        group_id: String,
        user_id: String,
        operator_id: String,
    },
    GroupMessageDelete {
        sub_type: MessageDeleteType,
        message_id: String,
        group_id: String,
        user_id: String,
        operator_id: String,
    },
    GuildMemberIncrease {
        sub_type: IncreaseType,
        guild_id: String,
        user_id: String,
        operator_id: String,
    },
    GuildMemberDecrease {
        sub_type: DecreaseType,
        guild_id: String,
        user_id: String,
        operator_id: String,
    },
    ChannelMemberIncrease {
        sub_type: IncreaseType,
        guild_id: String,
        channel_id: String,
        user_id: String,
        operator_id: String,
    },
    ChannelMemberDecrease {
        sub_type: DecreaseType,
        guild_id: String,
        channel_id: String,
        user_id: String,
        operator_id: String,
    },
    ChannelMessageDelete {
        sub_type: MessageDeleteType,
        message_id: String,
        guild_id: String,
        channel_id: String,
        user_id: String,
        operator_id: String,
    },
    ChannelCreate {
        sub_type: String,
        guild_id: String,
        channel_id: String,
        operator_id: String,
    },
    ChannelDelete {
        sub_type: String,
        guild_id: String,
        channel_id: String,
        operator_id: String,
    },
}

impl_from_into!(NoticeEvent, EventType::Notice);
