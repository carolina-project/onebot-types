pub mod action;
pub mod event;
pub mod message;

pub use message::MessageSeg;

mod macros {
    #[macro_export]
    macro_rules! scalable_struct {
        {
            $(#[resp($resp:ty)])?
            $(#[derive($($derives:path),+)])?
            $typ:ident $(= {
                $(
                    $(#[$meta:meta])*
                    $field:ident : $f_ty:ty
                ),* $(,)?
            })? $(, $($rest:tt)*)?
        } => {
            #[ob_types_macro::__data]
            $(
            #[derive(ob_types_macro::OBAction)]
            #[action(__crate_path = crate, resp = $resp)]
            )?
            $(#[derive($($derives),+)])?
            pub struct $typ {
                $(
                    $(
                        $(#[$meta])*
                        pub $field: $f_ty,
                    )*)?
                #[serde(flatten)]
                pub extra: $crate::ValueMap,
            }

            $crate::scalable_struct! {
                $($($rest)*)?
            }

        };
        {
            #[msg]
            $typ:ident $(= {
                $(
                    $(#[$meta:meta])*
                    $field:ident : $f_ty:ty
                ),* $(,)?
            })? $(, $($rest:tt)*)?
        } => {
            #[ob_types_macro::__data]
            #[derive(ob_types_macro::OBMessage)]
            #[msg(__crate_path = crate)]
            pub struct $typ {
                $(
                    $(
                        $(#[$meta])*
                        pub $field: $f_ty,
                    )*)?
                #[serde(flatten)]
                pub extra: $crate::ValueMap,
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
use ob_types_macro::__data;

#[__data(default)]
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
    GuildInfo = {
        guild_id: String,
        guild_name: String,
    },
    ChannelInfo = {
        channel_id: String,
        channel_name: String,
    },
}

impl Default for VersionInfo {
    fn default() -> Self {
        Self {
            r#impl: "ob11".into(),
            version: env!("CARGO_PKG_VERSION").into(),
            onebot_version: "12".into(),
            extra: Default::default(),
        }
    }
}

#[__data]
#[serde(tag = "detail_type", rename_all = "snake_case")]
pub enum ChatTarget {
    Private {
        user_id: String,
    },
    Group {
        group_id: String,
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

impl Default for BotState {
    fn default() -> Self {
        BotState {
            self_: Default::default(),
            online: false,
            extra: Default::default(),
        }
    }
}
