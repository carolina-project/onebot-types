use crate::ob12::{self, BotSelf};

pub mod event;
pub mod message;

#[inline]
pub(self) fn default_obj() -> serde_value::Value {
    serde_value::Value::Map(Default::default())
}

#[inline]
pub fn compat_self(id: String) -> ob12::BotSelf {
    BotSelf {
        platform: "ob11".into(),
        user_id: id,
    }
}
