use std::time::Duration;

#[cfg(feature = "json")]
use ob_types_base::tool::duration_secs;
use ob_types_macro::json;

#[cfg_attr(feature = "json", derive(serde::Serialize))]
#[derive(Clone, Debug)]
pub enum NoticeEvent {
    GroupNotice(GroupNotice),
    FriendNotice(FriendNotice),
}

#[cfg(feature = "json")]
impl<'de> serde::Deserialize<'de> for NoticeEvent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;
        use serde_json::Value;
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
    pub group_id: u64,
    pub user_id: u64,
    #[cfg_attr(feature = "json", serde(flatten))]
    pub kind: GroupNoticeKind,
}

#[json(serde(tag = "notice_type", rename_all = "snake_case"))]
pub enum GroupNoticeKind {
    #[cfg_attr(feature = "json", serde(rename = "group_upload"))]
    Upload {
        file: GroupUpload,
    },
    #[cfg_attr(feature = "json", serde(rename = "group_admin"))]
    Admin {
        sub_type: AdminChange,
    },
    #[cfg_attr(feature = "json", serde(rename = "group_increase"))]
    MemberIncrease {
        sub_type: IncreaseType,
        operator_id: u64,
    },
    #[cfg_attr(feature = "json", serde(rename = "group_decrease"))]
    MemberDecrease {
        sub_type: DecreaseType,
        operator_id: u64,
    },
    #[cfg_attr(feature = "json", serde(rename = "group_ban"))]
    Mute {
        sub_type: MuteType,
        operator_id: u64,
        #[cfg_attr(feature = "json", serde(with = "duration_secs"))]
        duration: Duration,
    },
    #[cfg_attr(feature = "json", serde(rename = "group_recall"))]
    Recall {
        operator_id: u64,
        message_id: u32,
    },
    /// poke target user id
    Poke {
        target_id: u64,
    },
    /// lucky king user id
    LuckyKing {
        target_id: u64,
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
    pub busid: u64,
}

#[json(serde(rename_all = "snake_case"))]
pub enum AdminChange {
    Set,
    Unset,
}

#[json(serde(rename_all = "snake_case"))]
pub enum IncreaseType {
    Approve,
    Invite,
}

#[json(serde(rename_all = "snake_case"))]
pub enum DecreaseType {
    Leave,
    Kick,
    KickMe,
}

#[json(serde(rename_all = "snake_case"))]
pub enum MuteType {
    Ban,
    LiftBan,
}

#[json(serde(rename_all = "snake_case"))]
pub enum GroupHonor {
    Talkative,
    Performer,
    Emotion,
}

#[json]
pub struct FriendNotice {
    pub user_id: u64,
    #[cfg_attr(feature = "json", serde(flatten))]
    pub kind: FriendNoticeKind,
}

#[json(serde(tag = "notice_type", rename_all = "snake_case"))]
pub enum FriendNoticeKind {
    FriendAdd,
    #[cfg_attr(feature = "json", serde(rename = "friend_recall"))]
    /// recalled message's id
    Recall(#[cfg_attr(feature = "json", serde(rename = "message_id"))] u32),
}
