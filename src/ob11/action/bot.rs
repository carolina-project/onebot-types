use ob_types_macro::{native_data, onebot_action};
use crate::ob11::{event::message::{GroupSender, PrivateSender}, message::MessageSeg};

#[native_data]
pub enum ChatTarget {
    Private(u64),
    Group(u64),
}

#[onebot_action("send_msg", MessageResp)]
pub struct SendMessage {
    pub target: ChatTarget,
    pub message: Vec<MessageSeg>,
}

#[native_data]
pub struct MessageResp {
    pub message_id: u32,
}

pub type EmptyResp = ();
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

#[native_data]
pub struct LoginInfo {
    pub user_id: u64,
    pub nickname: String,
}

#[onebot_action("get_login_info", LoginInfo)]
pub struct GetLoginInfo;
