use std::time::Duration;

use super::*;

use crate::compat::default_obj;

impl<F, E> IntoOB11Action<F> for ob12action::SendMessage
where
    F: Fn(ob12::MessageSeg) -> Result<ob11::MessageSeg, E>,
    E: std::error::Error,
{
    type Output = ob11action::SendMsg;

    fn into_ob11(self, msg_trans_fn: F) -> DesResult<Self::Output> {
        let message: Vec<_> = {
            let ob12::message::MessageChain::Array(arr) = self.message else {
                unimplemented!("cq code string")
            };

            arr.into_iter()
                .map(|e| msg_trans_fn(e).map_err(DeError::custom))
                .collect::<Result<_, DeserializerError>>()?
        };

        Ok(ob11action::SendMsg {
            target: self.target.try_into().map_err(DeserializerError::custom)?,
            message: ob11::message::MessageChain::Array(message),
        })
    }
}

impl IntoOB11Action for ob12action::DeleteMessage {
    type Output = ob11action::DeleteMsg;
    fn into_ob11(self, _: ()) -> DesResult<Self::Output> {
        Ok(ob11action::DeleteMsg {
            message_id: self.message_id.parse().map_err(DeserializerError::custom)?,
        })
    }
}

impl FromOB11Resp<Duration> for ob12action::SendMessageResp {
    type In = ob11action::MessageResp;

    fn from_ob11(from: Self::In, time: Duration) -> DesResult<Self> {
        Ok(Self {
            message_id: from.message_id.to_string(),
            time,
            extra: default_obj(),
        })
    }
}
