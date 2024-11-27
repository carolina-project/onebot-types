use std::time::Duration;

use ob_types_macro::{data, onebot_action};
use serde::{Deserialize, Serialize};

use crate::ob11::{
    event::{message::AnonymousSender, request::AddGroupType},
    message::MessageChain,
};

use ob_types_base::tool::duration_secs_opt;

use super::bot::MessageResp;
use super::EmptyResp;

#[onebot_action(MessageResp)]
pub struct SendGroupMsg {
    pub group_id: i64,
    pub message: MessageChain,
}

#[onebot_action(EmptyResp)]
pub struct SetGroupKick {
    pub group_id: i64,
    pub user_id: i64,
    pub reject_add_request: Option<bool>,
}

#[onebot_action(EmptyResp)]
pub struct SetGroupBan {
    pub group_id: i64,
    pub user_id: i64,
    #[serde(with = "duration_secs_opt")]
    pub duration: Option<Duration>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AnonymousFlag {
    Sender(AnonymousSender),
    Flag(String),
}

#[onebot_action(EmptyResp)]
pub struct SetGroupAnonymousBan {
    pub group_id: i64,
    #[serde(alias = "anonymous_flag", alias = "flag")]
    pub anonymous: AnonymousFlag,
    #[serde(with = "duration_secs_opt")]
    pub duration: Option<Duration>,
}

#[onebot_action(EmptyResp)]
pub struct SetGroupWholeBan {
    pub group_id: i64,
    pub enable: Option<bool>,
}

#[onebot_action(EmptyResp)]
pub struct SetGroupAdmin {
    pub group_id: i64,
    pub user_id: i64,
    pub enable: Option<bool>,
}

#[onebot_action(EmptyResp)]
pub struct SetGroupAnonymous {
    pub group_id: i64,
    pub enable: Option<bool>,
}

#[onebot_action(EmptyResp)]
pub struct SetGroupCard {
    pub group_id: i64,
    pub user_id: i64,
    pub card: Option<String>,
}

#[onebot_action(EmptyResp)]
pub struct SetGroupName {
    pub group_id: i64,
    pub group_name: String,
}

#[onebot_action(EmptyResp)]
pub struct SetGroupLeave {
    pub group_id: i64,
    pub is_dismiss: Option<bool>,
}

#[onebot_action(EmptyResp)]
pub struct SetGroupSpecialTitle {
    pub group_id: i64,
    pub user_id: i64,
    pub special_title: Option<String>,
    #[serde(with = "duration_secs_opt")]
    pub duration: Option<Duration>,
}

#[onebot_action(EmptyResp)]
pub struct SetGroupAddRequest {
    pub flag: String,
    pub sub_type: AddGroupType,
    pub approve: Option<bool>,
    pub reason: Option<String>,
}

#[onebot_action(GroupInfo)]
pub struct GetGroupInfo {
    pub group_id: i64,
    pub no_cache: Option<bool>,
}

#[data]
pub struct GroupInfo {
    pub group_id: i64,
    pub group_name: String,
    pub member_count: u32,
    pub max_member_count: u32,
}

#[onebot_action(Vec<GroupInfo>)]
pub struct GetGroupList;

#[onebot_action(GroupMemberInfo)]
pub struct GetGroupMemberInfo {
    pub group_id: i64,
    pub user_id: i64,
    pub no_cache: Option<bool>,
}

#[data]
pub struct GroupMemberInfo {
    pub group_id: i64,
    pub user_id: i64,
    pub nickname: String,
    pub card: String,
    pub sex: String,
    pub age: u32,
    pub area: String,
    pub join_time: u32,
    pub last_sent_time: u32,
    pub level: String,
    pub role: String,
    pub unfriendly: bool,
    pub title: String,
    pub title_expire_time: u32,
    pub card_changeable: bool,
}

#[onebot_action(Vec<GroupMemberInfo>)]
pub struct GetGroupMemberList {
    pub group_id: i64,
}

#[data]
#[serde(rename_all = "snake_case")]
pub enum GroupHonor {
    Talkative,
    Performer,
    Legend,
    StrongNewbie,
    Emotion,
    All,
}

/// see [get_group_honor_info](https://github.com/botuniverse/onebot-11/blob/master/api/public.md#get_group_honor_info-%E8%8E%B7%E5%8F%96%E7%BE%A4%E8%8D%A3%E8%AA%89%E4%BF%A1%E6%81%AF)
#[onebot_action(GroupHonorResp)]
pub struct GetGroupHonorInfo {
    pub group_id: i64,
    pub r#type: GroupHonor,
}

#[data]
pub struct GroupHonorUser {
    pub user_id: i64,
    pub nickname: String,
    pub avatar: String,
    pub description: String,
}

// -talkative
#[data]
pub struct CurrentTalkative {
    pub user_id: i64,
    pub nickname: String,
    pub avatar: String,
    pub day_count: u32,
}

#[data]
pub struct GroupHonorResp {
    pub group_id: i64,
    pub current_talkative: Option<CurrentTalkative>,
    pub talkative_list: Option<Vec<GroupHonorUser>>,
    pub performer_list: Option<Vec<GroupHonorUser>>,
    pub legend_list: Option<Vec<GroupHonorUser>>,
    pub strong_newbie_list: Option<Vec<GroupHonorUser>>,
    pub emotion_list: Option<Vec<GroupHonorUser>>,
}
