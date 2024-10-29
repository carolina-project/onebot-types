use ob_types_base::tool::{duration_from_seconds, duration_to_seconds};
use std::time::Duration;

use ob_types_macro::json;

#[json(serde(untagged))]
pub enum NoticeEvent {
    GroupNotice(GroupNotice),
    FriendNotice(FriendNotice),
}

#[json]
pub struct GroupNotice {
    pub group_id: u64,
    pub user_id: u64,
    #[cfg_attr(feature = "json", serde(flatten))]
    pub kind: GroupNoticeKind,
}

#[json]
pub enum GroupNoticeKind {
    Upload(#[cfg_attr(feature = "json", serde(rename = "file"))] GroupUpload),
    Admin(#[cfg_attr(feature = "json", serde(rename = "sub_type"))] AdminChange),
    MemberIncrease {
        sub_type: IncreaseType,
        operator_id: u64,
    },
    MemberDecrease {
        sub_type: DecreaseType,
        operator_id: u64,
    },
    Mute {
        sub_type: MuteType,
        operator_id: u64,
        #[cfg_attr(
            feature = "json",
            serde(
                deserialize_with = "duration_from_seconds",
                serialize_with = "duration_to_seconds"
            )
        )]
        duration: Duration,
    },
    Recall {
        operator_id: u64,
        message_id: u32,
    },
    /// poke target user id
    Poke(#[cfg_attr(feature = "json", serde(rename = "target_id"))] u64),
    /// lucky king user id
    LuckyKing(u64),
    Honor(GroupHonor),
}

#[json]
pub struct GroupUpload {
    pub id: String,
    pub name: String,
    pub size: u64,
    pub busid: u64,
}

#[json(serde(rename_all = "lowercase"))]
pub enum AdminChange {
    Set,
    Unset,
}

#[json(serde(rename_all = "lowercase"))]
pub enum IncreaseType {
    Approve,
    Invite,
}

#[json(serde(rename_all = "lowercase"))]
pub enum DecreaseType {
    Leave,
    Kick,
    KickMe,
}

#[json(serde(rename_all = "lowercase"))]
pub enum MuteType {
    Ban,
    LiftBan,
}

#[json(serde(rename_all = "lowercase"))]
pub enum GroupHonor {
    Talkative,
    Performer,
    Emotion,
}

pub struct FriendNotice {
    pub user_id: u64,
    pub kind: FriendNoticeKind,
}

pub enum FriendNoticeKind {
    FriendAdd,
    /// recalled message's id
    Recall(u32),
}
