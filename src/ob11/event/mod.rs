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

pub struct Event {
    pub time: i64,
    pub self_id: i64,
    pub kind: EventKind,
}

#[cfg(feature = "json")]
impl<'de> serde::Deserialize<'de> for Event {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {

    }
}

pub enum EventKind {
    Message(MessageEvent),
    Meta(MetaEvent),
    Request(RequestEvent),
    Notice(NoticeEvent),
}
