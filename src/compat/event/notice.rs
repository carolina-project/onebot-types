use super::*;

pub mod ob11to12 {
    use ob11event::notice::*;
    use ob_types_base::ext::{IntoValue, ValueExt};
    use serde_value::Value;

    use crate::compat::compat_self;
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

    fn group_upload(self_id: String, file: GroupUpload) -> SerResult<ob12event::MessageEvent> {
        Ok(ob12event::MessageEvent {
            self_: compat_self(self_id),
            message_id: (),
            sub_type: (),
            message: (),
            alt_message: (),
            source: (),
            extra: (),
        })
    }

    impl IntoOB12Event<(String, String)> for  {
        
    }

    impl IntoOB12Event<String> for ob11event::NoticeEvent {
        type Output = ob12event::EventType;

        fn into_ob12(self, param: String) -> SerResult<Self::Output> {
            use ob11event::notice::*;
            match self {
                NoticeEvent::GroupNotice(group) => match group.kind {
                    GroupNoticeKind::Upload { file } => todo!(),
                    GroupNoticeKind::Admin { sub_type } => todo!(),
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
                },
                NoticeEvent::FriendNotice(_) => todo!(),
            }
        }
    }
}
