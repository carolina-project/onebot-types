use ob_types_macro::__data;

use crate::base::{define_action, MessageChain};
use crate::ob11::Sex;

use super::bot::MessageResp;
use super::EmptyResp;

const fn true_value() -> bool {
    true
}

#[__data]
pub struct StrangerInfo {
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
    #[resp(StrangerInfo)]
    pub struct GetStrangerInfo {
        pub user_id: i64,
        #[serde(default)]
        pub no_cache: bool,
    }
    #[data(default)]
    #[resp(Vec<FriendInfo>)]
    pub struct GetFriendList;
}

impl SendPrivateMsg {
    pub fn new(user_id: i64) -> Self {
        Self {
            user_id,
            message: Default::default(),
        }
    }

    pub fn message_chain(mut self, message: impl Into<MessageChain>) -> Self {
        self.message = message.into();
        self
    }
}

impl SendLike {
    pub fn new(user_id: i64) -> Self {
        Self {
            user_id,
            times: None,
        }
    }

    pub fn times(mut self, times: u16) -> Self {
        self.times = Some(times);
        self
    }
}

impl SetFriendAddRequest {
    pub fn new(flag: String) -> Self {
        Self {
            flag,
            approve: true,
            remark: None,
        }
    }

    pub fn approve(mut self, approve: bool) -> Self {
        self.approve = approve;
        self
    }

    pub fn remark(mut self, remark: impl Into<Option<String>>) -> Self {
        self.remark = remark.into();
        self
    }
}

impl GetStrangerInfo {
    pub fn new(user_id: i64) -> Self {
        Self {
            user_id,
            no_cache: false,
        }
    }

    pub fn no_cache(mut self, no_cache: bool) -> Self {
        self.no_cache = no_cache;
        self
    }
}
