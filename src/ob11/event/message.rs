use ob_types_base::OBResult;
use ob_types_macro::json;

use crate::ob11::{message::MessageSeg, Sex};

#[json]
pub struct MessageEvent {
    #[cfg_attr(feature = "json", serde(flatten))]
    pub message: Message,
    #[cfg_attr(feature = "json", serde(flatten))]
    pub kind: MessageKind,
}

#[derive(Debug, Clone)]
pub enum MessageSegs {
    Array(Vec<MessageSeg>),
    String(String),
}

mod serde_impl_segs {
    use std::borrow::Cow;

    use super::MessageSegs;
    use serde::{de, ser, Serialize};
    impl ser::Serialize for MessageSegs {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ser::Serializer,
        {
            #[derive(serde::Serialize)]
            struct Helper<'a, T: Serialize> {
                message_format: &'a str,
                message: &'a T,
            }

            match self {
                MessageSegs::Array(segs) => Helper {
                    message_format: "array",
                    message: segs,
                }
                .serialize(serializer),
                MessageSegs::String(s) => Helper {
                    message_format: "string",
                    message: s,
                }
                .serialize(serializer),
            }
        }
    }
    impl<'de> de::Deserialize<'de> for MessageSegs {
        fn deserialize<D>(deserializer: D) -> Result<MessageSegs, D::Error>
        where
            D: de::Deserializer<'de>,
        {
            use serde::de::Error;
            #[derive(serde::Deserialize)]
            struct Helper<'a> {
                message_format: Option<Cow<'a, str>>,
                message: serde_json::Value,
            }
            let helper = Helper::deserialize(deserializer)?;
            let match_closure = |typ: &str| match typ {
                "array" => serde_json::from_value(helper.message)
                    .map(Self::Array)
                    .map_err(Error::custom),
                "string" => serde_json::from_value(helper.message)
                    .map(Self::String)
                    .map_err(Error::custom),
                _ => Err(Error::custom("unknown message_format")),
            };
            if let Some(r) = helper.message_format {
                match_closure(r.as_ref())
            } else {
                match_closure("string")
            }
        }
    }
}

impl MessageSegs {
    pub fn into_segs(self) -> OBResult<Vec<MessageSeg>> {
        match self {
            MessageSegs::Array(segs) => Ok(segs),
            MessageSegs::String(_) => unimplemented!("cq code parse"),
        }
    }
}

#[json]
pub struct Message {
    pub message_id: u32,
    pub user_id: u64,
    #[cfg_attr(feature = "json", serde(flatten))]
    pub message_segs: MessageSegs,
    pub raw_message: String,
    pub font: u32,
}
#[json]
#[allow(unused)]
pub struct PrivateMessageKind {
    sub_type: PrivateSubType,
    sender: PrivateSender,
}
#[json]
#[allow(unused)]
pub struct GroupMessageKind {
    sub_type: GroupSubType,
    group_id: u64,
    sender: GroupSender,
    anonymous: Option<AnonymousSender>,
}
#[json(serde(tag = "message_type", rename_all = "snake_case"))]
pub enum MessageKind {
    Private(PrivateMessageKind),
    Group(GroupMessageKind),
}

#[json(serde(rename_all = "snake_case"))]
pub enum PrivateSubType {
    Friend,
    Group,
    Other,
}

#[json]
pub struct PrivateSender {
    pub user_id: Option<u64>,
    pub nickname: Option<String>,
    pub sex: Option<Sex>,
    pub age: Option<u32>,
}

#[json(serde(rename_all = "snake_case"))]
pub enum GroupSubType {
    Normal,
    Anonymous,
    Notice,
}

#[json(serde(rename_all = "snake_case"))]
pub struct GroupSender {
    pub user_id: Option<u64>,
    pub nickname: Option<String>,
    pub card: Option<String>,
    pub sex: Option<Sex>,
    pub age: Option<u32>,
    pub area: Option<String>,
    pub level: Option<String>,
    pub role: Option<String>,
    pub title: Option<String>,
}
#[json]
pub struct AnonymousSender {
    pub id: u64,
    pub name: String,
    pub flag: String,
}
