use meta::MetaEvent;
use notice::NoticeEvent;
use ob_types_base::JSONValue;
use ob_types_macro::json;
use request::RequestEvent;

use self::message::MessageEvent;

pub mod message;
pub mod meta;
pub mod notice;
pub mod request;

#[derive(Clone, Copy, Debug)]
#[json(serde(rename_all = "snake_case"))]
pub enum OB11PostType {
    MetaEvent,
    Message,
    Notice,
    Request,
}

#[derive(Clone, Debug)]
#[json]
pub struct OB11EventRaw {
    pub time: u64,
    pub self_id: u64,
    pub post_type: OB11PostType,
    #[cfg_attr(feature = "json", serde(flatten))]
    pub extra: JSONValue,
}

#[json]
pub struct Event {
    pub time: i64,
    pub self_id: i64,
    #[cfg_attr(feature = "json", serde(flatten))]
    pub kind: EventKind,
}

#[json(serde(untagged))]
pub enum EventKind {
    Message(MessageEvent),
    Meta(MetaEvent),
    Request(RequestEvent),
    Notice(NoticeEvent),
}
