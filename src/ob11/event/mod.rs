use meta::MetaEvent;
use notice::NoticeEvent;
use ob_types_base::cross::Data;
use ob_types_macro::native_data;
use request::RequestEvent;

use self::message::MessageEvent;

pub mod message;
pub mod meta;
pub mod notice;
pub mod request;

#[derive(Clone, Copy, Debug)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
pub enum OB11PostType {
    MetaEvent,
    Message,
    Notice,
    Request,
}

#[derive(Clone, Debug)]
#[native_data]
pub struct OB11EventRaw {
    pub time: u64,
    pub self_id: u64,
    pub post_type: OB11PostType,
    #[cfg_attr(not(target_arch = "wasm32"), serde(flatten))]
    pub extra: Data,
}

pub struct Event {
    pub time: i64,
    pub self_id: i64,
    pub kind: EventKind,
}

pub enum EventKind {
    Message(MessageEvent),
    Meta(MetaEvent),
    Request(RequestEvent),
    Notice(NoticeEvent),
}
