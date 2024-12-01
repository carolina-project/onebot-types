use crate::ob11::{
    event::message::{GroupSender, PrivateSender},
    message::{MessageChain, MessageSeg},
};
#[allow(unused)]
use ob_types_base::OBRespData;
use ob_types_macro::{data, onebot_action};
use serde_value::Value;

use super::EmptyResp;

#[data]
pub enum ChatTarget {
    #[serde(rename = "group_id")]
    Private(i64),
    #[serde(rename = "user_id")]
    Group(i64),
    Unknown,
}

#[onebot_action(MessageResp)]
pub struct SendMsg {
    #[serde(flatten)]
    pub target: ChatTarget,
    pub message: MessageChain,
}

#[data]
pub struct MessageResp {
    pub message_id: i32,
}

#[onebot_action(EmptyResp)]
#[allow(unused)]
pub struct DeleteMsg {
    pub message_id: i32,
}

#[onebot_action(GetMessageResp)]
pub struct GetMsg {
    pub message_id: i32,
}

#[data]
pub enum MessageSender {
    Private(PrivateSender),
    Group(GroupSender),
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
    struct DeHelper<'a> {
        time: u32,
        message_id: i32,
        real_id: i32,
        sender: Value,
        message: Vec<MessageSeg>,
        message_type: &'a str,
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

            let sender: MessageSender = match value.message_type {
                "private" => Deserialize::deserialize(value.sender)
                    .map(MessageSender::Private)
                    .map_err(serde::de::Error::custom)?,
                "group" => Deserialize::deserialize(value.sender)
                    .map(MessageSender::Group)
                    .map_err(serde::de::Error::custom)?,
                t => Err(serde::de::Error::custom(format!(
                    "unkown message type: {}",
                    t
                )))?,
            };
            Ok(Self {
                time: value.time,
                message_id: value.message_id,
                real_id: value.real_id,
                sender,
                message: value.message,
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

#[onebot_action(GetForwardMsgResp)]
pub struct GetForwardMsg {
    pub id: String,
}

#[data]
pub struct GetForwardMsgResp {
    pub message: Vec<MessageSeg>,
}

#[onebot_action(LoginInfo)]
pub struct GetLoginInfo;

#[data]
pub struct LoginInfo {
    pub user_id: i64,
    pub nickname: String,
}

#[onebot_action(Cookies)]
pub struct GetCookies {
    pub domain: Option<String>,
}

#[data]
pub struct Cookies {
    pub cookies: String,
}

#[onebot_action(CSRFToken)]
pub struct GetCsrfToken;

#[data]
pub struct CSRFToken {
    pub token: i32,
}

#[onebot_action(Credentials)]
pub struct GetCredentials {
    pub domain: Option<String>,
}

#[data]
pub struct Credentials {
    pub cookies: String,
    pub csrf_token: i32,
}

#[data]
pub struct FileResp {
    pub file: String,
}

#[onebot_action(FileResp)]
pub struct GetRecord {
    pub file: String,
    pub out_format: String,
}

#[onebot_action(FileResp)]
pub struct GetImage {
    pub file: String,
}

#[data]
pub struct IsAllowd {
    pub yes: bool,
}

#[onebot_action(IsAllowd)]
pub struct CanSendImage;

#[onebot_action(IsAllowd)]
pub struct CanSendRecord;

#[onebot_action(Status)]
pub struct GetStatus;

#[data]
pub struct Status {
    pub online: bool,
    pub good: bool,
    #[serde(flatten)]
    pub extra: Value,
}

#[onebot_action(VersionInfo)]
pub struct GetVersionInfo;

#[data]
pub struct VersionInfo {
    pub app_name: String,
    pub app_version: String,
    pub protocol_version: String,
    #[serde(flatten)]
    pub extra: Value,
}

#[onebot_action(EmptyResp)]
pub struct SetRestart {
    pub delay: i32,
}

#[onebot_action(EmptyResp)]
pub struct CleanCache;
