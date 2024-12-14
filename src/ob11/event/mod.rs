use std::{fmt::Debug, time::Duration};

use ob_types_base::tool::duration_secs;

use ob_types_macro::data;

pub mod message;
pub mod meta;
pub mod notice;
pub mod request;

pub use message::MessageEvent;
pub use meta::MetaEvent;
pub use notice::NoticeEvent;
pub use request::RequestEvent;
use serde::{de::IntoDeserializer, Deserialize};
use serde_value::DeserializerError;

use crate::ValueMap;

#[derive(Copy)]
#[data]
#[serde(rename_all = "snake_case")]
pub enum PostType {
    MetaEvent,
    Message,
    Notice,
    Request,
}

#[data]
pub struct Event
{
    #[serde(with = "duration_secs")]
    pub time: Duration,
    pub self_id: i64,
    #[serde(flatten)]
    pub detail: EventDetail,
}

#[data]
pub struct EventDetail {
    pub post_type: String,
    #[serde(flatten)]
    pub detail: ValueMap,
}

#[data]
#[serde(tag = "post_type", rename_all = "snake_case")]
pub enum EventKind {
    Message(MessageEvent),
    #[serde(rename = "meta_event")]
    Meta(MetaEvent),
    Request(RequestEvent),
    Notice(NoticeEvent),
}

impl TryFrom<EventDetail> for EventKind {
    type Error = DeserializerError;

    fn try_from(detail: EventDetail) -> Result<Self, Self::Error> {
        let EventDetail { post_type, detail } = detail;
        match post_type.as_str() {
            "message" => Ok(Self::Message(MessageEvent::deserialize(
                detail.into_deserializer(),
            )?)),
            "meta_event" => Ok(Self::Meta(MetaEvent::deserialize(
                detail.into_deserializer(),
            )?)),
            "request" => Ok(Self::Request(RequestEvent::deserialize(
                detail.into_deserializer(),
            )?)),
            "notice" => Ok(Self::Notice(NoticeEvent::deserialize(
                detail.into_deserializer(),
            )?)),
            _ => Err(DeserializerError::Custom("unknown post_type".into())),
        }
    }
}
