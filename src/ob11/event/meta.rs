use ob_types_macro::__data;
use serde::{
    de::{Error as DeErr, IntoDeserializer},
    ser::Error as SerErr,
    Deserialize,
};
use serde_value::{DeserializerError, SerializerError};

use crate::{ob11::Status, ValueMap};

#[__data]
pub struct MetaDetail {
    pub meta_event_type: String,
    #[serde(flatten)]
    pub detail: ValueMap,
}

impl TryFrom<MetaDetail> for MetaEvent {
    type Error = DeserializerError;

    fn try_from(detail: MetaDetail) -> Result<Self, Self::Error> {
        let MetaDetail {
            meta_event_type,
            detail,
        } = detail;

        match meta_event_type.as_str() {
            "lifecycle" => Ok(MetaEvent::LifeCycle(Deserialize::deserialize(
                detail.into_deserializer(),
            )?)),
            "heartbeat" => Ok(MetaEvent::Heartbeat(Deserialize::deserialize(
                detail.into_deserializer(),
            )?)),
            _ => Err(DeserializerError::custom("unknown meta type")),
        }
    }
}

impl TryInto<MetaDetail> for MetaEvent {
    type Error = SerializerError;

    fn try_into(self) -> Result<MetaDetail, Self::Error> {
        MetaDetail::deserialize(serde_value::to_value(self)?).map_err(SerErr::custom)
    }
}

#[__data]
#[serde(tag = "meta_event_type", rename_all = "lowercase")]
pub enum MetaEvent {
    LifeCycle(LifeCycle),
    Heartbeat(Heartbeat),
}

#[__data]
pub struct Heartbeat {
    pub status: Status,
    pub interval: u64,
}

#[__data]
#[serde(tag = "sub_type", rename_all = "lowercase")]
pub enum LifeCycle {
    Enable,
    Disable,
    Connect,
}
