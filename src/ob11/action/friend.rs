use ob_types_macro::__data;

use crate::base::{define_action, MessageChain};
use crate::ob11::Sex;

use super::bot::MessageResp;
use super::EmptyResp;

const fn true_value() -> bool {
    true
}

#[__data]
pub struct StrangerInfoResp {
    pub user_id: i64,
    pub nickname: String,
    pub sex: Sex,
    pub age: u32,
}

#[__data]
pub struct FriendInfo {
    pub user_id: i64,
    pub nickname: String,
    pub remark: String,
}

define_action! {
    #[resp(MessageResp)]
    pub struct SendPrivateMsg {
        pub user_id: i64,
        pub message: MessageChain,
    }
    #[resp(EmptyResp)]
    pub struct SendLike {
        pub user_id: i64,
        pub times: Option<u16>,
    }
    #[resp(EmptyResp)]
    pub struct SetFriendAddRequest {
        pub flag: String,
        #[serde(default = "true_value")]
        pub approve: bool,
        #[serde(default)]
        pub remark: Option<String>,
    }
    #[resp(StrangerInfoResp)]
    pub struct GetStrangerInfo {
        pub user_id: i64,
        #[serde(default)]
        pub no_cache: bool,
    }
    #[resp(Vec<FriendInfo>)]
    pub struct GetFriendList;
}

