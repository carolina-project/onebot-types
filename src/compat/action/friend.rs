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
        let mut extra = unwrap_value_map(self.extra)?;
        Ok(ob11action::GetStrangerInfo {
            user_id: self.user_id.parse().map_err(DeserializerError::custom)?,
            no_cache: remove_field_or_default(&mut extra, "ob11.no_cache")?,
        })
    }
}
