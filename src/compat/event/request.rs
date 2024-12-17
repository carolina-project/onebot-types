use ob12event::EventDetailed;
use ob_types_macro::__data;
use serde::Deserialize;
use serde_value::SerializerError;

use crate::ob12;

use super::*;

#[__data]
#[serde(tag = "detail_type")]
pub enum CompatRequestKind {
    #[serde(rename = "ob11.friend")]
    Friend(ob11event::request::RequestArgs),
    #[serde(rename = "ob11.group")]
    Group(ob11event::request::GroupRequest),
}

#[__data]
pub struct CompatRequest {
    #[serde(rename = "self")]
    pub self_: ob12::BotSelf,
    #[serde(flatten)]
    pub kind: CompatRequestKind,
}

impl TryFrom<CompatRequest> for EventDetailed {
    type Error = SerializerError;

    fn try_from(value: CompatRequest) -> Result<Self, Self::Error> {
        serde_value::to_value(value)
            .and_then(|r| Deserialize::deserialize(r).map_err(serde::ser::Error::custom))
    }
}

pub mod ob11to12 {
    use crate::compat::compat_self;

    use super::IntoOB12Event;
    use super::*;
    use ob11event::request::*;

    impl IntoOB12Event<String> for RequestEvent {
        type Output = ob12event::RequestEvent;

        fn into_ob12(self, self_id: String) -> SerResult<Self::Output> {
            let compat = match self {
                RequestEvent::Friend(req) => CompatRequestKind::Friend(req),
                RequestEvent::Group(req) => CompatRequestKind::Group(req),
            };
            Ok(ob12event::RequestEvent::Other(
                CompatRequest {
                    self_: compat_self(self_id),
                    kind: compat,
                }
                .try_into()?,
            ))
        }
    }
}
