use std::time::Duration;

use ob_types_macro::__data;
use serde::Deserialize;
use serde_value::Value;

use crate::{base::ext::IntoValue, DesResult};

use super::*;

#[__data]
#[serde(tag = "detail_type")]
pub enum CompatGNoticeKind {
    #[serde(rename = "ob11.group_admin")]
    GroupAdmin {
        sub_type: ob11event::notice::AdminChange,
    },
    #[serde(rename = "ob11.group_ban")]
    GroupBan {
        sub_type: ob11event::notice::MuteType,
        operator_id: i64,
        duration: Duration,
    },
    #[serde(rename = "ob11.poke")]
    Poke { target_id: i64 },
    #[serde(rename = "ob11.lucky_king")]
    LuckyKing { target_id: i64 },
    #[serde(rename = "ob11.honor")]
    Honor {
        honor_type: ob11event::notice::GroupHonor,
    },
}

#[__data]
pub struct CompatGroupNotice {
    pub group_id: String,
    pub user_id: String,
    #[serde(flatten)]
    pub kind: CompatGNoticeKind,
}

impl CompatGroupNotice {
    pub fn parse_data(type_name: impl AsRef<str>, data: Value) -> DesResult<Self> {
        if let Value::Map(mut data) = data {
            data.insert("type".into_value(), type_name.as_ref().into_value());
            CompatGroupNotice::deserialize(Value::Map(data))
        } else {
            Err(serde::de::Error::custom("Invalid data format"))
        }
    }
}

pub mod ob11to12 {
    use ob11event::notice::*;
    use ob12event::EventDetailed;
    use serde::ser::Error;
    use serde_value::SerializerError;

    use crate::base::ext::ValueMapExt;
    use crate::base::MessageChain;
    use crate::compat::compat_self;
    use crate::ob12;
    use crate::ValueMap;

    use ob12event::EventKind as O12EventType;
    use ob12event::NoticeEvent as O12Notice;

    use super::IntoOB12Event;
    use super::*;

    impl From<GroupUploadFile> for ob12::message::File {
        fn from(value: GroupUploadFile) -> Self {
            Self {
                file_id: value.id,
                extra: [
                    ("ob11.name", value.name.into_value()),
                    ("ob11.size", value.size.into_value()),
                    ("ob11.busid", value.busid.into_value()),
                ]
                .into_map(),
            }
        }
    }

    fn group_value_map(notice: CompatGroupNotice) -> SerResult<(ValueMap, String)> {
        #[derive(Deserialize)]
        struct Helper {
            detail_type: String,
            #[serde(flatten)]
            data: ValueMap,
        }

        let value = serde_value::to_value(notice)?;
        let helper = Helper::deserialize(value).map_err(SerializerError::custom)?;
        Ok((helper.data, helper.detail_type))
    }

    #[inline]
    fn other_group_notice_event(
        self_id: String,
        group_id: String,
        user_id: String,
        kind: CompatGNoticeKind,
    ) -> SerResult<ob12event::EventKind> {
        let (mut detail, detail_type) = group_value_map(CompatGroupNotice {
            group_id,
            user_id,
            kind,
        })?;
        detail.insert("self".into(), serde_value::to_value(compat_self(self_id))?);
        Ok(ob12event::EventKind::Notice(O12Notice::Other(
            EventDetailed {
                detail_type,
                detail,
            },
        )))
    }

