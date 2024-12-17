use ob_types_macro::__data;

pub mod message;
pub mod meta;
pub mod notice;
pub mod request;

pub use message::MessageEvent;
pub use meta::MetaEvent;
pub use notice::NoticeEvent;
pub use request::RequestEvent;
use serde::Deserialize;
use serde_value::{DeserializerError, SerializerError};

use crate::ValueMap;

#[__data]
pub struct EventDetail {
    pub r#type: String,
    pub detail_type: String,
    #[serde(flatten)]
    pub detail: ValueMap,
}

#[__data]
pub struct EventDetailed {
    pub detail_type: String,
    #[serde(flatten)]
    pub detail: ValueMap,
}

#[__data]
pub struct RawEvent {
    pub id: String,
    pub time: f64,
    #[serde(flatten)]
    pub event: EventDetail,
}

#[__data]
#[derive(Copy, Eq)]
#[serde(rename_all = "lowercase")]
pub enum EventType {
    Meta,
    Message,
    Notice,
    Request,
}

/// Standard event types in OneBot 12.
#[__data]
pub struct Event {
    pub r#type: EventType,
    #[serde(flatten)]
    pub detailed: EventDetailed,
}

impl TryFrom<EventDetail> for Event {
    type Error = DeserializerError;

    fn try_from(detail: EventDetail) -> Result<Self, Self::Error> {
        serde_value::to_value(detail)
            .map_err(serde::de::Error::custom)
            .and_then(Deserialize::deserialize)
    }
}

impl TryFrom<Event> for EventDetail {
    type Error = SerializerError;

    fn try_from(event: Event) -> Result<Self, Self::Error> {
        serde_value::to_value(event)
            .and_then(|r| Self::deserialize(r).map_err(serde::ser::Error::custom))
    }
}

macro_rules! impl_from_into {
    ($typ: ty, $e_ty:expr) => {
        impl TryFrom<$typ> for super::EventDetailed {
            type Error = serde_value::SerializerError;

            fn try_from(event: $typ) -> Result<Self, Self::Error> {
                serde_value::to_value(event)
                    .and_then(|r| serde::Deserialize::deserialize(r).map_err(serde::ser::Error::custom))
            }
        }

        impl TryFrom<$typ> for super::Event {
            type Error = serde_value::SerializerError;

            fn try_from(event: $typ) -> Result<Self, Self::Error> {
                Ok(super::Event {
                    r#type: $e_ty,
                    detailed: event.try_into()?,
                })
            }
        }

        impl TryFrom<$typ> for super::EventDetail {
            type Error = serde_value::SerializerError;

            fn try_from(event: $typ) -> Result<Self, Self::Error> {
                serde_value::to_value(event)
                    .and_then(|r|
                        serde::Deserialize::deserialize(r)
                        .map_err(serde::ser::Error::custom)
                    )
            }
        }

        impl TryFrom<super::EventDetailed> for $typ {
            type Error = serde_value::DeserializerError;

            fn try_from(event: super::EventDetailed) -> Result<Self, Self::Error> {
                serde_value::to_value(event)
                    .map_err(serde::de::Error::custom)
                    .and_then(serde::Deserialize::deserialize)
            }
        }

        impl TryFrom<super::Event> for $typ {
            type Error = serde_value::DeserializerError;

            fn try_from(event: super::Event) -> Result<Self, Self::Error> {
                if event.r#type == $e_ty {
                    event.detailed.try_into()
                } else {
                    Err(serde::de::Error::custom(format!(
                        "expected event type `{:?}`, found {:?}",
                        $e_ty,
                        event.r#type
                    )))
                }
            }
        }

        impl TryFrom<super::EventDetail> for $typ {
            type Error = serde_value::DeserializerError;

            fn try_from(event: super::EventDetail) -> Result<Self, Self::Error> {
                serde_value::to_value(event)
                    .map_err(serde::de::Error::custom)
                    .and_then(serde::Deserialize::deserialize)
            }
        }
    };
}

pub(self) use impl_from_into;
