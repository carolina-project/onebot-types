use super::*;

pub mod ob11to12 {

    use crate::compat::compat_self;

    use super::*;
    use crate::ob11::{message::MessageChain, MessageSeg};
    use crate::ob12::{self, message};
    use ob11event::message::*;
    use ob_types_base::ext::{IntoValue, ValueMapExt};
    use ob_types_base::tool;
    use serde_value::SerializerError;

    /// Converts an OB11 MessageEvent into an OB12 MessageEvent using the provided transformation function(transform message segment).
    impl<F, R> IntoOB12EventAsync<(String, F)> for ob11event::MessageEvent
    where
        F: Fn(MessageSeg) -> R,
        R: Future<Output = Result<ob12::MessageSeg, SerializerError>>,
    {
        type Output = ob12event::EventType;

        async fn into_ob12(self, param: (String, F)) -> SerResult<Self::Output> {
            let (self_id, trans_fn) = param;
            let Message {
                message_id,
                user_id,
                message_segs,
                raw_message,
                font,
            } = self.message;
            let message = match message_segs.0 {
                MessageChain::Array(segs) => {
                    let mut transformed = vec![];
                    for ele in segs {
                        transformed.push(trans_fn(ele).await?);
                    }
                    transformed
                }
                MessageChain::String(_) => unimplemented!("cq code string"),
            };

            let mut extra = [("ob11.font", font.into_value())].into_map();
            let (sub_type, source);
            match self.kind {
                MessageKind::Private(private) => {
                    sub_type = tool::serde_to_string(private.sub_type)?;
                    source = ob12::ChatTarget::Private {
                        user_id: user_id.to_string(),
                    };
                    extra.insert("ob11.sender".into(), serde_value::to_value(private.sender)?);
                }
                MessageKind::Group(group) => {
                    sub_type = tool::serde_to_string(group.sub_type)?;
                    source = ob12::ChatTarget::Group {
                        group_id: group.group_id.to_string(),
                        user_id: Some(user_id.to_string()),
                    };
                    extra.insert("ob11.sender".into(), serde_value::to_value(group.sender)?);
                    if let Some(anonymous) = group.anonymous {
                        extra.insert("ob11.anonymous".into(), serde_value::to_value(anonymous)?);
                    }
                }
            }
            Ok(ob12event::EventType::Message(ob12event::MessageEvent {
                self_: compat_self(self_id.to_string()),
                message_id: message_id.to_string(),
                sub_type,
                message: message::MessageChain::Array(message),
                alt_message: Some(raw_message),
                source,
                extra,
            }))
        }
    }
}
