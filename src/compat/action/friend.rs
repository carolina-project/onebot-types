use super::*;

impl From<ob11action::LoginInfo> for UserInfoResp {
    fn from(value: ob11action::LoginInfo) -> Self {
        Self::LoginInfo(value)
    }
}

impl From<ob11action::StrangerInfoResp> for UserInfoResp {
    fn from(value: ob11action::StrangerInfoResp) -> Self {
        Self::StrangerInfo(value)
    }
}

impl IntoOB11Action for ob12action::GetSelfInfo {
    type Output = ob11action::GetLoginInfo;
    fn into_ob11(self, _: ()) -> DesResult<Self::Output> {
        Ok(ob11action::GetLoginInfo {})
    }
}

impl IntoOB11Action for ob12action::GetUserInfo {
    type Output = ob11action::GetStrangerInfo;
    fn into_ob11(self, _: ()) -> DesResult<Self::Output> {
        let mut map = self.extra;
        Ok(ob11action::GetStrangerInfo {
            user_id: self.user_id.parse().map_err(DeserializerError::custom)?,
            no_cache: remove_field_or_default(&mut map, "ob11.no_cache")?,
        })
    }
}

impl IntoOB11Action for ob12action::GetFriendList {
    type Output = ob11action::GetFriendList;
    fn into_ob11(self, _: ()) -> DesResult<Self::Output> {
        Ok(ob11action::GetFriendList {})
    }
}

impl FromOB11Resp for Vec<ob12::UserInfo> {
    type In = Vec<UserInfoResp>;

    fn from_ob11(from: Self::In, param: ()) -> DesResult<Self> {
        from.into_iter()
            .map(|r| ob12::UserInfo::from_ob11(r, param))
            .collect()
    }
}
