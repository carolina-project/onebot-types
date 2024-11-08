use ob_types_macro::json;

use crate::{ob12::BotSelf, scalable_struct};

#[json(serde(rename_all = "snake_case"))]
pub enum IncreaseType {
    Join,
    Invite,
    #[serde(untagged)]
    Extra(String),
}

#[json(serde(rename_all = "snake_case"))]
pub enum DecreaseType {
    Kick,
    Leave,
    #[serde(untagged)]
    Extra(String),
}

#[json(serde(rename_all = "snake_case"))]
pub enum MessageDeleteType {
    Delete,
    Recall,
    #[serde(untagged)]
    Extra(String),
}

#[json]
pub struct GroupTarget {
    pub group_id: String,
    pub user_id: String,
    pub operator_id: String,
}

#[json]
pub struct GuildTarget {
    pub guild_id: String,
    pub user_id: String,
    pub operator_id: String,
}

#[json]
pub struct ChannelTarget {
    pub guild_id: String,
    pub channel_id: String,
    pub user_id: String,
    pub operator_id: String,
}

#[json(serde(tag = "detail_type", rename_all = "snake_case"))]
pub enum NoticeKind {
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
        #[serde(flatten)]
        target: GroupTarget,
    },
    GroupDecrease {
        sub_type: DecreaseType,
        #[serde(flatten)]
        target: GroupTarget,
    },
    GroupMessageDelete {
        sub_type: MessageDeleteType,
        message_id: String,
        #[serde(flatten)]
        target: GroupTarget,
    },
    GuildMemberIncrease {
        sub_type: IncreaseType,
        #[serde(flatten)]
        target: GuildTarget,
    },
    GuildMemberDecrease {
        sub_type: DecreaseType,
        #[serde(flatten)]
        target: GuildTarget,
    },
    ChannelMemberIncrease {
        sub_type: IncreaseType,
        #[serde(flatten)]
        target: ChannelTarget,
    },
    ChannelMemberDecrease {
        sub_type: DecreaseType,
        #[serde(flatten)]
        target: ChannelTarget,
    },
    ChannelMessageDelete {
        sub_type: MessageDeleteType,
        message_id: String,
        #[serde(flatten)]
        target: ChannelTarget,
    },
    ChannelCreate(ChannelTarget),
    ChannelDelete(ChannelTarget),
}

scalable_struct! {
    NoticeEvent = {
        #[serde(rename = "self")]
        self_: BotSelf,
        #[serde(flatten)]
        kind: NoticeKind,
    },
}
