use crate::ob11::{
    event::message::{GroupSender, PrivateSender},
    message::MessageSeg,
};
use ob_types_base::{cross::Data, OBAction};
use ob_types_macro::{native_data, onebot_action};

use super::EmptyResp;

#[native_data]
pub enum ChatTarget {
    Private(u64),
    Group(u64),
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
#[cfg(not(target_arch = "wasm32"))]
impl<'de> serde::Deserialize<'de> for SendMessage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde_json::Value;
        let mut value = Value::deserialize(deserializer)?;

        let target = {
            let getter = |index| {
                value
                    .get(index)
                    .and_then(Value::as_u64)
            };
            if let Some(id) = getter("group_id") {
                ChatTarget::Group(id)
            } else if let Some(id) = getter("user_id") {
                ChatTarget::Private(id)
            } else {
                return Err(serde::de::Error::missing_field("group_id/user_id"));
            }
        };
        let message: Vec<MessageSeg> = value
            .get_mut("message")
            .and_then(Value::as_array_mut)
            .ok_or_else(|| serde::de::Error::missing_field("message"))?
            .drain(..)
            .map(serde_json::from_value::<MessageSeg>)
            .collect::<serde_json::Result<_>>()
            .map_err(|e| serde::de::Error::custom(e))?;
        Ok(Self { target, message })
    }
}

#[native_data]
pub struct MessageResp {
    pub message_id: u32,
}

#[onebot_action("delete_msg", EmptyResp)]
pub struct DeleteMessage {
    pub message_id: u32,
}

#[onebot_action("get_msg", GetMessageResp)]
pub struct GetMessage {
    pub message_id: u32,
}

#[native_data]
pub enum MessageSender {
    Private(PrivateSender),
    Group(GroupSender),
}

#[native_data]
pub struct GetMessageResp {
    pub time: u32,
    pub message_id: u32,
    pub real_id: u32,
    pub sender: MessageSender,
    pub message: Vec<MessageSeg>,
}

#[onebot_action("get_forward_msg", GetForwardMsgResp)]
pub struct GetForwardMsg {
    pub id: String,
}

#[native_data]
pub struct GetForwardMsgResp {
    pub message: Vec<MessageSeg>,
}

#[onebot_action("get_login_info", LoginInfo)]
pub struct GetLoginInfo;

#[native_data]
pub struct LoginInfo {
    pub user_id: u64,
    pub nickname: String,
}

#[onebot_action("get_cookies", String)]
pub struct GetCookies {
    pub domain: Option<String>,
}

#[onebot_action("get_csrf_token", u32)]
pub struct GetCSRFToken;

#[onebot_action("get_credentials", CredentialsResp)]
pub struct GetCredentials {
    pub domain: Option<String>,
}

#[native_data]
pub struct CredentialsResp {
    pub cookies: String,
    pub csrf_token: u32,
}

#[onebot_action("get_record", String)]
pub struct GetRecord {
    pub file: String,
    pub out_format: String,
}

#[onebot_action("get_image", String)]
pub struct GetImage {
    pub file: String,
}

#[onebot_action("can_send_image", bool)]
pub struct CanSendImage;

#[onebot_action("can_send_record", bool)]
pub struct CanSendRecord;

#[onebot_action("get_status", Status)]
pub struct GetStatus;

#[native_data]
pub struct Status {
    pub online: bool,
    pub good: bool,
    #[cfg_attr(not(target_arch = "wasm32"), serde(flatten))]
    pub extra: Data,
}

#[onebot_action("get_version_info", VersionInfo)]
pub struct GetVersion;

#[native_data]
pub struct VersionInfo {
    pub app_name: String,
    pub app_version: String,
    pub protocol_version: String,
    #[cfg_attr(not(target_arch = "wasm32"), serde(flatten))]
    pub extra: Data,
}

#[onebot_action("set_restart", EmptyResp)]
pub struct SetRestart {
    pub delay: u32,
}

#[onebot_action("clean_cache", EmptyResp)]
pub struct CleanCache;
