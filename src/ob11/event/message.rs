use ob_types_macro::json;

use crate::ob11::{message::MessageChain, Sex};

#[json]
pub struct MessageEvent {
    #[serde(flatten)]
    pub message: Message,
    #[serde(flatten)]
    pub kind: MessageKind,
}

#[derive(Clone, Debug)]
pub struct MsgEventChain(#[allow(unused)] MessageChain);

#[cfg(feature = "json")]
mod serde_impl_segs {
    use std::borrow::Cow;

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

            match &self.0 {
                MessageChain::Array(segs) => Helper {
                    message_format: "array",
                    message: segs,
                }
                .serialize(serializer),
                MessageChain::String(s) => Helper {
                    message_format: "string",
                    message: s,
                }
                .serialize(serializer),
            }
        }
    }
    impl<'de> de::Deserialize<'de> for super::MsgEventChain {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
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
                    .map(MessageChain::Array)
                    .map(Self)
                    .map_err(Error::custom),
                "string" => serde_json::from_value(helper.message)
                    .map(MessageChain::String)
                    .map(Self)
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

#[json]
pub struct Message {
    pub message_id: i32,
    pub user_id: i64,
    #[cfg_attr(feature = "json", serde(flatten))]
    pub message_segs: MsgEventChain,
    pub raw_message: String,
    pub font: i32,
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
    group_id: i64,
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
    pub user_id: Option<i64>,
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
#[json]
pub struct AnonymousSender {
    pub id: i64,
    pub name: String,
    pub flag: String,
}
