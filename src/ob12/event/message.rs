use crate::{ob12::{BotSelf, ChatTarget}, scalable_struct};

scalable_struct! {
    MessageEvent = {
        #[serde(rename = "self")]
        self_: BotSelf,
        message_id: String,
        sub_type: String,
        alt_message: Option<String>,
        #[serde(flatten)]
        source: ChatTarget,
    },
}
