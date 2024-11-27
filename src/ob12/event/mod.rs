use std::time::Duration;

use ob_types_macro::data;

pub mod message;
pub mod meta;
pub mod notice;
pub mod request;

pub use message::MessageEvent;
pub use meta::MetaEvent;
pub use notice::NoticeEvent;
pub use request::RequestEvent;

#[data]
#[serde(rename_all = "lowercase", tag = "type")]
pub enum EventType {
    Meta(MetaEvent),
    Message(MessageEvent),
    Notice(NoticeEvent),
    Request(RequestEvent),
}

#[data]
pub struct Event {
    pub id: String,
    #[serde(with = "ob_types_base::tool::duration_f64")]
    pub time: Duration,
    #[serde(flatten)]
    pub event: EventType,
}
