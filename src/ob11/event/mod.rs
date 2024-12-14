use std::{fmt::Debug, time::Duration};

use crate::base::tool::duration_secs;

pub mod message;
pub mod meta;
pub mod notice;
pub mod request;

pub use message::{MessageDetail, MessageEvent};
pub use meta::{MetaDetail, MetaEvent};
pub use notice::{NoticeDetail, NoticeEvent};
use ob_types_macro::__data;
pub use request::{RequestDetail, RequestEvent};
use serde::{de::IntoDeserializer, Deserialize};
use serde_value::DeserializerError;

use crate::ValueMap;

#[__data]
pub struct RawEvent {
    #[serde(with = "duration_secs")]
    pub time: Duration,
    pub self_id: i64,
    #[serde(flatten)]
    pub detail: EventDetail,
}

#[__data]
pub struct EventDetail {
    pub post_type: String,
    #[serde(flatten)]
    pub detail: ValueMap,
}

#[__data]
#[serde(tag = "post_type", rename_all = "snake_case")]
pub enum EventKind {
    Message(MessageDetail),
    #[serde(rename = "meta_event")]
    Meta(MetaDetail),
    Request(RequestDetail),
    Notice(NoticeDetail),
}

impl TryFrom<EventDetail> for EventKind {
    type Error = DeserializerError;

    fn try_from(detail: EventDetail) -> Result<Self, Self::Error> {
        let EventDetail { post_type, detail } = detail;
        match post_type.as_str() {
            "message" => Ok(Self::Message(Deserialize::deserialize(
                detail.into_deserializer(),
            )?)),
            "meta_event" => Ok(Self::Meta(Deserialize::deserialize(
                detail.into_deserializer(),
            )?)),
            "request" => Ok(Self::Request(Deserialize::deserialize(
                detail.into_deserializer(),
            )?)),
            "notice" => Ok(Self::Notice(Deserialize::deserialize(
                detail.into_deserializer(),
            )?)),
            _ => Err(DeserializerError::Custom("unknown post_type".into())),
        }
    }
}
