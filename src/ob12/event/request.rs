use crate::{ob12::BotSelf, scalable_struct};

scalable_struct! {
    RequestEvent = {
        #[serde(rename = "self")]
        self_: BotSelf,
        sub_type: Option<String>,
    }
}
