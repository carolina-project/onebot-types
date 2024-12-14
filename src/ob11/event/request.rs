use ob_types_macro::__data;
use serde::{
    de::{Error as DeErr, IntoDeserializer},
    ser::Error as SerErr,
    Deserialize,
};
use serde_value::{DeserializerError, SerializerError};

use crate::ValueMap;

#[__data]
pub struct RequestDetail {
    pub request_type: String,
    #[serde(flatten)]
    pub detail: ValueMap,
}

impl TryFrom<RequestDetail> for RequestEvent {
    type Error = DeserializerError;

    fn try_from(detail: RequestDetail) -> Result<Self, Self::Error> {
        let RequestDetail {
            request_type,
            detail,
        } = detail;

        Ok(match request_type.as_str() {
            "friend" => RequestEvent::Friend(Deserialize::deserialize(detail.into_deserializer())?),
            "group" => RequestEvent::Group(Deserialize::deserialize(detail.into_deserializer())?),
            _ => return Err(DeserializerError::custom("Unknown request type")),
        })
    }
}

impl TryInto<RequestDetail> for RequestEvent {
    type Error = SerializerError;

    fn try_into(self) -> Result<RequestDetail, Self::Error> {
        RequestDetail::deserialize(serde_value::to_value(self)?).map_err(SerErr::custom)
    }
}

#[__data]
#[serde(tag = "request_type")]
pub enum RequestEvent {
    Friend(RequestArgs),
    Group(GroupRequest),
}

#[__data]
pub struct RequestArgs {
    pub user_id: i64,
    pub comment: String,
    pub flag: String,
}

#[__data]
#[serde(rename_all = "lowercase")]
pub enum AddGroupType {
    Add,
    Invite,
}
#[__data]
pub struct GroupRequest {
    pub sub_type: AddGroupType,
    pub group_id: i64,
    #[serde(flatten)]
    pub args: RequestArgs,
}
