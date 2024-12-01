use crate::ob12::{self, BotSelf};

pub mod action;
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

#[derive(thiserror::Error, Debug)]
pub enum CompatError {
    #[error(transparent)]
    Serializer(#[from] serde_value::SerializerError),
    #[error(transparent)]
    Deserializer(#[from] serde_value::DeserializerError),
    #[error("unknown compat type: {0}")]
    UnknownCompat(String),
}
