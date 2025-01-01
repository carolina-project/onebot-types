use std::collections::VecDeque;

use ob_types_macro::__data;
use serde::{
    de::{Error as DeErr, IntoDeserializer},
    ser::Error as SerErr,
    Deserialize,
};
use serde_value::{DeserializerError, SerializerError};

use crate::{
    base::{MessageChain, RawMessageSeg},
    ob11::Sex,
    ValueMap,
};

#[__data]
pub struct MessageDetail {
    pub message_type: String,
    #[serde(flatten)]
    pub detail: ValueMap,
}

impl TryFrom<MessageDetail> for MessageEvent {
    type Error = DeserializerError;

    fn try_from(detail: MessageDetail) -> Result<Self, Self::Error> {
        let MessageDetail {
            message_type,
            detail,
        } = detail;

        match message_type.as_str() {
            "private" => Ok(MessageEvent::Private(Deserialize::deserialize(
                detail.into_deserializer(),
            )?)),
            "group" => Ok(MessageEvent::Group(Deserialize::deserialize(
                detail.into_deserializer(),
            )?)),
            _ => Err(DeserializerError::custom("unknown message type")),
        }
    }
}

impl TryInto<MessageDetail> for MessageEvent {
    type Error = SerializerError;

    fn try_into(self) -> Result<MessageDetail, Self::Error> {
        MessageDetail::deserialize(serde_value::to_value(self)?).map_err(SerErr::custom)
    }
}

#[__data]
#[serde(tag = "message_type")]
pub enum MessageEvent {
    Private(PrivateMessage),
    Group(GroupMessage),
}

#[__data]
pub struct Message {
    pub message_id: i32,
    pub user_id: i64,
    #[serde(flatten)]
    pub message_segs: MsgEventChain,
    pub raw_message: String,
    pub font: i32,
}

#[__data]
pub struct PrivateMessage {
    pub sub_type: PrivateSubType,
    pub sender: PrivateSender,
    #[serde(flatten)]
    pub message: Message,
}

#[__data]
#[serde(rename_all = "snake_case")]
pub enum PrivateSubType {
    Friend,
    Group,
    Other,
}

#[__data]
pub struct PrivateSender {
    pub user_id: Option<i64>,
    pub nickname: Option<String>,
    pub sex: Option<Sex>,
    pub age: Option<u32>,
}

#[__data]
#[serde(rename_all = "snake_case")]
pub enum GroupSubType {
    Normal,
    Anonymous,
    Notice,
}

#[__data]
#[serde(rename_all = "snake_case")]
pub struct GroupSender {
    pub user_id: Option<i64>,
    pub nickname: Option<String>,
    pub card: Option<String>,
    pub sex: Option<Sex>,
    pub age: Option<u32>,
    pub area: Option<String>,
    pub level: Option<String>,
    pub role: Option<String>,
    pub title: Option<String>,
}
#[__data]
pub struct AnonymousSender {
    pub id: i64,
    pub name: String,
    pub flag: String,
}

#[__data]
pub struct GroupMessage {
    pub sub_type: GroupSubType,
    pub group_id: i64,
    pub sender: GroupSender,
    pub anonymous: Option<AnonymousSender>,
    #[serde(flatten)]
    pub message: Message,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MsgEventChain(pub MessageChain);
impl From<MsgEventChain> for MessageChain {
    fn from(value: MsgEventChain) -> Self {
        value.0
    }
}
impl MsgEventChain {
    pub fn into_inner(self) -> VecDeque<RawMessageSeg> {
        self.0.into_inner()
    }
}

mod serde_impl_segs {
    use super::MessageChain;
    use serde::{de, ser, Serialize};
    impl ser::Serialize for super::MsgEventChain {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ser::Serializer,
        {
            #[derive(serde::Serialize)]
            struct Helper<'a, T: Serialize> {
                message_format: &'a str,
                message: &'a T,
            }

            Helper {
                message_format: "array",
                message: self.0.inner(),
            }
            .serialize(serializer)
        }
    }
    impl<'de> de::Deserialize<'de> for super::MsgEventChain {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: de::Deserializer<'de>,
        {
            #[derive(serde::Deserialize)]
            struct Helper {
                message: MessageChain,
            }
            let helper = Helper::deserialize(deserializer)?;
            Ok(Self(helper.message))
        }
    }
}
