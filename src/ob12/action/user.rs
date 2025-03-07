use crate::{ob12::UserInfo, scalable_struct};

scalable_struct! {
    #[resp(UserInfo)]
    #[derive(Default)]
    GetSelfInfo,
    #[resp(UserInfo)]
    GetUserInfo = {
        user_id: String,
    },
    #[resp(Vec<UserInfo>)]
    #[derive(Default)]
    GetFriendList,
}

impl GetUserInfo {
    pub fn new(user_id: impl Into<String>) -> Self {
        Self {
            user_id: user_id.into(),
            extra: Default::default(),
        }
    }
}
