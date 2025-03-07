use crate::{
    ob12::{GroupInfo, UserInfo},
    scalable_struct,
};

use super::EmptyResp;

scalable_struct! {
    #[resp(GroupInfo)]
    GetGroupInfo = {
        group_id: String,
    },
    #[resp(Vec<GroupInfo>)]
    GetGroupList,
    #[resp(UserInfo)]
    GetGroupMemberInfo = {
        group_id: String,
        user_id: String
    },
    #[resp(Vec<UserInfo>)]
    GetGroupMemberList = {
        group_id: String,
    },
    #[resp(EmptyResp)]
    SetGroupName = {
        group_id: String,
        group_name: String,
    },
    #[resp(EmptyResp)]
    LeaveGroup = {
        group_id: String,
    },
}

impl GetGroupInfo {
    pub fn new(group_id: impl Into<String>) -> Self {
        Self {
            group_id: group_id.into(),
            extra: Default::default(),
        }
    }
}

impl GetGroupList {
    pub fn new() -> Self {
        GetGroupList {
            extra: Default::default(),
        }
    }
}

impl GetGroupMemberInfo {
    pub fn new(group_id: impl Into<String>, user_id: impl Into<String>) -> Self {
        GetGroupMemberInfo {
            group_id: group_id.into(),
            user_id: user_id.into(),
            extra: Default::default(),
        }
    }
}

impl GetGroupMemberList {
    pub fn new(group_id: impl Into<String>) -> Self {
        GetGroupMemberList {
            group_id: group_id.into(),
            extra: Default::default(),
        }
    }
}

impl SetGroupName {
    pub fn new(group_id: impl Into<String>, group_name: impl Into<String>) -> Self {
        SetGroupName {
            group_id: group_id.into(),
            group_name: group_name.into(),
            extra: Default::default(),
        }
    }
}

impl LeaveGroup {
    pub fn new(group_id: impl Into<String>) -> Self {
        LeaveGroup {
            group_id: group_id.into(),
            extra: Default::default(),
        }
    }
}
