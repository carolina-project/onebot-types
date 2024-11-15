use ob_types_base::JSONValue;
use ob_types_macro::json;

use crate::ob12::{message::MessageChain, BotSelf, ChatTarget};

#[cfg(feature = "json")]
fn de_extra<'de, D: serde::Deserializer<'de>>(deserializer: D) -> Result<JSONValue, D::Error> {
    use std::collections::BTreeMap;

    use serde::Deserialize;

    use crate::ob12::CHAT_TARGET_FIELDS;

    let mut extra = BTreeMap::deserialize(deserializer)?;
    for ele in CHAT_TARGET_FIELDS {
        extra.remove(*ele);
    }
    Ok(extra.into())
}

#[json]
pub struct MessageEvent {
    #[serde(rename = "self")]
    pub self_: BotSelf,
    pub message_id: String,
    pub sub_type: String,
    pub message: MessageChain,
    pub alt_message: Option<String>,
    #[serde(flatten)]
    pub source: ChatTarget,
    #[serde(flatten, deserialize_with = "de_extra")]
    pub extra: JSONValue,
}
