use ob_types_macro::data;
use serde_value::Value;

use crate::ob12::BotSelf;

#[data]
pub struct RequestEvent {
    #[serde(rename = "self")]
    pub self_: BotSelf,
    pub sub_type: String,
    pub kind: RequestKind,
}

#[data]
pub enum RequestKind {
    #[serde(untagged)]
    Other {
        detail_type: String,
        #[serde(flatten)]
        data: Value,
    },
}
