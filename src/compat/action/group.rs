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

