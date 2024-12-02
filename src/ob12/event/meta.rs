use ob_types_macro::data;
use crate::ValueMap;

use super::EventType;
use crate::ob12::{Status, VersionInfo};

#[data]
pub struct MetaEvent {
    pub sub_type: String,
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
            #[data]
            pub struct $kind {
                $(pub $field: $ty,)*
                #[serde(flatten)]
                pub extra: crate::ValueMap,
            }
        )*

        $(
            impl From<$kind> for EventType {
                fn from(value: $kind) -> Self {
                    Self::Meta(MetaEvent {
                        sub_type: Default::default(),
                        kind: MetaKind::$kind(value),
                    })
                }
            }
        )*

        #[data]
        #[serde(tag = "detail_type", rename_all = "snake_case")]
        pub enum MetaKind {
            $(
            $kind($kind),
            )*
            #[serde(untagged)]
            Other {
                detail_type: String,
                #[serde(flatten)]
                data: ValueMap,
            },
        }
    };
}

meta_kinds! {
    Connect {
        version: VersionInfo,
    },
    Heartbeat {
        interval: u64,
    },
    StatusUpdate {
        status: Status,
    },
}
