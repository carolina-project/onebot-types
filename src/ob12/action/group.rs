use ob_types_macro::onebot_action;

use crate::{
    ob12::{GroupInfo, GroupMemberInfo},
    scalable_struct,
};

use super::EmptyResp;

scalable_struct! {
    #[onebot_action(GroupInfo)]
    GetGroupInfo = {
        group_id: String,
    },
    #[onebot_action(Vec<GroupInfo>)]
    GetGroupList,
    #[onebot_action(GroupMemberInfo)]
    GetGroupMemberInfo = {
        group_id: String,
        user_id: String
    },
    #[onebot_action(Vec<GroupMemberInfo>)]
    GetGroupMemberList = {
        group_id: String,
    },
    #[onebot_action(EmptyResp)]
    SetGroupName = {
        group_id: String,
        group_name: String,
    },
    #[onebot_action(EmptyResp)]
    LeaveGroup = {
        group_id: String,
    },
}
