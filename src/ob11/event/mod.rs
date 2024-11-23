use std::time::Duration;

use ob_types_base::tool::duration_secs;

use ob_types_macro::json;

pub mod message;
pub mod meta;
pub mod notice;
pub mod request;

pub use message::MessageEvent;
pub use meta::MetaEvent;
pub use notice::NoticeEvent;
pub use request::RequestEvent;

#[derive(Copy)]
#[json]
#[serde(rename_all = "snake_case")]
pub enum PostType {
    MetaEvent,
    Message,
    Notice,
    Request,
}

#[json]
pub struct Event {
    #[serde(with = "duration_secs")]
    pub time: Duration,
    pub self_id: i64,
    #[serde(flatten)]
    pub kind: EventKind,
}

#[json]
#[serde(tag = "post_type", rename_all = "snake_case")]
pub enum EventKind {
    Message(MessageEvent),
    #[serde(rename = "meta_event")]
    Meta(MetaEvent),
    Request(RequestEvent),
    Notice(NoticeEvent),
}
