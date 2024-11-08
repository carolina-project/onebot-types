use crate::{ob12::ChatTarget, scalable_struct};

scalable_struct! {
    MessageEvent = {
        message_id: String,
        sub_type: String,
        alt_message: Option<String>,
        #[serde(flatten)]
        source: ChatTarget,
    },
}
