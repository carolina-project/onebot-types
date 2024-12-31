use super::*;
use crate::ob11::message as ob11message;
use crate::ob12::message as ob12message;
use crate::{base::RawMessageSeg, ValueMap};
use ob_types_macro::__data;
use serde::de::IntoDeserializer;
use serde::Deserialize;
use serde_value::*;
use std::future::Future;

/// Represents a file message segment's common fields.
#[__data(default)]
pub struct FileSeg {
    pub file: String,
    pub url: Option<String>,
}

pub trait IntoOB12Seg<P = ()> {
    type Output: TryInto<ob12message::MessageSeg>;

    fn into_ob12(self, param: P) -> CompatResult<Self::Output>;
}

pub trait IntoOB12SegAsync<P: Send = ()> {
    type Output: TryInto<ob12message::MessageSeg>;

    fn into_ob12(self, param: P) -> impl Future<Output = CompatResult<Self::Output>> + Send;
}

pub trait IntoOB11Seg {
    type Output: TryInto<ob11message::MessageSeg>;

    fn into_ob11(self) -> CompatResult<Self::Output>;
}

pub trait IntoOB11SegAsync<P: Send = ()> {
    type Output: TryInto<ob11message::MessageSeg>;

    fn into_ob11(self, param: P) -> impl Future<Output = CompatResult<Self::Output>> + Send;
}

