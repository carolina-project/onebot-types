use super::*;

pub mod ob11to12 {
    use std::error::Error;

    use crate::compat::compat_self;

    use super::*;
    use crate::ob11::{message::MessageChain, MessageSeg};
    use crate::ob12;
    use eyre::Result;
    use ob11event::message::*;
    use ob_types_base::ext::{IntoValue, ValueExt};
    use serde_value::{SerializerError, Value};

    impl<F> IntoOB12Event<(String, F)> for ob11event::MessageEvent
    where
        F: Fn(MessageSeg) -> Result<ob12::MessageSeg, SerializerError>,
    {
        type Output = ob12event::MessageEvent;

        fn into_ob12(self, param: (String, F)) -> SerResult<Self::Output> {
            let (user_id, trans_fn) = param;
            let Message {
                message_id,
                user_id,
                message_segs,
                raw_message,
                font,
            } = self.message;
            let message = match message_segs.0 {
                MessageChain::Array(segs) => segs
                    .into_iter()
                    .map(trans_fn)
                    .collect::<Result<Vec<_>, SerializerError>>()?,
                MessageChain::String(_) => unimplemented!("cq code string"),
            };
            ob12event::MessageEvent {
                self_: compat_self(param),
                message_id: message_id.into(),
                sub_type: Default::default(),
                message,
                alt_message: Some(raw_message),
                source: todo!(),
                extra: Value::from_map([("ob11.font".into(), user_id.into_value())].into()),
            }
        }
    }
}
