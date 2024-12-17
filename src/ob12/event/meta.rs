use super::impl_from_into;
use super::Event;
use super::EventType;
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
            impl TryFrom<$kind> for Event {
                type Error = serde_value::SerializerError;

                fn try_from(value: $kind) -> Result<Self, Self::Error> {
                    Ok(Self {
                        r#type: super::EventType::Meta,
                        detailed: MetaEvent::$kind(value).try_into()?,
                    })
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

impl_from_into!(MetaEvent, EventType::Meta);
