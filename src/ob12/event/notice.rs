use ob_types_macro::json;

use crate::ob12::BotSelf;

use super::EventType;

#[json]
#[serde(rename_all = "snake_case")]
pub enum IncreaseType {
    Join,
    Invite,
    #[serde(untagged)]
    Other(String),
}

#[json]
#[serde(rename_all = "snake_case")]
pub enum DecreaseType {
    Kick,
    Leave,
    #[serde(untagged)]
    Other(String),
}

#[json]
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
            #[json]
            pub struct $kind {
                $(pub $field: $ty,)*
                #[serde(flatten)]
                pub extra: serde_value::Value,
            }
        )*

        #[json]
        #[serde(tag = "detail_type", rename_all = "snake_case")]
        pub enum NoticeKind {
            $(
            $kind($kind),
            )*
            #[serde(untagged)]
            Other {
                detail_type: String,
                #[serde(flatten)]
                data: serde_value::Value,
            },
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

#[json]
pub struct NoticeEvent {
    #[serde(rename = "self")]
    self_: BotSelf,
    #[serde(flatten)]
    kind: NoticeKind,
}

impl From<NoticeEvent> for EventType {
    fn from(value: NoticeEvent) -> Self {
        Self::Notice(value)
    }
}
