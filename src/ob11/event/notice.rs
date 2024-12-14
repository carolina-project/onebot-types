use std::time::Duration;

use crate::base::tool::duration_secs;

use ob_types_macro::__data;
use serde::{de::IntoDeserializer, ser::Error, Deserialize};
use serde_value::{DeserializerError, SerializerError};

use crate::ValueMap;

#[__data]
pub struct NoticeDetail {
    pub notice_type: String,
    #[serde(flatten)]
    pub detail: ValueMap,
}

impl TryFrom<NoticeDetail> for NoticeEvent {
    type Error = DeserializerError;

    fn try_from(detail: NoticeDetail) -> Result<Self, Self::Error> {
        let NoticeDetail {
            notice_type,
            mut detail,
        } = detail;
        detail.insert("notice_type".into(), notice_type.into_value());

        Ok(Deserialize::deserialize(detail.into_deserializer())?)
    }
}

impl TryInto<NoticeDetail> for NoticeEvent {
    type Error = SerializerError;

    fn try_into(self) -> Result<NoticeDetail, Self::Error> {
        NoticeDetail::deserialize(serde_value::to_value(self)?).map_err(SerializerError::custom)
    }
}

macro_rules! mk_struct {
    {
        $(#[$meta:meta])*
        $name:ident {
            $(
                $(#[$f_meta:meta])*
                $field_name:ident: $field_type:ty
            ),* $(,)?
        }
    } => {
        $(#[$meta])*
        #[__data]
        pub struct $name {
            $(
                $(#[$f_meta])*
                pub $field_name: $field_type
            ),*
        }
    };
    {
        $(#[$meta:meta])*
        $name:ident group {
            $(
                $(#[$f_meta:meta])*
                $field_name:ident: $field_type:ty
            ),* $(,)?
        }
    } => {
        $(#[$meta])*
        #[__data]
        pub struct $name {
            pub group_id: i64,
            pub user_id: i64,
            $(
                $(#[$f_meta])*
                pub $field_name: $field_type
            ),*
        }
    };
    {
        $(#[$meta:meta])*
        $name:ident friend {
            $(
                $(#[$f_meta:meta])*
                $field_name:ident: $field_type:ty
            ),* $(,)?
        }
    } => {
        $(#[$meta])*
        #[__data]
        pub struct $name {
            pub user_id: i64,
            $(
                $(#[$f_meta])*
                pub $field_name: $field_type
            ),*
        }
    };
}

macro_rules! define_notice {
    {
        $(
            $(#[$meta:meta])*
            $name:ident $($typ:ident)? {
                $(
                    $(#[$f_meta:meta])*
                    $field_name:ident: $field_type:ty
                ),* $(,)?
            }
        ),* $(,)?
    } => {
        $(
            mk_struct! {
                $(#[$meta])*
                $name $($typ)? {
                    $(
                        $(#[$f_meta])*
                        $field_name: $field_type
                    ),*
                }
            }
        )*

        #[__data]
        #[serde(tag = "notice_type", rename_all = "snake_case")]
        pub enum NoticeEvent {
            $(
                $name($name),
            )*
        }
    };
}

define_notice! {
    GroupUpload group {
        file: GroupUploadFile,
    },
    GroupAdmin group {
        sub_type: AdminChange,
    },
    GroupIncrease group {
        sub_type: IncreaseType,
        operator_id: i64,
    },
    GroupDecrease group {
        sub_type: DecreaseType,
        operator_id: i64,
    },
    GroupBan group {
        sub_type: MuteType,
        operator_id: i64,
        #[serde(with = "duration_secs")]
        duration: Duration,
    },
    GroupRecall group {
        operator_id: i64,
        message_id: i32,
    },
    Poke group {
        target_id: i64,
    },
    LuckyKing group {
        target_id: i64,
    },
    Honor group {
        honor_type: GroupHonor,
    },
    FriendAdd friend {},
    FriendRecall friend {
        message_id: i32,
    },
}

#[__data]
pub struct GroupUploadFile {
    pub id: String,
    pub name: String,
    pub size: u64,
    pub busid: i64,
}

#[__data]
#[serde(rename_all = "snake_case")]
pub enum AdminChange {
    Set,
    Unset,
}

#[__data]
#[serde(rename_all = "snake_case")]
pub enum IncreaseType {
    Approve,
    Invite,
}

#[__data]
#[serde(rename_all = "snake_case")]
pub enum DecreaseType {
    Leave,
    Kick,
    KickMe,
}

#[__data]
#[serde(rename_all = "snake_case")]
pub enum MuteType {
    Ban,
    LiftBan,
}

#[__data]
#[serde(rename_all = "snake_case")]
pub enum GroupHonor {
    Talkative,
    Performer,
    Emotion,
}