    #[inline]
    fn group_upload_convert(
        self_id: String,
        group_id: String,
        user_id: String,
        message_id: String,
        upload: GroupUploadFile,
    ) -> SerResult<ob12event::EventKind> {
        use ob12event::message::{Group, MessageArgs, MessageEvent};
        let event = Group {
            group_id,
            args: MessageArgs {
                self_: compat_self(self_id),
                message_id,
                user_id,
                sub_type: Default::default(),
                message: MessageChain::try_from_seg(ob12::MessageSeg::File(upload.into()))?,
                alt_message: Some("[OneBot 11 File]".into()),
                extra: Default::default(),
            },
        };

        Ok(ob12event::EventKind::Message(MessageEvent::Group(event)))
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
        type Output = ob12event::EventKind;

        fn into_ob12(self, param: (String, F)) -> SerResult<Self::Output> {
            use ob11event::notice::*;
            use ob12event::notice;
            let (self_id, msg_id_provider) = param;
            match self {
                NoticeEvent::GroupUpload(GroupUpload {
                    group_id,
                    user_id,
                    file,
                }) => group_upload_convert(
                    self_id,
                    group_id.to_string(),
                    user_id.to_string(),
                    msg_id_provider(&file),
                    file,
                ),
                NoticeEvent::GroupAdmin(GroupAdmin {
                    group_id,
                    user_id,
                    sub_type,
                }) => other_group_notice_event(
                    self_id.to_string(),
                    group_id.to_string(),
                    user_id.to_string(),
                    CompatGNoticeKind::GroupAdmin { sub_type },
                ),
                NoticeEvent::GroupIncrease(GroupIncrease {
                    group_id,
                    user_id,
                    sub_type,
                    operator_id,
                }) => Ok(O12EventType::Notice(
                    ob12event::notice::GroupMemberIncrease {
                        self_: compat_self(self_id),
                        sub_type: sub_type.into(),
                        group_id: group_id.to_string(),
                        user_id: user_id.to_string(),
                        operator_id: operator_id.to_string(),
                        extra: Default::default(),
                    }
                    .into(),
                )),
                NoticeEvent::GroupDecrease(GroupDecrease {
                    group_id,
                    user_id,
                    sub_type,
                    operator_id,
                }) => Ok(O12EventType::Notice(
                    ob12event::notice::GroupMemberDecrease {
                        self_: compat_self(self_id),
                        sub_type: sub_type.into(),
                        group_id: group_id.to_string(),
                        user_id: user_id.to_string(),
                        operator_id: operator_id.to_string(),
                        extra: Default::default(),
                    }
                    .into(),
                )),
                NoticeEvent::GroupBan(GroupBan {
                    group_id,
                    user_id,
                    sub_type,
                    operator_id,
                    duration,
                }) => other_group_notice_event(
                    self_id.to_string(),
                    group_id.to_string(),
                    user_id.to_string(),
                    CompatGNoticeKind::GroupBan {
                        sub_type,
                        operator_id,
                        duration,
                    },
                ),
                NoticeEvent::GroupRecall(GroupRecall {
                    group_id,
                    user_id,
                    operator_id,
                    message_id,
                }) => Ok(O12EventType::Notice(
                    ob12event::notice::GroupMessageDelete {
                        self_: compat_self(self_id),
                        sub_type: if operator_id == user_id {
                            notice::MessageDeleteType::Recall
                        } else {
                            notice::MessageDeleteType::Delete
                        },
                        message_id: message_id.to_string(),
                        group_id: group_id.to_string(),
                        user_id: user_id.to_string(),
                        operator_id: operator_id.to_string(),
                        extra: Default::default(),
                    }
                    .into(),
                )),
                NoticeEvent::Poke(Poke {
                    group_id,
                    user_id,
                    target_id,
                }) => other_group_notice_event(
                    self_id.to_string(),
                    group_id.to_string(),
                    user_id.to_string(),
                    CompatGNoticeKind::Poke { target_id },
                ),
                NoticeEvent::LuckyKing(LuckyKing {
                    group_id,
                    user_id,
                    target_id,
                }) => other_group_notice_event(
                    self_id.to_string(),
                    group_id.to_string(),
                    user_id.to_string(),
                    CompatGNoticeKind::LuckyKing { target_id },
                ),
                NoticeEvent::Honor(Honor {
                    group_id,
                    user_id,
                    honor_type,
                }) => other_group_notice_event(
                    self_id.to_string(),
                    group_id.to_string(),
                    user_id.to_string(),
                    CompatGNoticeKind::Honor { honor_type },
                ),
                NoticeEvent::FriendAdd(FriendAdd { user_id }) => Ok(O12EventType::Notice(
                    notice::FriendIncrease {
                        self_: compat_self(self_id),
                        sub_type: Default::default(),
                        user_id: user_id.to_string(),
                        extra: Default::default(),
                    }
                    .into(),
                )),
                NoticeEvent::FriendRecall(FriendRecall {
                    user_id,
                    message_id,
                }) => Ok(O12EventType::Notice(
                    notice::PrivateMessageDelete {
                        self_: compat_self(self_id),
                        sub_type: Default::default(),
                        message_id: message_id.to_string(),
                        user_id: user_id.to_string(),
                        extra: Default::default(),
                    }
                    .into(),
                )),
            }
        }
    }
}
