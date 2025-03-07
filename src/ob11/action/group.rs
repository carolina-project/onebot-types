use std::time::Duration;

use ob_types_macro::__data;

use crate::{
    base::{define_action, MessageChain},
    ob11::event::{message::AnonymousSender, request::AddGroupType},
};

use crate::base::tool::duration_secs_opt;

use super::bot::MessageResp;
use super::EmptyResp;

define_action! {
    #[resp(MessageResp)]
    pub struct SendGroupMsg {
        pub group_id: i64,
        pub message: MessageChain,
    }
    #[resp(EmptyResp)]
    pub struct SetGroupKick {
        pub group_id: i64,
        pub user_id: i64,
        pub reject_add_request: Option<bool>,
    }
    #[resp(EmptyResp)]
    pub struct SetGroupBan {
        pub group_id: i64,
        pub user_id: i64,
        #[serde(with = "duration_secs_opt")]
        pub duration: Option<Duration>,
    }
    #[resp(EmptyResp)]
    pub struct SetGroupAnonymousBan {
        pub group_id: i64,
        pub anonymous: Option<AnonymousSender>,
        #[serde(alias = "flag")]
        pub anonymous_flag: Option<String>,
        #[serde(with = "duration_secs_opt")]
        pub duration: Option<Duration>,
    }
    #[resp(EmptyResp)]
    pub struct SetGroupWholeBan {
        pub group_id: i64,
        pub enable: Option<bool>,
    }
    #[resp(EmptyResp)]
    pub struct SetGroupAdmin {
        pub group_id: i64,
        pub user_id: i64,
        pub enable: Option<bool>,
    }
    #[resp(EmptyResp)]
    pub struct SetGroupAnonymous {
        pub group_id: i64,
        pub enable: Option<bool>,
    }
    #[resp(EmptyResp)]
    pub struct SetGroupCard {
        pub group_id: i64,
        pub user_id: i64,
        pub card: Option<String>,
    }
    #[resp(EmptyResp)]
    pub struct SetGroupName {
        pub group_id: i64,
        pub group_name: String,
    }
    #[resp(EmptyResp)]
    pub struct SetGroupLeave {
        pub group_id: i64,
        #[serde(default)]
        pub is_dismiss: bool,
    }
    #[resp(EmptyResp)]
    pub struct SetGroupSpecialTitle {
        pub group_id: i64,
        pub user_id: i64,
        pub special_title: Option<String>,
        #[serde(with = "duration_secs_opt")]
        pub duration: Option<Duration>,
    }
}

impl SendGroupMsg {
    pub fn new(group_id: i64) -> Self {
        Self {
            group_id,
            message: MessageChain::default(),
        }
    }

    pub fn message(mut self, message: impl Into<MessageChain>) -> Self {
        self.message = message.into();
        self
    }
}

impl SetGroupKick {
    pub fn new(group_id: i64, user_id: i64) -> Self {
        Self {
            group_id,
            user_id,
            reject_add_request: None,
        }
    }

    pub fn reject_add_request(mut self, reject_add_request: bool) -> Self {
        self.reject_add_request = Some(reject_add_request);
        self
    }
}

impl SetGroupBan {
    pub fn new(group_id: i64, user_id: i64) -> Self {
        Self {
            group_id,
            user_id,
            duration: None,
        }
    }

    pub fn duration(mut self, duration: Option<Duration>) -> Self {
        self.duration = duration;
        self
    }
}

impl SetGroupAnonymousBan {
    pub fn anonymous(group_id: i64, anonymous: AnonymousSender) -> Self {
        Self {
            group_id,
            anonymous: Some(anonymous),
            anonymous_flag: None,
            duration: None,
        }
    }

    pub fn anonymous_flag(group_id: i64, flag: impl Into<String>) -> Self {
        Self {
            group_id,
            anonymous: None,
            anonymous_flag: Some(flag.into()),
            duration: None,
        }
    }

    pub fn duration(mut self, duration: Duration) -> Self {
        self.duration = Some(duration);
        self
    }
}

impl SetGroupWholeBan {
    pub fn new(group_id: i64) -> Self {
        Self {
            group_id,
            enable: None,
        }
    }

    pub fn enable(mut self, enable: Option<bool>) -> Self {
        self.enable = enable;
        self
    }
}

impl SetGroupAdmin {
    pub fn new(group_id: i64, user_id: i64) -> Self {
        Self {
            group_id,
            user_id,
            enable: None,
        }
    }

    pub fn enable(mut self, enable: Option<bool>) -> Self {
        self.enable = enable;
        self
    }
}

impl SetGroupAnonymous {
    pub fn builder() -> Self {
        Self {
            group_id: 0,
            enable: None,
        }
    }

    pub fn group_id(mut self, group_id: i64) -> Self {
        self.group_id = group_id;
        self
    }

    pub fn enable(mut self, enable: Option<bool>) -> Self {
        self.enable = enable;
        self
    }

    pub fn build(self) -> Self {
        self
    }
}

impl SetGroupCard {
    pub fn new(group_id: i64, user_id: i64) -> Self {
        Self {
            group_id,
            user_id,
            card: None,
        }
    }

