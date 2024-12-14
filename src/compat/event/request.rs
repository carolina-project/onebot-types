use ob_types_base::ext::IntoValue;
use ob_types_macro::data;
use serde::Deserialize;
use serde_value::Value;

use crate::DesResult;

use super::*;

#[data]
#[serde(tag = "type")]
pub enum CompatRequestKind {
    #[serde(rename = "ob11.friend")]
    Friend,
    #[serde(rename = "ob11.group")]
    Group(ob11event::request::AddGroup),
}

#[data]
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

    use super::IntoOB12Event;
    use super::*;
    use ob11event::request::*;
    use ob12event::request;
    use ob_types_base::ext::IntoValue;
    use serde::ser::Error;
    use serde_value::Value;

    impl IntoOB12Event<String> for RequestEvent {
        type Output = ob12event::EventType;

        fn into_ob12(self, param: String) -> SerResult<Self::Output> {
            let RequestEvent {
                user_id,
                kind,
                comment,
                flag,
            } = self;
            let Value::Map(mut data) = serde_value::to_value(CompatRequestEvent {
                user_id: user_id.to_string(),
                kind: match kind {
                    RequestKind::Friend => CompatRequestKind::Friend,
                    RequestKind::Group(add) => CompatRequestKind::Group(add),
                    RequestKind::Other(_) => {
                        return Err(serde_value::SerializerError::custom(
                            "Unsupported request kind",
                        ))
                    }
                },
                comment: Some(comment),
                flag: Some(flag),
            })?
            else {
                return Err(serde_value::SerializerError::custom("Expected a map"));
            };
            let Value::String(detail_type) = data
                .remove(&"type".into_value())
                .ok_or_else(|| serde_value::SerializerError::custom("Missing type field"))?
            else {
                return Err(serde_value::SerializerError::custom("Expected a string"));
            };
            Ok(ob12event::EventType::Request(ob12event::RequestEvent {
                self_: compat_self(param),
                sub_type: Default::default(),
                kind: request::RequestKind::Other {
                    detail_type,
                    data: data.into_value(),
                },
            }))
        }
    }
}
