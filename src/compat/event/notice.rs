use super::*;

macro_rules! define_compat_types {
    ($($typ:ident $name:literal),*) => {
        pub enum OB12CompatNotice {
            $($typ {
                data: ob11event::notice::$typ 
            }),*
        }

        impl OB12CompatNotice {
            pub fn parse_data(
                name: &str, data: serde_value::Value
            ) -> Option<Result<Self, serde_value::DeserializerError>> {
                match name {
                    $(concat!("ob11.", $name) => {
                        Some(ob11message::$typ::deserialize(data).map(OB12CompatNotice::$typ))
                    })*
                    _ => None,
                }
            }

            pub fn into_data(self) -> Result<(&'static str, serde_value::Value), serde_value::SerializerError> {
                match self {
                    $(
                        OB12CompatNotice::$typ(data)
                            => Ok((concat!("ob11.", $name), serde_value::to_value(data)?)),
                    )*
                }
            }
        }
    };
}

define_compat_types!(
    
);

pub mod ob11to12 {
    use ob11event::notice::*;
    use ob_types_base::ext::{IntoValue, ValueExt};
    use serde_value::Value;

    use crate::compat::{compat_self, default_obj};
    use crate::ob12;

    use super::IntoOB12Event;
    use super::*;

    impl From<GroupUpload> for ob12::message::File {
        fn from(value: GroupUpload) -> Self {
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

    #[inline]
    pub fn group_upload_convert(
        self_id: String,
        group_id: String,
        user_id: String,
        message_id: String,
        upload: GroupUpload,
    ) -> SerResult<ob12event::MessageEvent> {
        Ok(ob12event::MessageEvent {
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
        })
    }

    /// (String, F): self id and message_id provider(from GroupUpload)
    impl<F> IntoOB12Event<(String, F)> for ob11event::NoticeEvent
    where
        F: FnOnce(ob11event::notice::GroupUpload) -> String,
    {
        type Output = ob12event::EventType;

        fn into_ob12(self, param: (String, F)) -> SerResult<Self::Output> {
            use ob11event::notice::*;
            let (self_id, msg_id_provider) = param;
            match self {
                NoticeEvent::GroupNotice(group) => {
                    let GroupNotice {
                        group_id,
                        user_id,
                        kind,
                    } = group;
                    match kind {
                        GroupNoticeKind::Upload { file } => group_upload_convert(
                            self_id,
                            group_id.to_string(),
                            user_id.to_string(),
                            msg_id_provider(file),
                            file,
                        ),
                        GroupNoticeKind::Admin { sub_type } => {
                            ob12event::notice::
                        }
                        GroupNoticeKind::MemberIncrease {
                            sub_type,
                            operator_id,
                        } => todo!(),
                        GroupNoticeKind::MemberDecrease {
                            sub_type,
                            operator_id,
                        } => todo!(),
                        GroupNoticeKind::Mute {
                            sub_type,
                            operator_id,
                            duration,
                        } => todo!(),
                        GroupNoticeKind::Recall {
                            operator_id,
                            message_id,
                        } => todo!(),
                        GroupNoticeKind::Poke { target_id } => todo!(),
                        GroupNoticeKind::LuckyKing { target_id } => todo!(),
                        GroupNoticeKind::Honor { honor_type } => todo!(),
                    }
                }
                NoticeEvent::FriendNotice(_) => todo!(),
            }
        }
    }
}
