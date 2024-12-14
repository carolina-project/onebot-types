use std::time::Duration;

use ob_types_base::tool::duration_secs;
use ob_types_macro::data;

use crate::ValueMap;

#[data]
pub struct NoticeRaw {
    pub notice_type: String,
    #[serde(flatten)]
    pub detail: ValueMap,
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
        #[data]
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
        #[data]
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
        #[data]
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

        #[data]
        #[serde(tag = "notice_type", rename_all = "snake_case")]
        pub enum NoticeEvent {
            $(
                $name($name),
            )*
            Other(NoticeRaw),
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
