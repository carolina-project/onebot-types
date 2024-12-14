use ob_types_macro::__data;

use crate::ob12::BotSelf;

#[__data]
pub struct RequestEvent {
    #[serde(rename = "self")]
    pub self_: BotSelf,
    #[serde(flatten)]
    pub kind: RequestKind,
}

#[__data]
pub enum RequestKind {
    Other(super::EventDetailed),
}
