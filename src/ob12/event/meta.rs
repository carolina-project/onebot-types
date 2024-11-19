use ob_types_macro::json;

use crate::ob12::{Status, VersionInfo};

#[json]
pub struct MetaEvent {
    pub sub_type: Option<String>,
    #[serde(flatten)]
    pub kind: MetaKind,
}

macro_rules! meta_kinds {
    {$(
        $kind:ident {
            $($field:ident: $ty:ty),* $(,)?
        },
    )*} => {
        $(
            #[json]
            pub struct $kind {
                $(pub $field: $ty,)*
                #[serde(flatten)]
                pub extra: serde_value::Value,
            }
        )*

        #[json]
        #[serde(tag = "detail_type", rename_all = "snake_case")]
        pub enum MetaKind {
            $(
            $kind($kind),
            )*
            #[serde(untagged)]
            Other {
                detail_type: String,
                #[serde(flatten)]
                data: serde_value::Value,
            },
        }
    };
}

meta_kinds! {
    Connect {
        version: VersionInfo,
    },
    Heartbeat {
        interval: i64,
    },
    StatusUpdate {
        status: Status,
    },
}
