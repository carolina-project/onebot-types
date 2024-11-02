use ob_types_macro::{json, onebot_action};

use crate::ob11::Sex;

use super::EmptyResp;

#[onebot_action("send_like", EmptyResp)]
pub struct SendLike {
    pub user_id: i64,
    pub times: Option<u16>,
}

#[onebot_action("set_friend_add_request", EmptyResp)]
pub struct SetFriendAddRequest {
    pub flag: String,
    pub approve: Option<bool>,
    pub remark: Option<String>,
}

#[onebot_action("get_stranger_info", StrangerInfoResp)]
pub struct GetStrangerInfo {
    pub user_id: i64,
    pub no_cache: Option<bool>,
}

#[json]
pub struct StrangerInfoResp {
    pub user_id: i64,
    pub nickname: String,
    pub sex: Sex,
    pub age: u32
}

#[onebot_action("get_friend_list", Vec<FriendInfo>)]
pub struct GetFriendList;

#[json]
pub struct FriendInfo {
    pub user_id: i64,
    pub nickname: String,
    pub remark: String
}
