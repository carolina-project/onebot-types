use ob_types_macro::{json, onebot_action};

use crate::ob11::{MessageSeg, Sex};

use super::{bot::MessageResp, EmptyResp};

#[onebot_action(MessageResp)]
pub struct SendPrivateMessage {
    pub user_id: i64,
    pub message: Vec<MessageSeg>,
}

#[onebot_action(EmptyResp)]
pub struct SendLike {
    pub user_id: i64,
    pub times: Option<u16>,
}

#[onebot_action(EmptyResp)]
pub struct SetFriendAddRequest {
    pub flag: String,
    pub approve: Option<bool>,
    pub remark: Option<String>,
}

#[onebot_action(StrangerInfoResp)]
pub struct GetStrangerInfo {
    pub user_id: i64,
    pub no_cache: Option<bool>,
}

#[json(resp)]
pub struct StrangerInfoResp {
    pub user_id: i64,
    pub nickname: String,
    pub sex: Sex,
    pub age: u32,
}

#[onebot_action( Vec<FriendInfo>)]
pub struct GetFriendList;

#[json(resp)]
pub struct FriendInfo {
    pub user_id: i64,
    pub nickname: String,
    pub remark: String,
}
