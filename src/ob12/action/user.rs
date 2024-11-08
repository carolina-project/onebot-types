use ob_types_macro::onebot_action;

use crate::{ob12::UserInfo, scalable_struct};

scalable_struct! {
    #[onebot_action(UserInfo)]
    GetSelfInfo,
    #[onebot_action(UserInfo)]
    GetUserInfo = {
        user_id: String,
    },
    #[onebot_action(Vec<UserInfo>)]
    GetFriendList,
}
