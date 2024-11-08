use ob_types_macro::json;

pub mod action;
pub mod event;
pub mod message;

#[macro_export]
macro_rules! scalable_data {
    {
        $(
            $(#[$head_meta:meta])*
            $typ:ident $(= {
                $(
                    $(#[$meta:meta])*
                    $field:ident : $f_ty:ty
                ),* $(,)?
            })?
        ),* $(,)?
    } => {
        $(
            $crate::scalable_data! {
                $(#[$head_meta])*
                $typ = {
                    $(
                    $(#[$meta])*
                    $field: $f_ty
                    ),*
                }
            }
        )*
    };
    {
        $(#[$head_meta:meta])+
        $typ:ident $(= {
            $(
                $(#[$meta:meta])*
                $field:ident : $f_ty:ty
            ),* $(,)?
        })?
    } => {
        $(#[$head_meta])+
        pub struct $typ {
            $(
                $(
                    $(#[$meta])*
                    pub $field: $f_ty,
                )*
            )?
            #[serde(flatten)]
            pub extra: ob_types_base::JSONValue,
        }
    };
    {
        $typ:ident $(= {
            $(
                $(#[$meta:meta])*
                $field:ident : $f_ty:ty
            ),* $(,)?
        })?
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
            pub extra: ob_types_base::JSONValue,
        }
    }
}

#[json]
pub struct BotSelf {
    pub platform: String,
    pub user_id: String,
}

#[json]
pub struct VersionInfo {
    pub r#impl: String,
    pub version: String,
    pub onebot_version: String,
}

#[json]
pub struct BotState {
    #[serde(rename = "self")]
    pub self_: BotSelf,
    pub online: bool,
}

#[json]
pub struct Status {
    pub good: bool,
    pub bots: Vec<BotState>,
}

#[json(serde(tag = "detail_type", rename_all = "snake_case"))]
pub enum ChatTarget {
    User {
        user_id: String
    },
    Group {
        group_id: String
    },
    Channel {
        channel_id: String
    },
    #[serde(untagged)]
    Other {
        detail_type: String
    }
}
