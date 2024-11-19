use std::time::Duration;

use ob_types_base::tool::duration_secs;
use ob_types_macro::json;

#[derive(serde::Serialize, Clone, Debug)]
pub enum NoticeEvent {
    GroupNotice(GroupNotice),
    FriendNotice(FriendNotice),
}

impl<'de> serde::Deserialize<'de> for NoticeEvent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;
        use serde_value::Value;
        let v = Value::deserialize(deserializer)?;
        let Value::String(s) = v
            .get("notice_type")
            .ok_or_else(|| serde::de::Error::custom("missing field `notice_type`"))?
        else {
            return Err(serde::de::Error::custom("invalid type: not a string"));
        };
        if s.starts_with("group") {
            GroupNotice::deserialize(v).map(NoticeEvent::GroupNotice)
        } else {
            FriendNotice::deserialize(v).map(NoticeEvent::FriendNotice)
        }
        .map_err(Error::custom)
    }
}

#[json]
pub struct GroupNotice {
    pub group_id: i64,
    pub user_id: i64,
    #[serde(flatten)]
    pub kind: GroupNoticeKind,
}

#[json]
#[serde(tag = "notice_type", rename_all = "snake_case")]
pub enum GroupNoticeKind {
    #[serde(rename = "group_upload")]
    Upload {
        file: GroupUpload,
    },
    #[serde(rename = "group_admin")]
    Admin {
        sub_type: AdminChange,
    },
    #[serde(rename = "group_increase")]
    MemberIncrease {
        sub_type: IncreaseType,
        operator_id: i64,
    },
    #[serde(rename = "group_decrease")]
    MemberDecrease {
        sub_type: DecreaseType,
        operator_id: i64,
    },
    #[serde(rename = "group_ban")]
    Mute {
        sub_type: MuteType,
        operator_id: i64,
        #[serde(with = "duration_secs")]
        duration: Duration,
    },
    #[serde(rename = "group_recall")]
    Recall {
        operator_id: i64,
        message_id: i32,
    },
    /// poke target user id
    Poke {
        target_id: i64,
    },
    /// lucky king user id
    LuckyKing {
        target_id: i64,
    },
    Honor {
        honor_type: GroupHonor,
    },
}

#[json]
pub struct GroupUpload {
    pub id: String,
    pub name: String,
    pub size: u64,
    pub busid: i64,
}

#[json]
#[serde(rename_all = "snake_case")]
pub enum AdminChange {
    Set,
    Unset,
}

#[json]
#[serde(rename_all = "snake_case")]
pub enum IncreaseType {
    Approve,
    Invite,
}

#[json]
#[serde(rename_all = "snake_case")]
pub enum DecreaseType {
    Leave,
    Kick,
    KickMe,
}

#[json]
#[serde(rename_all = "snake_case")]
pub enum MuteType {
    Ban,
    LiftBan,
}

#[json]
#[serde(rename_all = "snake_case")]
pub enum GroupHonor {
    Talkative,
    Performer,
    Emotion,
}

#[json]
pub struct FriendNotice {
    pub user_id: i64,
    #[serde(flatten)]
    pub kind: FriendNoticeKind,
}

#[json]
#[serde(tag = "notice_type", rename_all = "snake_case")]
pub enum FriendNoticeKind {
    FriendAdd,
    #[serde(rename = "friend_recall")]
    /// recalled message's id
    Recall {
        message_id: i32,
    },
}
