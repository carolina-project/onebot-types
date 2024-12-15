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
