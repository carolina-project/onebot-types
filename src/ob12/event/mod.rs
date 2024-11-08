use std::time::Duration;

use message::MessageEvent;
use meta::MetaEvent;
use ob_types_macro::json;
use ob_types_base::tool::duration_f64;

pub mod meta;
pub mod message;
pub mod notice;

#[json(serde(rename_all = "lowercase", tag = "type"))]
pub enum EventType {
    Meta(MetaEvent),
    Message(MessageEvent),
    Notice,
    Request,
}

#[json(resp)]
pub struct Event {
    pub id: String,
    #[serde(with = "duration_f64")]
    pub time: Duration,
    #[serde(flatten)]
    pub event: EventType,
}
