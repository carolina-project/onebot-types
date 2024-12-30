pub mod action;
pub mod event;
pub mod message;

pub use event::{MessageEvent, NoticeEvent, RawEvent, RequestEvent};
pub use message::MessageSeg;
use ob_types_macro::__data;

use crate::ValueMap;

#[__data]
#[serde(rename_all = "lowercase")]
pub enum Sex {
    Male,
    Female,
    Unknown,
}

#[__data]
pub struct Status {
    pub online: bool,
    pub good: bool,
    #[serde(flatten)]
    pub extra: ValueMap,
}
