use ob_types_macro::{data, onebot_action};

use crate::ob11::message::MessageChain;
use crate::ob11::Sex;

use super::bot::MessageResp;
use super::EmptyResp;

#[onebot_action(MessageResp)]
pub struct SendPrivateMsg {
    pub user_id: i64,
    pub message: MessageChain,
}

#[onebot_action(EmptyResp)]
pub struct SendLike {
    pub user_id: i64,
    pub times: Option<u16>,
}

const fn true_value() -> bool {
    true
}

#[onebot_action(EmptyResp)]
pub struct SetFriendAddRequest {
    pub flag: String,
    #[serde(default = "true_value")]
    pub approve: bool,
    #[serde(default)]
    pub remark: Option<String>,
}

#[onebot_action(StrangerInfoResp)]
pub struct GetStrangerInfo {
    pub user_id: i64,
    #[serde(default)]
    pub no_cache: bool,
}

#[data]
pub struct StrangerInfoResp {
    pub user_id: i64,
    pub nickname: String,
    pub sex: Sex,
    pub age: u32,
}

#[onebot_action(Vec<FriendInfo>)]
pub struct GetFriendList;

#[data]
pub struct FriendInfo {
    pub user_id: i64,
    pub nickname: String,
    pub remark: String,
}
