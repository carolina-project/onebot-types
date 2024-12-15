use crate::{ob12::UserInfo, scalable_struct};

scalable_struct! {
    #[resp(UserInfo)]
    GetSelfInfo,
    #[resp(UserInfo)]
    GetUserInfo = {
        user_id: String,
    },
    #[resp(Vec<UserInfo>)]
    GetFriendList,
}
