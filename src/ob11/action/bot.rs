use std::collections::HashMap;

use crate::ob11::{
    event::message::{GroupSender, PrivateSender},
    message::MessageSeg,
};
use ob_types_base::{cross::Data, OBAction};
use ob_types_macro::{native_attrs, native_cfg, native_data, onebot_action};

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
#[native_cfg]
impl<'de> serde::Deserialize<'de> for SendMessage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde_json::Value;
        let mut value = Value::deserialize(deserializer)?;

        let target = {
            let getter = |index| value.get(index).and_then(Value::as_u64);
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
#[native_cfg]
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

#[native_data]
pub struct MessageResp {
    pub message_id: u32,
}

#[onebot_action("delete_msg", EmptyResp)]
#[native_attrs(serde(transparent))]
pub struct DeleteMessage {
    message_id: u32,
}

#[onebot_action("get_msg", GetMessageResp)]
pub struct GetMessage {
    pub message_id: u32,
}

#[native_data(serde(untagged))]
pub enum MessageSender {
    Private(PrivateSender),
    Group(GroupSender),
}

pub struct GetMessageResp {
    pub time: u32,
    pub message_id: u32,
    pub real_id: u32,
    pub sender: MessageSender,
    pub message: Vec<MessageSeg>,
}
#[native_cfg]
impl<'de> serde::Deserialize<'de> for GetMessageResp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde_json::Value;
        use crate::hashmap_value_get;
        let mut value: HashMap<String, Value> = HashMap::deserialize(deserializer)?;

        let message_type: String = hashmap_value_get::<_, D>(&mut value, "message_type")?;
        let message_id: u32 = hashmap_value_get::<_, D>(&mut value, "message_id")?;
        let real_id = hashmap_value_get::<_, D>(&mut value, "real_id")?;
        let time = hashmap_value_get::<_, D>(&mut value, "time")?;
        let sender: MessageSender = match message_type.as_str() {
            "private" => {
                hashmap_value_get::<_, D>(&mut value, "sender").map(MessageSender::Private)?
            }
            "group" => hashmap_value_get::<_, D>(&mut value, "sender").map(MessageSender::Group)?,
            t => Err(serde::de::Error::custom(format!(
                "unkown message type: {}",
                t
            )))?,
        };
        let message = hashmap_value_get::<_, D>(&mut value, "message")?;

        Ok(Self {
            time,
            message_id,
            real_id,
            sender,
            message,
        })
    }
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

#[onebot_action("get_cookies", Cookies)]
pub struct GetCookies {
    pub domain: Option<String>,
}

#[native_data]
pub struct Cookies {
    pub cookies: String
}

#[onebot_action("get_csrf_token", u32)]
pub struct GetCSRFToken;

#[native_data]
pub struct CSRFToken {
    pub token: u32
}

#[onebot_action("get_credentials", Credentials)]
pub struct GetCredentials {
    pub domain: Option<String>,
}

#[native_data]
pub struct Credentials {
    pub cookies: String,
    pub csrf_token: u32,
}

#[native_data]
pub struct FileResp {
    pub file: String
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


#[native_data]
pub struct IsAllowd {
    pub yes: bool
}

#[onebot_action("can_send_image", IsAllowd)]
pub struct CanSendImage;

#[onebot_action("can_send_record", IsAllowd)]
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
