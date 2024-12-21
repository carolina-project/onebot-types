use std::fmt::Display;

use crate::ob12::{self, BotSelf};

pub mod action;
pub mod event;
pub mod message;

pub type CompatResult<T> = Result<T, CompatError>;

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
    #[error("{0}")]
    Other(String),
}

impl CompatError {
    pub fn other<E: Display>(e: E) -> Self {
        Self::Other(e.to_string())
    }
}
