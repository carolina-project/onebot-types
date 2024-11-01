use ob_types_macro::json;

use crate::ob11::{message::MessageSeg, Sex};

#[derive(Debug)]
pub struct MessageEvent {
    pub message: Message,
    pub kind: MessageKind,
}
#[json]
pub struct Message {
    pub message_id: u32,
    pub user_id: u64,
    pub message: Vec<MessageSeg>,
    pub raw_message: String,
    pub font: u32,
}
#[json]
pub struct PrivateMessageKind {
    sub_type: PrivateSubType,
    sender: PrivateSender,
}
#[json]
pub struct GroupMessageKind {
    sub_type: GroupSubType,
    group_id: u64,
    sender: GroupSender,
    anonymous: Option<AnonymousSender>,
}
#[json]
pub enum MessageKind {
    Private(PrivateMessageKind),
    Group(GroupMessageKind),
}

#[cfg(feature = "json")]
mod serde_impl {
    use serde::{Deserialize, Serialize};

    use super::*;

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "snake_case")]
    enum MsgType {
        Private,
        Group,
    }

    #[derive(Serialize)]
    struct SerHelper<'a> {
        message_type: MsgType,
        #[serde(flatten)]
        message: &'a Message,
        #[serde(flatten)]
        kind: &'a MessageKind,
    }

    #[derive(Deserialize)]
    struct DeHelper {
        message_type: MsgType,
        #[serde(flatten)]
        message: Message,
        #[serde(flatten)]
        extra: serde_json::Value,
    }

    impl Serialize for MessageEvent {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            let message_type = match self.kind {
                MessageKind::Private(_) => MsgType::Private,
                MessageKind::Group(_) => MsgType::Group,
            };
            SerHelper {
                message_type,
                message: &self.message,
                kind: &self.kind,
            }
            .serialize(serializer)
        }
    }

    impl<'de> Deserialize<'de> for MessageEvent {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let helper = DeHelper::deserialize(deserializer)?;
            match helper.message_type {
                MsgType::Private => {
                    let kind: PrivateMessageKind =
                        serde_json::from_value(helper.extra).map_err(serde::de::Error::custom)?;
                    Ok(Self {
                        message: helper.message,
                        kind: MessageKind::Private(kind),
                    })
                }
                MsgType::Group => {
                    let kind: GroupMessageKind =
                        serde_json::from_value(helper.extra).map_err(serde::de::Error::custom)?;
                    Ok(Self {
                        message: helper.message,
                        kind: MessageKind::Group(kind),
                    })
                }
            }
        }
    }
}

#[json(serde(rename_all = "snake_case"))]
pub enum PrivateSubType {
    Friend,
    Group,
    Other,
}

#[json]
pub struct PrivateSender {
    pub user_id: u64,
    pub nickname: String,
    pub sex: Sex,
    pub age: u32,
}

#[json(serde(rename_all = "snake_case"))]
pub enum GroupSubType {
    Normal,
    Anonymous,
    Notice,
}

#[json(serde(rename_all = "snake_case"))]
pub struct GroupSender {
    pub user_id: u64,
    pub nickname: String,
    pub card: String,
    pub sex: Sex,
    pub age: u32,
    pub area: String,
    pub level: String,
    pub role: String,
    pub title: String,
}
#[json]
pub struct AnonymousSender {
    pub id: u64,
    pub name: String,
    pub flag: String,
}
