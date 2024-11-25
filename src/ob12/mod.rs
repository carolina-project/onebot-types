use ob_types_macro::json;

pub mod action;
pub mod event;
pub mod message;

pub use message::MessageSeg;

mod macros {
    #[macro_export]
    macro_rules! scalable_struct {
        {
            $(#[$head_meta:meta])+
            $typ:ident $(= {
                $(
                    $(#[$meta:meta])*
                    $field:ident : $f_ty:ty
                ),* $(,)?
            })? $(, $($rest:tt)*)?
        } => {
            $(#[$head_meta])+
            pub struct $typ {
                $(
                    $(
                        $(#[$meta])*
                        pub $field: $f_ty,
                    )*)?
                #[serde(flatten)]
                pub extra: serde_value::Value,
            }

            $crate::scalable_struct! {
                $($($rest)*)?
            }

        };

        {
            $typ:ident $(= {
                $(
                    $(#[$meta:meta])*
                    $field:ident : $f_ty:ty
                ),* $(,)?
            })? $(, $($rest:tt)*)?
        } => {
            #[ob_types_macro::json]
            pub struct $typ {
                $(
                    $(
                        $(#[$meta])*
                        pub $field: $f_ty,
                    )*
                )?
                #[serde(flatten)]
                pub extra: serde_value::Value,
            }

            $crate::scalable_struct! {
                $($($rest)*)?
            }
        };
        {} => {};
    }

    pub use scalable_struct;
}

pub(self) use macros::scalable_struct;

#[json]
pub struct BotSelf {
    pub platform: String,
    pub user_id: String,
}

scalable_struct! {
    BotState = {
        #[serde(rename = "self")]
        self_: BotSelf,
        online: bool,
    },
    Status = {
        good: bool,
        bots: Vec<BotState>,
    },
    VersionInfo = {
        r#impl: String,
        version: String,
        onebot_version: String,
    },
    UserInfo = {
        user_id: String,
        user_name: String,
        user_display_name: String,
        user_remark: Option<String>,
    },
    GroupInfo = {
        group_id: String,
        group_name: String,
    },
    GroupMemberInfo = {
        group_id: String,
        user_id: String,
    },
    GuildInfo = {
        guild_id: String,
        guild_name: String,
    },
    ChannelInfo = {
        channel_id: String,
        channel_name: String,
    },
}

pub(crate) static CHAT_TARGET_FIELDS: &[&str] = &[
    "user_id",
    "group_id",
    "guild_id",
    "channel_id",
    "detail_type",
];

#[json]
#[serde(tag = "detail_type", rename_all = "snake_case")]
pub enum ChatTarget {
    Private {
        user_id: String,
    },
    Group {
        group_id: String,
        user_id: Option<String>,
    },
    Channel {
        guild_id: String,
        channel_id: String,
    },
    #[serde(untagged)]
    Other {
        detail_type: String,
    },
}
