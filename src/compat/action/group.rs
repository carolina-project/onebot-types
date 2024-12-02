use super::*;

impl IntoOB11Action for ob12action::LeaveGroup {
    type Output = ob11action::SetGroupLeave;
    fn into_ob11(self, _: ()) -> crate::DesResult<Self::Output> {
        let mut map = unwrap_value_map(self.extra)?;
        Ok(ob11action::SetGroupLeave {
            group_id: self.group_id.parse().map_err(DeserializerError::custom)?,
            is_dismiss: remove_field_or_default(&mut map, "ob11.is_dismiss")?,
        })
    }
}

impl IntoOB11Action for ob12action::GetGroupInfo {
    type Output = ob11action::GetGroupInfo;

    fn into_ob11(self, _: ()) -> DesResult<Self::Output> {
        let mut map = unwrap_value_map(self.extra)?;
        Ok(ob11action::GetGroupInfo {
            group_id: self.group_id.parse().map_err(DeserializerError::custom)?,
            no_cache: remove_field_or_default(&mut map, "ob11.no_cache")?,
        })
    }
}

impl FromOB11Resp for ob12::GroupInfo {
    type In = ob11action::GroupInfo;

    fn from_ob11(from: Self::In, _: ()) -> DesResult<Self> {
        let ob11action::GroupInfo {
            group_id,
            group_name,
            member_count,
            max_member_count,
        } = from;
        let extra = Value::from_map(
            [
                ("ob11.member_count", member_count.into_value()),
                ("ob11.max_member_count", max_member_count.into_value()),
            ]
            .into(),
        );
        Ok(Self {
            group_id: group_id.to_string(),
            group_name,
            extra,
        })
    }
}

impl IntoOB11Action for ob12action::GetGroupList {
    type Output = ob11action::GetGroupList;

    fn into_ob11(self, _: ()) -> DesResult<Self::Output> {
        Ok(ob11action::GetGroupList {})
    }
}

impl FromOB11Resp for Vec<ob12::GroupInfo> {
    type In = Vec<ob11action::GroupInfo>;

    fn from_ob11(from: Self::In, param: ()) -> DesResult<Self> {
        from.into_iter()
            .map(|r| ob12::GroupInfo::from_ob11(r, param))
            .collect()
    }
}

impl IntoOB11Action for ob12action::SetGroupName {
    type Output = ob11action::SetGroupName;

    fn into_ob11(self, _: ()) -> DesResult<Self::Output> {
        Ok(ob11action::SetGroupName {
            group_id: self.group_id.parse().map_err(DeserializerError::custom)?,
            group_name: self.group_name,
        })
    }
}

impl IntoOB11Action for ob12action::GetGroupMemberInfo {
    type Output = ob11action::GetGroupMemberInfo;

    fn into_ob11(self, _: ()) -> DesResult<Self::Output> {
        let mut map = unwrap_value_map(self.extra)?;
        Ok(ob11action::GetGroupMemberInfo {
            group_id: self.group_id.parse().map_err(DeserializerError::custom)?,
            user_id: self.user_id.parse().map_err(DeserializerError::custom)?,
            no_cache: remove_field_or_default(&mut map, "ob11.no_cache")?,
        })
    }
}

impl IntoOB11Action for ob12action::GetGroupMemberList {
    type Output = ob11action::GetGroupMemberList;

    fn into_ob11(self, _: ()) -> DesResult<Self::Output> {
        Ok(ob11action::GetGroupMemberList {
            group_id: self.group_id.parse().map_err(DeserializerError::custom)?,
        })
    }
}
