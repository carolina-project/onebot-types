use super::*;

pub mod ob11to12 {

    use crate::compat::compat_self;

    use super::*;
    use crate::base::ext::{IntoValue, ValueMapExt};
    use crate::base::{tool, MessageChain, RawMessageSeg};
    use ob11event::message::*;
    use serde_value::SerializerError;

    /// Converts an OB11 MessageEvent into an OB12 MessageEvent using the provided transformation function(transform message segment).
    impl<F, R> IntoOB12EventAsync<(String, F)> for ob11event::MessageEvent
    where
        F: (Fn(RawMessageSeg) -> R) + Send,
        R: Future<Output = Result<RawMessageSeg, SerializerError>> + Send,
    {
        type Output = ob12event::MessageEvent;

        async fn into_ob12(self, param: (String, F)) -> SerResult<Self::Output> {
            use ob12event::message::{Group, MessageArgs, MessageEvent as O12MsgEvent, Private};

            let (self_id, trans_fn) = param;
            match self {
                MessageEvent::Private(PrivateMessage {
                    sub_type,
                    sender,
                    message:
                        Message {
                            message_id,
                            user_id,
                            message_segs,
                            raw_message,
                            font,
                        },
                }) => {
                    let mut message = vec![];
                    for ele in message_segs.into_inner() {
                        message.push(trans_fn(ele).await?);
                    }

                    let sub_type = tool::serde_to_string(sub_type)?;

                    let mut extra = [("ob11.font", font.into_value())].into_map();
                    extra.insert("ob11.sender".into(), serde_value::to_value(sender)?);

                    let args = ob12event::message::MessageArgs {
                        self_: compat_self(self_id),
                        message_id: message_id.to_string(),
                        user_id: user_id.to_string(),
                        sub_type,
                        message: MessageChain::new(message),
                        alt_message: Some(raw_message),
                        extra,
                    };
                    Ok(O12MsgEvent::Private(Private(args)))
                }
                MessageEvent::Group(GroupMessage {
                    sub_type,
                    group_id,
                    sender,
                    anonymous,
                    message:
                        Message {
                            message_id,
                            user_id,
                            message_segs,
                            raw_message,
                            font,
                        },
                }) => {
                    let mut message = vec![];
                    for ele in message_segs.into_inner() {
                        message.push(trans_fn(ele).await?);
                    }

                    let sub_type = tool::serde_to_string(sub_type)?;

                    let mut extra = [("ob11.font", font.into_value())].into_map();
                    extra.insert("ob11.sender".into(), serde_value::to_value(sender)?);
                    if let Some(anonymous) = anonymous {
                        extra.insert("ob11.anonymous".into(), serde_value::to_value(anonymous)?);
                    }

                    let args = MessageArgs {
                        self_: compat_self(self_id),
                        message_id: message_id.to_string(),
                        user_id: user_id.to_string(),
                        sub_type,
                        message: MessageChain::new(message),
                        alt_message: Some(raw_message),
                        extra,
                    };
                    Ok(O12MsgEvent::Group(Group {
                        group_id: group_id.to_string(),
                        args,
                    }))
                }
            }
        }
    }
}
