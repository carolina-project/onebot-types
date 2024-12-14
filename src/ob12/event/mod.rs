use std::time::Duration;

use ob_types_base::{ext::IntoValue, OBAction};
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
use serde_value::{DeserializerError, SerializerError};

use crate::ValueMap;

#[data]
pub struct EventDetail {
    pub r#type: String,
    pub detail_type: String,
    #[serde(flatten)]
    pub detail: ValueMap,
}

#[data]
pub struct Event {
    pub id: String,
    #[serde(with = "ob_types_base::tool::duration_f64")]
    pub time: Duration,
    #[serde(flatten)]
    pub event: EventDetail,
}

impl TryFrom<EventDetail> for EventType {
    type Error = DeserializerError;

    fn try_from(detail: EventDetail) -> Result<Self, Self::Error> {
        let EventDetail {
            r#type,
            detail_type,
            mut detail,
        } = detail;
        detail.insert("type".into(), r#type.into_value());
        detail.insert("detail_type".into(), detail_type.into_value());
        Deserialize::deserialize(detail.into_deserializer())
    }
}

#[data]
#[serde(rename_all = "lowercase", tag = "type")]
pub enum EventType {
    Meta(MetaEvent),
    Message(MessageEvent),
    Notice(NoticeEvent),
    Request(RequestEvent),
}
