use ob_types_macro::data;

use super::*;

#[data]
#[serde(tag = "type")]
pub enum CompatGNoticeKind {
    #[serde(rename = "ob11.group_admin")]
    GroupAdmin(ob11event::notice::GroupAdmin),
    #[serde(rename = "ob11.group_ban")]
    GroupBan(ob11event::notice::GroupBan),
    #[serde(rename = "ob11.poke")]
    Poke(ob11event::notice::Poke),
    #[serde(rename = "ob11.lucky_king")]
    LuckyKing(ob11event::notice::LuckyKing),
    #[serde(rename = "ob11.honor")]
    Honor(ob11event::notice::Honor),
}

#[data]
pub struct CompatGroupNotice {
    pub group_id: String,
    pub user_id: String,
    #[serde(flatten)]
    pub kind: CompatGNoticeKind,
}

pub mod ob11to12 {
    use ob11event::notice::*;
    use ob_types_base::ext::{IntoValue, ValueExt};
    use serde::ser::Error;
    use serde_value::Value;

    use crate::compat::{compat_self, default_obj};
    use crate::ob12;

    use ob12event::notice::NoticeKind as O12NoticeKind;
    use ob12event::EventType as O12EventType;
    use ob12event::NoticeEvent as O12Notice;

    use super::IntoOB12Event;
    use super::*;

    impl From<GroupUploadFile> for ob12::message::File {
        fn from(value: GroupUploadFile) -> Self {
            Self {
                file_id: value.id,
                extra: Value::from_map(
                    [
                        ("ob11.name", value.name.into_value()),
                        ("ob11.size", value.size.into_value()),
                        ("ob11.busid", value.busid.into_value()),
                    ]
                    .into(),
                ),
            }
        }
    }

