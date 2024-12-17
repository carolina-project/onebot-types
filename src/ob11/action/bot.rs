use ob_types_macro::{OBAction, __data};

use crate::{
    base::MessageChain,
    ob11::{
        event::message::{GroupSender, PrivateSender},
        MessageSeg, Status,
    },
    ValueMap,
};

use super::EmptyResp;

#[derive(Debug, serde::Serialize, Clone, PartialEq, Eq)]
#[serde(tag = "message_type", rename_all = "snake_case")]
pub enum ChatTarget {
    Private { user_id: i64 },
    Group { group_id: i64 },
    Unknown,
}

impl<'de> serde::Deserialize<'de> for ChatTarget {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        #[serde(rename_all = "snake_case")]
        enum ChatType {
            Private,
            Group,
        }
        #[derive(serde::Deserialize)]
        struct Helper {
            message_type: Option<ChatType>,
            user_id: Option<i64>,
            group_id: Option<i64>,
        }

        let Helper {
            message_type,
            user_id,
            group_id,
        } = Helper::deserialize(deserializer)?;
        if let Some(chat_type) = message_type {
            match chat_type {
                ChatType::Private => Ok(ChatTarget::Private {
                    user_id: user_id.ok_or_else(|| serde::de::Error::missing_field("user_id"))?,
                }),
                ChatType::Group => Ok(ChatTarget::Group {
                    group_id: group_id
                        .ok_or_else(|| serde::de::Error::missing_field("group_id"))?,
                }),
            }
        } else {
            if let Some(user_id) = user_id {
                Ok(ChatTarget::Private { user_id })
            } else if let Some(group_id) = group_id {
                Ok(ChatTarget::Group { group_id })
            } else {
                Err(serde::de::Error::missing_field("message_type or *_id"))
            }
        }
    }
}

#[__data]
#[derive(OBAction)]
#[action(__crate_path = crate, resp = MessageResp)]
pub struct SendMsg {
    #[serde(flatten)]
    pub target: ChatTarget,
    pub message: MessageChain,
}

#[__data]
pub struct MessageResp {
    pub message_id: i32,
}

#[__data]
#[derive(OBAction)]
#[action(__crate_path = crate, resp = EmptyResp)]
#[allow(unused)]
pub struct DeleteMsg {
    pub message_id: i32,
}

#[__data]
#[derive(OBAction)]
#[action(__crate_path = crate, resp = GetMessageResp)]
pub struct GetMsg {
    pub message_id: i32,
}

#[__data]
pub enum MessageSender {
    Private(PrivateSender),
    Group(GroupSender),
}

impl MessageSender {
    pub fn user_id(&self) -> Option<i64> {
        match self {
            MessageSender::Private(sender) => sender.user_id.clone(),
            MessageSender::Group(sender) => sender.user_id.clone(),
        }
    }
}

pub struct GetMessageResp {
    pub time: u32,
    pub message_id: i32,
    pub real_id: i32,
    pub sender: MessageSender,
    pub message: Vec<MessageSeg>,
}

mod serde_impl_get {
    use serde::{Deserialize, Serialize};
    use serde_value::Value;

    use crate::ob11::{action::bot::MessageSender, MessageSeg};

    #[derive(Deserialize)]
    #[serde(rename_all = "snake_case")]
    pub enum MsgType {
        Private,
        Group,
    }
    #[derive(Deserialize)]
    struct DeHelper {
        time: u32,
        message_id: i32,
        real_id: i32,
        sender: Value,
        message: Vec<MessageSeg>,
        message_type: MsgType,
    }
    #[derive(Serialize)]
    struct SerHelper<'a> {
        time: u32,
        message_id: i32,
        real_id: i32,
        sender: &'a MessageSender,
        message: &'a Vec<MessageSeg>,
        message_type: &'a str,
    }

    impl<'de> Deserialize<'de> for super::GetMessageResp {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let value = DeHelper::deserialize(deserializer)?;

            let DeHelper {
                time,
                message_id,
                real_id,
                sender,
                message,
                message_type,
            } = value;

            let sender: MessageSender = match message_type {
                MsgType::Private => Deserialize::deserialize(sender)
                    .map(MessageSender::Private)
                    .map_err(serde::de::Error::custom)?,
                MsgType::Group => Deserialize::deserialize(sender)
                    .map(MessageSender::Group)
                    .map_err(serde::de::Error::custom)?,
            };
            Ok(Self {
                time,
                message_id,
                real_id,
                sender,
                message,
            })
        }
    }

    impl Serialize for super::GetMessageResp {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            let message_type = match &self.sender {
                MessageSender::Private(_) => "private",
                MessageSender::Group(_) => "group",
            };
            SerHelper {
                time: self.time,
                message_id: self.message_id,
                real_id: self.real_id,
                sender: &self.sender,
                message: &self.message,
                message_type,
            }
            .serialize(serializer)
        }
    }
}

#[__data]
#[derive(OBAction)]
#[action(__crate_path = crate, resp = GetForwardMsgResp)]
pub struct GetForwardMsg {
    pub id: String,
}

#[__data]
pub struct GetForwardMsgResp {
    pub message: Vec<MessageSeg>,
}

#[__data]
#[derive(OBAction)]
#[action(__crate_path = crate, resp = LoginInfo)]
pub struct GetLoginInfo;

#[__data]
pub struct LoginInfo {
    pub user_id: i64,
    pub nickname: String,
}

#[__data]
#[derive(OBAction)]
#[action(__crate_path = crate, resp = Cookies)]
pub struct GetCookies {
    pub domain: Option<String>,
}

#[__data]
pub struct Cookies {
    pub cookies: String,
}

#[__data]
#[derive(OBAction)]
#[action(__crate_path = crate, resp = CSRFToken)]
pub struct GetCsrfToken;

#[__data]
pub struct CSRFToken {
    pub token: i32,
}

#[__data]
#[derive(OBAction)]
#[action(__crate_path = crate, resp = Credentials)]
pub struct GetCredentials {
    pub domain: Option<String>,
}

#[__data]
pub struct Credentials {
    pub cookies: String,
    pub csrf_token: i32,
}

#[__data]
pub struct FileResp {
    pub file: String,
}

#[__data]
#[derive(OBAction)]
#[action(__crate_path = crate, resp = FileResp)]
pub struct GetRecord {
    pub file: String,
    pub out_format: String,
}

#[__data]
#[derive(OBAction)]
#[action(__crate_path = crate, resp = FileResp)]
pub struct GetImage {
    pub file: String,
}

#[__data]
pub struct IsAllowd {
    pub yes: bool,
}

#[__data]
#[derive(OBAction)]
#[action(__crate_path = crate, resp = IsAllowd)]
pub struct CanSendImage;

#[__data]
#[derive(OBAction)]
#[action(__crate_path = crate, resp = IsAllowd)]
pub struct CanSendRecord;

#[__data]
#[derive(OBAction)]
#[action(__crate_path = crate, resp = Status)]
pub struct GetStatus;

#[__data]
#[derive(OBAction)]
#[action(__crate_path = crate, resp = VersionInfo)]
pub struct GetVersionInfo;

#[__data]
pub struct VersionInfo {
    pub app_name: String,
    pub app_version: String,
    pub protocol_version: String,
    #[serde(flatten)]
    pub extra: ValueMap,
}

#[__data]
#[derive(OBAction)]
#[action(__crate_path = crate, resp = EmptyResp)]
pub struct SetRestart {
    pub delay: i32,
}

#[__data]
#[derive(OBAction)]
#[action(__crate_path = crate, resp = EmptyResp)]
pub struct CleanCache;
