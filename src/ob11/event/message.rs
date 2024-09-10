use ob_types_macro::native_data;

use crate::ob11::message::MessageSeg;

pub struct MessageEvent {
    pub message: Message,
    pub kind: MessageKind,
}
pub struct Message {
    pub message_id: u32,
    pub user_id: u64,
    pub message: Vec<MessageSeg>,
    pub raw_message: String,
    pub font: u32,
}

#[native_data(serde(rename_all = "lowercase"))]
pub enum Sex {
    Male,
    Female,
    Unknown,
}

pub enum MessageKind {
    Private {
        sub_type: PrivateSubType,
        sender: PrivateSender,
    },
    Group {
        sub_type: GroupSubType,
        group_id: u64,
        sender: GroupSender,
        anonymous: Option<AnonymousSender>,
    },
}

pub enum PrivateSubType {
    Friend,
    Group,
    Other,
}

#[native_data]
pub struct PrivateSender {
    pub user_id: u64,
    pub nickname: String,
    pub sex: Sex,
    pub age: u32,
}

pub enum GroupSubType {
    Normal,
    Anonymous,
    Notice,
}

#[native_data(serde(rename_all = "lowercase"))]
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
#[native_data]
pub struct AnonymousSender {
    pub id: u64,
    pub name: String,
    pub flag: String,
}
