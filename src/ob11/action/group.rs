use std::time::Duration;

use ob_types_macro::{native_data, onebot_action};

use crate::ob11::event::message::AnonymousSender;

use super::bot::EmptyResp;

#[onebot_action("set_group_kick", EmptyResp)]
pub struct SetGroupKick {
    pub group_id: u64,
    pub user_id: u64,
    pub reject_add_request: Option<bool>,
}

#[onebot_action("set_group_ban", EmptyResp)]
pub struct SetGroupBan {
    pub group_id: u64,
    pub user_id: u64,
    pub duration: Option<Duration>,
}

#[native_data]
pub enum AnonymousFlag {
    Sender(AnonymousSender),
    Flag(String)
}

#[onebot_action("set_group_anonymous_ban", EmptyResp)]
pub struct SetGroupAnonymousBan {
    pub group_id: u64,
    pub anonymous: AnonymousFlag,
    pub duration: Option<Duration>,
}

#[onebot_action("set_group_whole_ban", EmptyResp)]
pub struct SetGroupWholeBan {
    pub group_id: u64,
    pub enable: Option<bool>,
}

#[onebot_action("set_group_admin", EmptyResp)]
pub struct SetGroupAdmin {
    pub group_id: u64,
    pub user_id: u64,
    pub enable: Option<bool>,
}

#[onebot_action("set_group_anonymous", EmptyResp)]
pub struct SetGroupAnonymous {
    pub group_id: u64,
    pub enable: Option<bool>,
}

#[onebot_action("set_group_card", EmptyResp)]
pub struct SetGroupCard {
    pub group_id: u64,
    pub user_id: u64,
    pub card: Option<String>,
}

#[onebot_action("set_group_name", EmptyResp)]
pub struct SetGroupName {
    pub group_id: u64,
    pub group_name: String,
}

#[onebot_action("set_group_leave", EmptyResp)]
pub struct SetGroupLeave {
    pub group_id: u64,
    pub is_dismiss: Option<bool>,
}

#[onebot_action("set_group_special_title", EmptyResp)]
pub struct SetGroupSpecialTitle {
    pub group_id: u64,
    pub user_id: u64,
    pub special_title: Option<String>,
    pub duration: Option<Duration>,
}
