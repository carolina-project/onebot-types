use ob_types_macro::__data;
use serde::Deserialize;
use serde_value::Value;

use crate::{base::ext::IntoValue, DesResult};

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
pub struct CompatRequestEvent {
    pub user_id: String,
    #[serde(flatten)]
    pub kind: CompatRequestKind,
    pub comment: Option<String>,
    pub flag: Option<String>,
}

impl CompatRequestEvent {
    pub fn parse_data(type_name: impl AsRef<str>, data: Value) -> DesResult<Self> {
        if let Value::Map(mut data) = data {
            data.insert("type".into_value(), type_name.as_ref().into_value());
            CompatRequestEvent::deserialize(Value::Map(data))
        } else {
            Err(serde::de::Error::custom("Invalid data format"))
        }
    }
}

pub mod ob11to12 {
    use crate::compat::compat_self;
    use crate::ValueMap;

    use super::IntoOB12Event;
    use super::*;
    use ob11event::request::*;
    use ob12event::EventDetailed;
    use serde::ser::Error;
    use serde_value::SerializerError;

    impl IntoOB12Event<String> for RequestEvent {
        type Output = ob12event::EventKind;

        fn into_ob12(self, self_id: String) -> SerResult<Self::Output> {
            #[derive(Deserialize)]
            struct Helper {
                detail_type: String,
                #[serde(flatten)]
                detail: ValueMap,
            }

            let compat = match self {
                RequestEvent::Friend(req) => CompatRequestKind::Friend(req),
                RequestEvent::Group(req) => CompatRequestKind::Group(req),
            };
            let Helper {
                detail_type,
                mut detail,
            } = Helper::deserialize(serde_value::to_value(compat)?)
                .map_err(SerializerError::custom)?;
            detail.insert("self".into(), serde_value::to_value(compat_self(self_id))?);
            let request = EventDetailed {
                detail_type,
                detail,
            }
            .into();
            Ok(ob12event::EventKind::Request(request))
        }
    }
}
