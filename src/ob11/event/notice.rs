use std::time::Duration;

use ob_types_base::tool::duration_secs;
use ob_types_macro::data;

use crate::ValueMap;

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

        #[derive(serde::Deserialize)]
        struct Helper {
            notice_type: String,
            #[serde(flatten)]
            extra: ValueMap,
        }
        let Helper {
            notice_type,
            mut extra,
        } = Helper::deserialize(deserializer)?;

        if notice_type.starts_with("group") {
            extra.insert(
                Value::String("notice_type".into()),
                Value::String(notice_type),
            );
            GroupNotice::deserialize(Value::Map(extra)).map(NoticeEvent::GroupNotice)
        } else {
            extra.insert(
                Value::String("notice_type".into()),
                Value::String(notice_type),
            );
            FriendNotice::deserialize(Value::Map(extra)).map(NoticeEvent::FriendNotice)
        }
        .map_err(Error::custom)
    }
}

#[data]
pub struct GroupNotice {
    pub group_id: i64,
    pub user_id: i64,
    #[serde(flatten)]
    pub kind: GroupNoticeKind,
}

macro_rules! group_notice {
    {
        $(
            $(#[$meta:meta])*
            $name:ident {
                $(
                    $(#[$f_meta:meta])*
                    $field_name:ident: $field_type:ty
                ),* $(,)?
            }
        ),* $(,)?
    } => {
        $(
            $(#[$meta])*
            #[data]
            pub struct $name {
                $(
                    $(#[$f_meta])*
                    pub $field_name: $field_type
                ),*
            }
        )*

        #[data]
        #[serde(tag = "notice_type", rename_all = "snake_case")]
        pub enum GroupNoticeKind {$(
            $name($name)
        ),*}
    };
}

group_notice! {
    GroupUpload {
        file: GroupUploadFile,
    },
    GroupAdmin {
        sub_type: AdminChange,
    },
    GroupIncrease {
        sub_type: IncreaseType,
        operator_id: i64,
    },
    GroupDecrease {
        sub_type: DecreaseType,
        operator_id: i64,
    },
    GroupBan {
        sub_type: MuteType,
        operator_id: i64,
        #[serde(with = "duration_secs")]
        duration: Duration,
    },
    GroupRecall {
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

#[data]
pub struct GroupUploadFile {
    pub id: String,
    pub name: String,
    pub size: u64,
    pub busid: i64,
}

#[data]
#[serde(rename_all = "snake_case")]
pub enum AdminChange {
    Set,
    Unset,
}

#[data]
#[serde(rename_all = "snake_case")]
pub enum IncreaseType {
    Approve,
    Invite,
}

#[data]
#[serde(rename_all = "snake_case")]
pub enum DecreaseType {
    Leave,
    Kick,
    KickMe,
}

#[data]
#[serde(rename_all = "snake_case")]
pub enum MuteType {
    Ban,
    LiftBan,
}

#[data]
#[serde(rename_all = "snake_case")]
pub enum GroupHonor {
    Talkative,
    Performer,
    Emotion,
}

#[data]
pub struct FriendNotice {
    pub user_id: i64,
    #[serde(flatten)]
    pub kind: FriendNoticeKind,
}

#[data]
#[serde(tag = "notice_type", rename_all = "snake_case")]
pub enum FriendNoticeKind {
    FriendAdd,
    #[serde(rename = "friend_recall")]
    /// recalled message's id
    Recall {
        message_id: i32,
    },
}
