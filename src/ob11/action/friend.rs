use ob_types_macro::onebot_action;

use crate::ob11::event::request::AddGroupType;

use super::bot::EmptyResp;

#[onebot_action("send_like", EmptyResp)]
pub struct SendLike {
    pub user_id: u64,
    pub times: Option<u16>,
}

#[onebot_action("set_friend_add_request", EmptyResp)]
pub struct SetFriendAddRequest {
    pub flag: String,
    pub approve: Option<bool>,
    pub remark: Option<String>,
}

#[onebot_action("set_group_add_request", EmptyResp)]
pub struct SetGroupAddRequest {
    pub flag: String,
    pub sub_type: AddGroupType,
    pub approve: Option<bool>,
    pub reason: Option<String>,
}
