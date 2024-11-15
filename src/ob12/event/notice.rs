use ob_types_macro::json;

use crate::ob12::BotSelf;
use ob_types_base::JSONValue;

#[json(serde(rename_all = "snake_case"))]
pub enum IncreaseType {
    Join,
    Invite,
    #[serde(untagged)]
    Other(String),
}

#[json(serde(rename_all = "snake_case"))]
pub enum DecreaseType {
    Kick,
    Leave,
    #[serde(untagged)]
    Other(String),
}

#[json(serde(rename_all = "snake_case"))]
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
            #[json(resp)]
            pub struct $kind {
                $(pub $field: $ty,)*
                #[serde(flatten)]
                pub extra: JSONValue,
            }
        )*

        #[json(serde(tag = "detail_type", rename_all = "snake_case"))]
        pub enum NoticeKind {
            $(
            $kind($kind),
            )*
            //#[serde(untagged)]
            //Other {
            //    detail_type: String,
            //    #[serde(flatten)]
            //    data: JSONValue,
            //},
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
    GroupIncrease {
        sub_type: IncreaseType,
        group_id: String,
        user_id: String,
        operator_id: String,
    },
    GroupDecrease {
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
        guild_id: String,
        channel_id: String,
        user_id: String,
        operator_id: String,
    },
    ChannelDelete {
        guild_id: String,
        channel_id: String,
        user_id: String,
        operator_id: String,
    },
}

#[json(resp)]
pub struct NoticeEvent {
    #[serde(rename = "self")]
    self_: BotSelf,
    #[serde(flatten)]
    kind: NoticeKind,
}
