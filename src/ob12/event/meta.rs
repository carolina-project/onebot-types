use ob_types_macro::json;

use crate::ob12::{Status, VersionInfo};
use ob_types_base::JSONValue;

#[json(resp)]
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
            #[json(resp)]
            pub struct $kind {
                $(pub $field: $ty,)*
                #[serde(flatten)]
                pub extra: JSONValue,
            }
        )*

        #[json(serde(tag = "detail_type", rename_all = "snake_case"))]
        pub enum MetaKind {
            $(
            $kind($kind),
            )*
            #[serde(untagged)]
            Other {
                detail_type: String,
                #[serde(flatten)]
                data: JSONValue,
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
