pub(self) use crate::ob11::message as ob11message;
pub(self) use crate::ob12::message as ob12message;
use serde::Deserialize;

type SerResult<T> = Result<T, serde_value::SerializerError>;
type DesResult<T> = Result<T, serde_value::DeserializerError>;

pub trait IntoOB12<P = ()> {
    type Output: Into<ob12message::MessageSeg>;

    fn into_ob12(self, param: P) -> SerResult<Self::Output>;
}

pub trait IntoOB11 {
    type Output: Into<ob11message::MessageSeg>;

    fn into_ob12(self) -> SerResult<Self::Output>;
}

macro_rules! define_compat_types {
    ($($typ:ident $name:literal),* $(,)?) => {
        pub enum OB12CompatSegment {
            $($typ(ob11message::$typ),)*
        }

        $(
            impl IntoOB12 for ob11message::$typ {
                type Output = OB12CompatSegment;
                fn into_ob12(self, _param: ()) -> SerResult<Self::Output> {
                    Ok(OB12CompatSegment::$typ(self))
                }
            }
        )*



        impl OB12CompatSegment {
            /// parse from name and data(ob11 messages that transformed into ob12)
            pub fn parse_data(
                name: &str, data: serde_value::Value
            ) -> Option<Result<Self, serde_value::DeserializerError>> {
                match name {
                    $(concat!("ob11.", $name) => {
                        Some(ob11message::$typ::deserialize(data).map(OB12CompatSegment::$typ))
                    })*
                    _ => None,
                }
            }

            pub fn into_data(self) -> Result<(&'static str, serde_value::Value), serde_value::SerializerError> {
                match self {
                    $(
                        OB12CompatSegment::$typ(data)
                            => Ok((concat!("ob11.", $name), serde_value::to_value(data)?)),
                    )*
                }
            }

            /// transform ob11 message type name to ob12
            pub fn rename_from_ob11(name: &str) -> Option<&'static str> {
                match name {
                    $($name => Some(concat!("ob11.", $name)),)*
                    _ => None,
                }
            }

            /// check if the ob12 type name is convertible to ob11
            pub fn is_convertible(name: &str) -> bool {
                match name {
                    $($name)|* => true,
                    _ => false,
                }
            }
        }
    };
}

define_compat_types! (
    Face "face",
    Dice "dice",
    Rps "rps",
    Shake "shake",
    Poke "poke",
    Anonymous "anonymous",
    Share "share",
    Contact "contact",
    Location "location",
    Music "music",
    Forward "forward",
    ForwardNode "node",
    XML "xml",
    JSON "json",
);

impl From<OB12CompatSegment> for ob12message::MessageSeg {
    fn from(value: OB12CompatSegment) -> Self {
        let (r#type, data) = value.into_data().unwrap();
        Self::Other {
            r#type: r#type.into(),
            data,
        }
    }
}

pub mod ob11to12 {

    use crate::ValueMap;
    use serde::{ser::Error, Serialize};
    use serde_value::Value;

    use super::*;

    pub enum OB12Mention {
        Mention(ob12message::Mention),
        MentionAll,
    }

    impl From<OB12Mention> for ob12message::MessageSeg {
        fn from(value: OB12Mention) -> Self {
            match value {
                OB12Mention::Mention(m) => ob12message::MessageSeg::Mention(m),
                OB12Mention::MentionAll => {
                    ob12message::MessageSeg::MentionAll(ob12message::MentionAll {
                        extra: default_obj(),
                    })
                }
            }
        }
    }

    #[inline(always)]
    fn default_obj() -> Value {
        Value::Map(Default::default())
    }

    #[inline]
    fn to_map<T: Serialize>(value: T) -> SerResult<ValueMap> {
        match serde_value::to_value(value)? {
            Value::Map(map) => Ok(map),
            _ => Err(serde_value::SerializerError::custom(
                "invalid value, expected map",
            )),
        }
    }

    #[inline]
    fn remove_field(map: &mut ValueMap, key: &str) -> SerResult<Value> {
        map.remove(&Value::String(key.into()))
            .ok_or_else(|| serde_value::SerializerError::custom(format!("missing field {}", key)))
    }

    #[inline]
    fn rename_ob11_field(map: ValueMap) -> Value {
        Value::Map(
            map.into_iter()
                .filter_map(|(k, v)| {
                    if let Value::String(k) = k {
                        Some((Value::String("ob11.".to_owned() + &k), v))
                    } else {
                        None
                    }
                })
                .collect::<ValueMap>(),
        )
    }

    impl IntoOB12 for ob11message::Text {
        type Output = ob12message::Text;

        fn into_ob12(self, _param: ()) -> SerResult<Self::Output> {
            Ok(ob12message::Text {
                text: self.text,
                extra: default_obj(),
            })
        }
    }

    impl IntoOB12 for ob11message::Image {
        type Output = ob12message::Image;
        fn into_ob12(self, _param: ()) -> SerResult<Self::Output> {
            let mut value = to_map(self)?;
            let Value::String(file_id) = remove_field(&mut value, "file")? else {
                return Err(serde_value::SerializerError::custom("missing field file"));
            };
            Ok(ob12message::Image {
                file_id,
                extra: rename_ob11_field(value),
            })
        }
    }

    impl IntoOB12 for ob11message::Record {
        type Output = ob12message::Voice;
        fn into_ob12(self, _param: ()) -> SerResult<Self::Output> {
            let mut value = to_map(self)?;
            let Value::String(file_id) = remove_field(&mut value, "file")? else {
                return Err(serde_value::SerializerError::custom("missing field file"));
            };
            Ok(ob12message::Voice {
                file_id,
                extra: rename_ob11_field(value),
            })
        }
    }

    impl IntoOB12 for ob11message::Video {
        type Output = ob12message::Video;
        fn into_ob12(self, _param: ()) -> SerResult<Self::Output> {
            let mut value = to_map(self)?;
            let Value::String(file_id) = remove_field(&mut value, "file")? else {
                return Err(serde_value::SerializerError::custom("missing field file"));
            };
            Ok(ob12message::Video {
                file_id,
                extra: rename_ob11_field(value),
            })
        }
    }

    impl IntoOB12 for ob11message::AtTarget {
        type Output = OB12Mention;

        fn into_ob12(self, _param: ()) -> SerResult<Self::Output> {
            Ok(match self {
                ob11message::AtTarget::QQ(id) => OB12Mention::Mention(ob12message::Mention {
                    user_id: id.to_string(),
                    extra: default_obj(),
                }),
                ob11message::AtTarget::All => OB12Mention::MentionAll,
            })
        }
    }

    impl IntoOB12<Option<String>> for ob11message::Reply {
        type Output = ob12message::Reply;

        fn into_ob12(self, param: Option<String>) -> SerResult<Self::Output> {
            Ok(ob12message::Reply {
                message_id: self.id.to_string(),
                user_id: param,
                extra: default_obj(),
            })
        }
    }
}

pub mod ob12to11 {
    use crate::ob11::message as ob11message;
    use crate::ob12::message as ob12message;

    use super::*;

    impl IntoOB11 for ob12message::Audio {
        type Output = ob11message::Record;
        fn into_ob12(self) -> SerResult<Self::Output> {
            
        }
    }
}