    pub fn card(mut self, card: Option<impl Into<String>>) -> Self {
        self.card = card.map(Into::into);
        self
    }
}

impl SetGroupName {
    pub fn new(group_id: i64, group_name: impl Into<String>) -> Self {
        Self {
            group_id,
            group_name: group_name.into(),
        }
    }
}

impl SetGroupLeave {
    pub fn new(group_id: i64) -> Self {
        Self {
            group_id,
            is_dismiss: false,
        }
    }

    pub fn is_dismiss(mut self, is_dismiss: bool) -> Self {
        self.is_dismiss = is_dismiss;
        self
    }
}

impl SetGroupSpecialTitle {
    pub fn new(group_id: i64, user_id: i64) -> Self {
        Self {
            group_id,
            user_id,
            special_title: None,
            duration: None,
        }
    }

    pub fn special_title(mut self, special_title: Option<String>) -> Self {
        self.special_title = special_title;
        self
    }

    pub fn duration(mut self, duration: Option<Duration>) -> Self {
        self.duration = duration;
        self
    }
}

const fn true_v() -> bool {
    true
}

define_action! {
    #[resp(EmptyResp)]
    pub struct SetGroupAddRequest {
        pub flag: String,
        pub sub_type: AddGroupType,
        #[serde(default = "true_v")]
        pub approve: bool,
        pub reason: Option<String>,
    }
    #[resp(GroupInfo)]
    pub struct GetGroupInfo {
        pub group_id: i64,
        #[serde(default)]
        pub no_cache: bool,
    }
}

impl SetGroupAddRequest {
    pub fn new(flag: impl Into<String>, sub_type: AddGroupType) -> Self {
        Self {
            flag: flag.into(),
            sub_type,
            approve: true,
            reason: None,
        }
    }

    pub fn approve(mut self, approve: bool) -> Self {
        self.approve = approve;
        self
    }

    pub fn reason(mut self, reason: impl Into<String>) -> Self {
        self.reason = Some(reason.into());
        self
    }
}

impl GetGroupInfo {
    pub fn new(group_id: i64) -> Self {
        Self {
            group_id,
            no_cache: false,
        }
    }

    pub fn no_cache(mut self, no_cache: bool) -> Self {
        self.no_cache = no_cache;
        self
    }
}

#[__data]
pub struct GroupInfo {
    pub group_id: i64,
    pub group_name: String,
    pub member_count: u32,
    pub max_member_count: u32,
}

define_action! {
    #[data(default)]
    #[resp(Vec<GroupInfo>)]
    pub struct GetGroupList;
    #[resp(GroupMemberInfo)]
    pub struct GetGroupMemberInfo {
        pub group_id: i64,
        pub user_id: i64,
        #[serde(default)]
        pub no_cache: bool,
    }
}

impl GetGroupMemberInfo {
    pub fn new(group_id: i64, user_id: i64) -> Self {
        Self {
            group_id,
            user_id,
            no_cache: false,
        }
    }

    pub fn no_cache(mut self, no_cache: bool) -> Self {
        self.no_cache = no_cache;
        self
    }
}

#[__data]
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

define_action! {
    #[resp(Vec<GroupMemberInfo>)]
    pub struct GetGroupMemberList {
        pub group_id: i64,
    }
}

impl GetGroupMemberList {
    pub fn new(group_id: i64) -> Self {
        Self { group_id }
    }
}

#[__data]
#[serde(rename_all = "snake_case")]
pub enum GroupHonor {
    Talkative,
    Performer,
    Legend,
    StrongNewbie,
    Emotion,
    All,
}

define_action! {
    #[resp(GroupHonorResp)]
    /// see [get_group_honor_info](https://github.com/botuniverse/onebot-11/blob/master/api/public.md#get_group_honor_info-%E8%8E%B7%E5%8F%96%E7%BE%A4%E8%8D%A3%E8%AA%89%E4%BF%A1%E6%81%AF)
    pub struct GetGroupHonorInfo {
        pub group_id: i64,
        pub r#type: GroupHonor,
    }
}

impl GetGroupHonorInfo {
    pub fn new(group_id: i64) -> Self {
        Self {
            group_id,
            r#type: GroupHonor::All,
        }
    }

    pub fn set_type(mut self, r#type: GroupHonor) -> Self {
        self.r#type = r#type;
        self
    }
}

#[__data]
pub struct GroupHonorUser {
    pub user_id: i64,
    pub nickname: String,
    pub avatar: String,
    pub description: String,
}

// -talkative
#[__data]
pub struct CurrentTalkative {
    pub user_id: i64,
    pub nickname: String,
    pub avatar: String,
    pub day_count: u32,
}

#[__data]
pub struct GroupHonorResp {
    pub group_id: i64,
    pub current_talkative: Option<CurrentTalkative>,
    pub talkative_list: Option<Vec<GroupHonorUser>>,
    pub performer_list: Option<Vec<GroupHonorUser>>,
    pub legend_list: Option<Vec<GroupHonorUser>>,
    pub strong_newbie_list: Option<Vec<GroupHonorUser>>,
    pub emotion_list: Option<Vec<GroupHonorUser>>,
}
