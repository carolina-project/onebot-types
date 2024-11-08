use std::time::Duration;

use meta::MetaEvent;
use notice::NoticeEvent;
use ob_types_base::tool::duration_secs;
use ob_types_base::JSONValue;
use ob_types_macro::json;
use request::RequestEvent;

use self::message::MessageEvent;

pub mod message;
pub mod meta;
pub mod notice;
pub mod request;

#[derive(Copy)]
#[json(serde(rename_all = "snake_case"))]
pub enum PostType {
    MetaEvent,
    Message,
    Notice,
    Request,
}

#[json]
pub struct EventRaw {
    #[serde(with = "duration_secs")]
    pub time: Duration,
    pub self_id: i64,
    pub post_type: PostType,
    #[cfg_attr(feature = "json", serde(flatten))]
    pub extra: JSONValue,
}

#[json]
pub struct Event {
    #[serde(with = "duration_secs")]
    pub time: Duration,
    pub self_id: i64,
    #[cfg_attr(feature = "json", serde(flatten))]
    pub kind: EventKind,
}

#[json(serde(tag = "post_type", rename_all = "snake_case"))]
pub enum EventKind {
    Message(MessageEvent),
    #[cfg_attr(feature = "json", serde(rename = "meta_event"))]
    Meta(MetaEvent),
    Request(RequestEvent),
    Notice(NoticeEvent),
}
