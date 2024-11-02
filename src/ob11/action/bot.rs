use crate::ob11::{
    event::message::{GroupSender, PrivateSender},
    message::MessageSeg,
};
#[allow(unused)]
use ob_types_base::OBRespData;
use ob_types_base::{json::JSONValue, OBAction};
use ob_types_macro::{json, onebot_action, OBRespData};

use super::EmptyResp;

pub enum ChatTarget {
    Private(i64),
    Group(i64),
}

pub struct SendMessage {
    pub target: ChatTarget,
    pub message: Vec<MessageSeg>,
}
impl OBAction for SendMessage {
    type Resp = MessageResp;

    fn action(&self) -> &str {
        "send_msg"
    }
}

#[cfg(feature = "json")]
mod serde_impl_send {
    use serde::Deserialize;

    use crate::ob11::MessageSeg;

    use super::{ChatTarget, SendMessage};

    impl<'de> serde::Deserialize<'de> for SendMessage {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            #[derive(Deserialize)]
            struct Helper {
                group_id: Option<i64>,
                user_id: Option<i64>,
                message: Vec<MessageSeg>,
            }
            let helper = Helper::deserialize(deserializer)?;

            let target = {
                if let Some(id) = helper.group_id {
                    ChatTarget::Group(id)
                } else if let Some(id) = helper.user_id {
                    ChatTarget::Private(id)
                } else {
                    return Err(serde::de::Error::missing_field("group_id/user_id"));
                }
            };
            Ok(Self {
                target,
                message: helper.message,
            })
        }
    }
    #[cfg(feature = "json")]
    impl serde::Serialize for SendMessage {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            use serde::ser::SerializeStruct;
            let mut result = serializer.serialize_struct("SendMessage", 3)?;
            match self.target {
                ChatTarget::Private(id) => {
                    result.serialize_field("message_type", "private")?;
                    result.serialize_field("user_id", &id)?;
                }
                ChatTarget::Group(id) => {
                    result.serialize_field("message_type", "group")?;
                    result.serialize_field("group_id", &id)?;
                }
            }
            result.serialize_field("message", &self.message)?;
            result.end()
        }
    }
}

#[json]
pub struct MessageResp {
    pub message_id: i32,
}

#[onebot_action("delete_msg", EmptyResp)]
#[cfg_attr(feature = "serde", serde(transparent))]
#[allow(unused)]
pub struct DeleteMessage {
    message_id: i32,
}

#[onebot_action("get_msg", GetMessageResp)]
pub struct GetMessage {
    pub message_id: i32,
}

#[json]
pub enum MessageSender {
    Private(PrivateSender),
    Group(GroupSender),
}

#[derive(OBRespData)]
pub struct GetMessageResp {
    pub time: u32,
    pub message_id: i32,
    pub real_id: i32,
    pub sender: MessageSender,
    pub message: Vec<MessageSeg>,
}

#[cfg(feature = "json")]
mod serde_impl_get {
    use serde::{Deserialize, Serialize};
    use serde_json::Value;

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
                "private" => serde_json::from_value(value.sender)
                    .map_err(serde::de::Error::custom)
                    .map(MessageSender::Private)?,
                "group" => serde_json::from_value(value.sender)
                    .map_err(serde::de::Error::custom)
                    .map(MessageSender::Group)?,
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

#[onebot_action("get_forward_msg", GetForwardMsgResp)]
pub struct GetForwardMsg {
    pub id: String,
}

#[json]
pub struct GetForwardMsgResp {
    pub message: Vec<MessageSeg>,
}

#[onebot_action("get_login_info", LoginInfo)]
pub struct GetLoginInfo;

#[json]
pub struct LoginInfo {
    pub user_id: i64,
    pub nickname: String,
}

#[onebot_action("get_cookies", Cookies)]
pub struct GetCookies {
    pub domain: Option<String>,
}

#[json]
pub struct Cookies {
    pub cookies: String,
}

#[onebot_action("get_csrf_token", i32)]
pub struct GetCSRFToken;

#[json]
pub struct CSRFToken {
    pub token: i32,
}

#[onebot_action("get_credentials", Credentials)]
pub struct GetCredentials {
    pub domain: Option<String>,
}

#[json]
pub struct Credentials {
    pub cookies: String,
    pub csrf_token: i32,
}

#[json]
pub struct FileResp {
    pub file: String,
}

#[onebot_action("get_record", FileResp)]
pub struct GetRecord {
    pub file: String,
    pub out_format: String,
}

#[onebot_action("get_image", FileResp)]
pub struct GetImage {
    pub file: String,
}

#[json]
pub struct IsAllowd {
    pub yes: bool,
}

#[onebot_action("can_send_image", IsAllowd)]
pub struct CanSendImage;

#[onebot_action("can_send_record", IsAllowd)]
pub struct CanSendRecord;

#[onebot_action("get_status", Status)]
pub struct GetStatus;

#[json]
pub struct Status {
    pub online: bool,
    pub good: bool,
    #[cfg_attr(feature = "json", serde(flatten))]
    pub extra: JSONValue,
}

#[onebot_action("get_version_info", VersionInfo)]
pub struct GetVersion;

#[json]
pub struct VersionInfo {
    pub app_name: String,
    pub app_version: String,
    pub protocol_version: String,
    #[cfg_attr(feature = "json", serde(flatten))]
    pub extra: JSONValue,
}

#[onebot_action("set_restart", EmptyResp)]
pub struct SetRestart {
    pub delay: i32,
}

#[onebot_action("clean_cache", EmptyResp)]
pub struct CleanCache;