    fn group_value_map(
        group_id: String,
        user_id: String,
        kind: CompatGNoticeKind,
    ) -> SerResult<(Value, String)> {
        if let Value::Map(mut data) = serde_value::to_value(CompatGroupNotice {
            group_id,
            user_id,
            kind,
        })? {
            let Value::String(r#type) = data
                .remove(&"type".into_value())
                .ok_or_else(|| serde_value::SerializerError::custom("Missing type field"))?
            else {
                return Err(serde_value::SerializerError::custom("Expected a string"));
            };
            Ok((data.into_value(), r#type))
        } else {
            Err(serde_value::SerializerError::custom("Expected a map"))
        }
    }

    #[inline]
    fn other_group_notice_event(
        self_id: String,
        group_id: String,
        user_id: String,
        kind: CompatGNoticeKind,
    ) -> SerResult<ob12event::EventType> {
        let (data, type_) = group_value_map(group_id, user_id, kind)?;
        Ok(ob12event::EventType::Notice(O12Notice {
            self_: compat_self(self_id),
            kind: O12NoticeKind::Other {
                detail_type: type_,
                data,
            },
        }))
    }

    #[inline]
    fn group_upload_convert(
        self_id: String,
        group_id: String,
        user_id: String,
        message_id: String,
        upload: GroupUploadFile,
    ) -> SerResult<ob12event::EventType> {
        Ok(ob12event::EventType::Message(ob12event::MessageEvent {
            self_: compat_self(self_id),
            message_id,
            sub_type: String::default(),
            message: ob12::message::MessageChain::Array(vec![ob12::MessageSeg::File(
                upload.into(),
            )]),
            alt_message: Some("[OneBot 11 File]".into()),
            source: ob12::ChatTarget::Group {
                group_id,
                user_id: Some(user_id),
            },
            extra: default_obj(),
        }))
    }

    impl From<IncreaseType> for ob12event::notice::IncreaseType {
        fn from(value: IncreaseType) -> Self {
            match value {
                IncreaseType::Approve => Self::Join,
                IncreaseType::Invite => Self::Invite,
            }
        }
    }

    impl From<DecreaseType> for ob12event::notice::DecreaseType {
        fn from(value: DecreaseType) -> Self {
            match value {
                DecreaseType::Leave => Self::Leave,
                DecreaseType::Kick => Self::Kick,
                DecreaseType::KickMe => Self::Kick,
            }
        }
    }

    /// (String, F): self id and message_id provider(from GroupUpload)
    impl<F> IntoOB12Event<(String, F)> for ob11event::NoticeEvent
    where
        F: FnOnce(&ob11event::notice::GroupUploadFile) -> String,
    {
        type Output = ob12event::EventType;

        fn into_ob12(self, param: (String, F)) -> SerResult<Self::Output> {
            use ob11event::notice::*;
            use ob12event::notice;
            let (self_id, msg_id_provider) = param;
            match self {
                NoticeEvent::GroupNotice(group) => {
                    let GroupNotice {
                        group_id,
                        user_id,
                        kind,
                    } = group;
                    match kind {
                        GroupNoticeKind::GroupUpload(GroupUpload { file }) => group_upload_convert(
                            self_id,
                            group_id.to_string(),
                            user_id.to_string(),
                            msg_id_provider(&file),
                            file,
                        ),
                        GroupNoticeKind::GroupAdmin(admin) => other_group_notice_event(
                            self_id.to_string(),
                            group_id.to_string(),
                            user_id.to_string(),
                            CompatGNoticeKind::GroupAdmin(admin),
                        ),
                        GroupNoticeKind::GroupIncrease(GroupIncrease {
                            sub_type,
                            operator_id,
                        }) => Ok(O12EventType::Notice(O12Notice {
                            self_: compat_self(self_id),
                            kind: ob12event::notice::GroupMemberIncrease {
                                sub_type: sub_type.into(),
                                group_id: group_id.to_string(),
                                user_id: user_id.to_string(),
                                operator_id: operator_id.to_string(),
                                extra: default_obj(),
                            }
                            .into(),
                        })),
                        GroupNoticeKind::GroupDecrease(GroupDecrease {
                            sub_type,
                            operator_id,
                        }) => Ok(O12EventType::Notice(O12Notice {
                            self_: compat_self(self_id),
                            kind: ob12event::notice::GroupMemberDecrease {
                                sub_type: sub_type.into(),
                                group_id: group_id.to_string(),
                                user_id: user_id.to_string(),
                                operator_id: operator_id.to_string(),
                                extra: default_obj(),
                            }
                            .into(),
                        })),
                        GroupNoticeKind::GroupBan(ban) => other_group_notice_event(
                            self_id.to_string(),
                            group_id.to_string(),
                            user_id.to_string(),
                            CompatGNoticeKind::GroupBan(ban),
                        ),
                        GroupNoticeKind::GroupRecall(GroupRecall {
                            operator_id,
                            message_id,
                        }) => Ok(O12EventType::Notice(O12Notice {
                            self_: compat_self(self_id),
                            kind: ob12event::notice::GroupMessageDelete {
                                sub_type: if operator_id == user_id {
                                    notice::MessageDeleteType::Recall
                                } else {
                                    notice::MessageDeleteType::Delete
                                },
                                message_id: message_id.to_string(),
                                group_id: group_id.to_string(),
                                user_id: user_id.to_string(),
                                operator_id: operator_id.to_string(),
                                extra: default_obj(),
                            }
                            .into(),
                        })),
                        GroupNoticeKind::Poke(poke) => other_group_notice_event(
                            self_id.to_string(),
                            group_id.to_string(),
                            user_id.to_string(),
                            CompatGNoticeKind::Poke(poke),
                        ),
                        GroupNoticeKind::LuckyKing(luck) => other_group_notice_event(
                            self_id.to_string(),
                            group_id.to_string(),
                            user_id.to_string(),
                            CompatGNoticeKind::LuckyKing(luck),
                        ),
                        GroupNoticeKind::Honor(honor) => other_group_notice_event(
                            self_id.to_string(),
                            group_id.to_string(),
                            user_id.to_string(),
                            CompatGNoticeKind::Honor(honor),
                        ),
                    }
                }
                NoticeEvent::FriendNotice(friend) => {
                    let FriendNotice { user_id, kind } = friend;
                    match kind {
                        FriendNoticeKind::FriendAdd(_) => Ok(O12EventType::Notice(O12Notice {
                            self_: compat_self(self_id),
                            kind: notice::FriendIncrease {
                                sub_type: Default::default(),
                                user_id: user_id.to_string(),
                                extra: default_obj(),
                            }
                            .into(),
                        })),
                        FriendNoticeKind::FriendRecall(FriendRecall { message_id }) => {
                            Ok(O12EventType::Notice(O12Notice {
                                self_: compat_self(self_id),
                                kind: notice::PrivateMessageDelete {
                                    sub_type: Default::default(),
                                    message_id: message_id.to_string(),
                                    user_id: user_id.to_string(),
                                    extra: default_obj(),
                                }.into(),
                            }))
                        }
                    }
                }
            }
        }
    }
}
