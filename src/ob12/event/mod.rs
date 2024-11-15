use std::time::Duration;

use message::MessageEvent;
use meta::MetaEvent;
use notice::NoticeEvent;
use ob_types_macro::json;
use request::RequestEvent;

pub mod message;
pub mod meta;
pub mod notice;
pub mod request;

#[json(serde(rename_all = "lowercase", tag = "type"))]
pub enum EventType {
    Meta(MetaEvent),
    Message(MessageEvent),
    Notice(NoticeEvent),
    Request(RequestEvent),
}

#[json(resp)]
pub struct Event {
    pub id: String,
    #[serde(with = "ob_types_base::tool::duration_f64")]
    pub time: Duration,
    #[serde(flatten)]
    pub event: EventType,
}
