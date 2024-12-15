use super::EventKind;
use crate::ob12::{Status, VersionInfo};
use ob_types_macro::OBEvent;
use ob_types_macro::__data;

macro_rules! meta_kinds {
    {$(
        $kind:ident {
            $($field:ident: $ty:ty),* $(,)?
        },
    )*} => {
        $(
            #[__data]
            #[derive(OBEvent)]
            #[event(__crate_path = crate, type = "meta")]
            pub struct $kind {
                $(pub $field: $ty,)*
                #[serde(flatten)]
                pub extra: crate::ValueMap,
            }
        )*

        $(
            impl From<$kind> for EventKind {
                fn from(value: $kind) -> Self {
                    Self::Meta(MetaEvent::$kind(value))
                }
            }
        )*

        #[__data]
        #[serde(tag = "detail_type", rename_all = "snake_case")]
        pub enum MetaEvent {
            $(
            $kind($kind),
            )*
            #[serde(untagged)]
            Other(super::EventDetailed),
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

impl From<MetaEvent> for EventKind {
    fn from(value: MetaEvent) -> Self {
        Self::Meta(value)
    }
}