macro_rules! define_compat_types {
    ($($typ:ident $name:literal),* $(,)?) => {
        pub enum CompatSegment {
            $($typ(ob11message::$typ),)*
        }

        $(
            impl IntoOB12Seg for ob11message::$typ {
                type Output = CompatSegment;

                #[inline]
                fn into_ob12(self, _param: ()) -> CompatResult<Self::Output> {
                    Ok(CompatSegment::$typ(self))
                }
            }
        )*

        impl From<CompatSegment> for ob11message::MessageSeg {
            fn from(value: CompatSegment) -> Self {
                match value {
                    $(CompatSegment::$typ(data) => data.into(),)*
                }
            }
        }

        impl CompatSegment {
            /// parse from name and data(ob11 messages that transformed into ob12)
            pub fn parse_data(
                name: impl AsRef<str>, data: ValueMap
            ) -> CompatResult<Self> {
                match name.as_ref() {
                    $(concat!("ob11.", $name) => {
                        Ok(CompatSegment::$typ(ob11message::$typ::deserialize(data.into_deserializer())?))
                    })*
                    _ => Err(CompatError::UnknownCompat(name.as_ref().to_string())),
                }
            }

            pub fn into_data(self) -> Result<(&'static str, ValueMap), CompatError> {
                match self {
                    $(
                        CompatSegment::$typ(data)
                            => Ok((concat!("ob11.", $name),
                                ValueMap::deserialize(serde_value::to_value(data)?)?
                            )),
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
    Music "music",
    Forward "forward",
    Node "node",
    Xml "xml",
    Json "json",
);

impl From<CompatSegment> for ob12message::MessageSeg {
    fn from(value: CompatSegment) -> Self {
        let (r#type, data) = value.into_data().unwrap();
        Self::Other(RawMessageSeg {
            r#type: r#type.to_owned(),
            data,
        })
    }
}

pub mod ob11to12 {
    use std::future::Future;

    use crate::base::ext::IntoValue;
    use serde::ser::Error;

    use super::*;
    use ob12message::*;

    #[inline]
    fn rename_ob11_field(map: ValueMap) -> ValueMap {
        map.into_iter()
            .map(|(k, v)| ("ob11.".to_owned() + &k, v))
            .collect::<ValueMap>()
    }
    fn serialize_into_map<T: serde::Serialize>(value: T) -> CompatResult<ValueMap> {
        let v = serde_value::to_value(value)?;
        if let Value::Map(map) = v {
            map.into_iter()
                .map(|(k, v)| {
                    let Value::String(k) = k else {
                        return Err(serde_value::SerializerError::custom(format!(
                            "expected string key({k:?})"
                        ))
                        .into());
                    };
                    Ok((k, v))
                })
                .collect()
        } else {
            Err(SerializerError::custom(format!("expected map({v:?})")).into())
        }
    }

    pub enum OB12Mention {
        Mention(Mention),
        MentionAll,
    }

    impl From<OB12Mention> for ob12message::MessageSeg {
        fn from(value: OB12Mention) -> Self {
            match value {
                OB12Mention::Mention(m) => MessageSeg::Mention(m),
                OB12Mention::MentionAll => MessageSeg::MentionAll(MentionAll {
                    extra: Default::default(),
                }),
            }
        }
    }

    impl IntoOB12Seg for ob11message::Text {
        type Output = Text;

        fn into_ob12(self, _param: ()) -> CompatResult<Self::Output> {
            Ok(Text {
                text: self.text,
                extra: Default::default(),
            })
        }
    }

    fn extract_file_opt(
        option: Option<ob11message::FileOption>,
        append: impl IntoIterator<Item = (String, Value)>,
    ) -> CompatResult<(Option<String>, ValueMap)> {
        use ob11message::FileOption;
        let (url, mut extra) = if let Some(opt) = option {
            match opt {
                FileOption::Send(send) => (None, serialize_into_map(send)?),
                FileOption::Receive(recv) => (Some(recv.url), Default::default()),
            }
        } else {
            (None, Default::default())
        };
        extra.extend(append);

        Ok((url, rename_ob11_field(extra)))
    }

    impl<F, R> IntoOB12SegAsync<F> for ob11message::Image
    where
        F: (FnOnce(FileSeg) -> R) + Send,
        R: Future<Output = CompatResult<String>> + Send,
    {
        type Output = Image;
        async fn into_ob12(self, trans_fn: F) -> CompatResult<Self::Output> {
            let (url, extra) = extract_file_opt(
                self.option,
                [("type".into(), serde_value::to_value(self.r#type)?)],
            )?;
            let file_id = trans_fn(FileSeg {
                file: self.file,
                url,
            })
            .await?;
            Ok(ob12message::Image { file_id, extra })
        }
    }

    impl<F, R> IntoOB12SegAsync<F> for ob11message::Record
    where
        F: (FnOnce(FileSeg) -> R) + Send,
        R: Future<Output = CompatResult<String>> + Send,
    {
        type Output = Voice;
        async fn into_ob12(self, trans_fn: F) -> CompatResult<Self::Output> {
            let (url, extra) =
                extract_file_opt(self.option, [("magic".into(), self.magic.into_value())])?;
            let file_id = trans_fn(FileSeg {
                file: self.file,
                url,
            })
            .await?;
            Ok(ob12message::Voice { file_id, extra })
        }
    }

    impl<F, R> IntoOB12SegAsync<F> for ob11message::Video
    where
        F: (FnOnce(FileSeg) -> R) + Send,
        R: Future<Output = CompatResult<String>> + Send,
    {
        type Output = Video;
        async fn into_ob12(self, trans_fn: F) -> CompatResult<Self::Output> {
            let (url, extra) = extract_file_opt(self.option, [])?;
            let file_id = trans_fn(FileSeg {
                file: self.file,
                url,
            })
            .await?;
            Ok(ob12message::Video { file_id, extra })
        }
    }

    impl IntoOB12Seg for ob11message::At {
        type Output = OB12Mention;
        fn into_ob12(self, param: ()) -> CompatResult<Self::Output> {
            self.qq.into_ob12(param)
        }
    }

    impl IntoOB12Seg for ob11message::AtTarget {
        type Output = OB12Mention;

        fn into_ob12(self, _param: ()) -> CompatResult<Self::Output> {
            Ok(match self {
                ob11message::AtTarget::QQ(id) => OB12Mention::Mention(ob12message::Mention {
                    user_id: id.to_string(),
                    extra: Default::default(),
                }),
                ob11message::AtTarget::All => OB12Mention::MentionAll,
            })
        }
    }

    impl IntoOB12Seg for ob11message::Location {
        type Output = Location;

        fn into_ob12(self, _param: ()) -> CompatResult<Self::Output> {
            Ok(Location {
                latitude: self.lat,
                longitude: self.lon,
                title: self.title.unwrap_or_else(|| "OneBot 11 Title".into()),
                content: self.content.unwrap_or_else(|| "OneBot 11 Content".into()),
                extra: Default::default(),
            })
        }
    }

    impl IntoOB12Seg<Option<String>> for ob11message::Reply {
        type Output = Reply;

        fn into_ob12(self, param: Option<String>) -> CompatResult<Self::Output> {
            Ok(ob12message::Reply {
                message_id: self.id.to_string(),
                user_id: param,
                extra: Default::default(),
            })
        }
    }
}

pub mod ob12to11 {
    use crate::base::tool;

    use super::*;
    use ob11message::*;
    use serde::de::{Error, IntoDeserializer};

    fn rename_ob12_field(map: ValueMap) -> ValueMap {
        map.into_iter()
            .map(|(k, v)| {
                if let Some(key) = k.strip_suffix("ob11.") {
                    (key.to_owned(), v)
                } else {
                    (k, v)
                }
            })
            .collect::<ValueMap>()
    }

    impl IntoOB11Seg for ob12message::Text {
        type Output = Text;

        fn into_ob11(self) -> CompatResult<Self::Output> {
            Ok(Text { text: self.text })
        }
    }

    impl IntoOB11Seg for ob12message::Mention {
        type Output = At;

        #[inline]
        fn into_ob11(self) -> CompatResult<Self::Output> {
            Ok(At {
                qq: AtTarget::QQ(self.user_id.parse().map_err(DeserializerError::custom)?),
            })
        }
    }

    impl IntoOB11Seg for ob12message::MentionAll {
        type Output = At;

        #[inline]
        fn into_ob11(self) -> CompatResult<Self::Output> {
            Ok(At { qq: AtTarget::All })
        }
    }

    /// get file from cache by file id
    impl<F, R> IntoOB11SegAsync<F> for ob12message::Image
    where
        F: (FnOnce(String) -> R) + Send,
        R: Future<Output = CompatResult<String>> + Send,
    {
        type Output = Image;

        async fn into_ob11(self, trans_fn: F) -> CompatResult<Self::Output> {
            let mut extra = rename_ob12_field(self.extra);
            let img_ty = match extra.remove("type") {
                Some(Value::String(name)) => ImageType::deserialize(Value::String(name))?,
                _ => ImageType::Normal,
            };
            let option = if !extra.is_empty() {
                Some(FileOption::deserialize(extra.into_deserializer())?)
            } else {
                None
            };
            let img = Image {
                file: trans_fn(self.file_id).await?,
                r#type: img_ty,
                option,
            };

            Ok(img)
        }
    }

    impl<F, R> IntoOB11SegAsync<F> for ob12message::Voice
    where
        F: (FnOnce(String) -> R) + Send,
        R: Future<Output = CompatResult<String>> + Send,
    {
        type Output = Record;

        async fn into_ob11(self, trans_fn: F) -> CompatResult<Self::Output> {
            let mut extra = rename_ob12_field(self.extra);
            let magic = match extra.remove("magic") {
                Some(r) => tool::str_bool::deserialize(r)?,
                None => false,
            };
            let option = if !extra.is_empty() {
                Some(FileOption::deserialize(extra.into_deserializer())?)
            } else {
                None
            };
            let img = Record {
                file: trans_fn(self.file_id).await?,
                magic,
                option,
            };

            Ok(img)
        }
    }

    impl<F, R> IntoOB11SegAsync<F> for ob12message::Audio
    where
        F: (FnOnce(String) -> R) + Send,
        R: Future<Output = CompatResult<String>> + Send,
    {
        type Output = Record;

        async fn into_ob11(self, trans_fn: F) -> CompatResult<Self::Output> {
            let mut extra = rename_ob12_field(self.extra);
            let magic = match extra.remove("magic") {
                Some(r) => tool::str_bool::deserialize(r)?,
                None => false,
            };
            let option = if !extra.is_empty() {
                Some(FileOption::deserialize(extra.into_deserializer())?)
            } else {
                None
            };
            let img = Record {
                file: trans_fn(self.file_id).await?,
                magic,
                option,
            };

            Ok(img)
        }
    }

    impl<F, R> IntoOB11SegAsync<F> for ob12message::Video
    where
        F: (FnOnce(String) -> R) + Send,
        R: Future<Output = CompatResult<String>> + Send,
    {
        type Output = Video;

        async fn into_ob11(self, trans_fn: F) -> CompatResult<Self::Output> {
            let extra = rename_ob12_field(self.extra);
            let option = if !extra.is_empty() {
                Some(FileOption::deserialize(extra.into_deserializer())?)
            } else {
                None
            };
            let img = Video {
                file: trans_fn(self.file_id).await?,
                option,
            };

            Ok(img)
        }
    }

    impl IntoOB11Seg for ob12message::Location {
        type Output = Location;

        #[inline]
        fn into_ob11(self) -> CompatResult<Self::Output> {
            Ok(Location {
                lat: self.latitude,
                lon: self.longitude,
                title: Some(self.title),
                content: Some(self.content),
            })
        }
    }

    impl IntoOB11Seg for ob12message::Reply {
        type Output = Reply;

        #[inline]
        fn into_ob11(self) -> CompatResult<Self::Output> {
            Ok(Reply {
                id: self.message_id.parse().map_err(DeserializerError::custom)?,
            })
        }
    }
}
