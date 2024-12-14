use ob_types_macro::data;

use crate::{
    ob11::{message::MessageChain, Sex},
    ValueMap,
};

#[data]
pub struct MessageEvent {
    #[serde(flatten)]
    pub message: Message,
    #[serde(flatten)]
    pub kind: MessageKind,
}

#[derive(Clone, Debug)]
pub struct MsgEventChain(pub MessageChain);

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
            #[derive(serde::Deserialize)]
            struct Helper {
                message: MessageChain,
            }
            let helper = Helper::deserialize(deserializer)?;
            Ok(Self(helper.message))
        }
    }
}

#[data]
pub struct Message {
    pub message_id: i32,
    pub user_id: i64,
    #[serde(flatten)]
    pub message_segs: MsgEventChain,
    pub raw_message: String,
    pub font: i32,
}
#[data]
#[allow(unused)]
pub struct PrivateMessageKind {
    pub sub_type: PrivateSubType,
    pub sender: PrivateSender,
}
#[data]
#[allow(unused)]
pub struct GroupMessageKind {
    pub sub_type: GroupSubType,
    pub group_id: i64,
    pub sender: GroupSender,
    pub anonymous: Option<AnonymousSender>,
}
#[data]
pub struct MessageKindRaw {
    pub message_type: String,
    pub detail: ValueMap,
}

#[data]
#[serde(tag = "message_type", rename_all = "snake_case")]
pub enum MessageKind {
    Private(PrivateMessageKind),
    Group(GroupMessageKind),
    #[serde(untagged)]
    Other(MessageKindRaw),
}

#[data]
#[serde(rename_all = "snake_case")]
pub enum PrivateSubType {
    Friend,
    Group,
    Other,
}

#[data]
pub struct PrivateSender {
    pub user_id: Option<i64>,
    pub nickname: Option<String>,
    pub sex: Option<Sex>,
    pub age: Option<u32>,
}

#[data]
#[serde(rename_all = "snake_case")]
pub enum GroupSubType {
    Normal,
    Anonymous,
    Notice,
}

#[data]
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
#[data]
pub struct AnonymousSender {
    pub id: i64,
    pub name: String,
    pub flag: String,
}
