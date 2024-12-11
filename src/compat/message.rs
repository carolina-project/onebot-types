use super::*;
pub(self) use crate::ob11::message as ob11message;
pub(self) use crate::ob12::message as ob12message;
use crate::ValueMap;
use serde::Deserialize;
pub(self) use serde_value::*;
use std::future::Future;

pub(self) use crate::{DesResult, SerResult};

pub trait IntoOB12Seg<P = ()> {
    type Output: TryInto<ob12message::MessageSeg>;

    fn into_ob12(self, param: P) -> SerResult<Self::Output>;
}

pub trait IntoOB12SegAsync<P = ()> {
    type Output: TryInto<ob12message::MessageSeg>;

    fn into_ob12(self, param: P) -> impl Future<Output = SerResult<Self::Output>>;
}

pub trait IntoOB11Seg {
    type Output: TryInto<ob11message::MessageSeg>;

    fn into_ob11(self) -> DesResult<Self::Output>;
}

pub trait IntoOB11SegAsync<P> {
    type Output: TryInto<ob11message::MessageSeg>;

    fn into_ob11(self, param: P) -> impl Future<Output = DesResult<Self::Output>>;
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
                fn into_ob12(self, _param: ()) -> SerResult<Self::Output> {
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
                name: impl AsRef<str>, data: serde_value::Value
            ) -> Result<Self, CompatError> {
                match name.as_ref() {
                    $(concat!("ob11.", $name) => {
                        Ok(CompatSegment::$typ(ob11message::$typ::deserialize(data)?))
                    })*
                    _ => Err(CompatError::UnknownCompat(name.as_ref().to_string())),
                }
            }

            pub fn into_data(self) -> Result<(&'static str, serde_value::Value), CompatError> {
                match self {
                    $(
                        CompatSegment::$typ(data)
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
    Music "music",
    Forward "forward",
    Node "node",
    Xml "xml",
    Json "json",
);

impl From<CompatSegment> for ob12message::MessageSeg {
    fn from(value: CompatSegment) -> Self {
        let (r#type, data) = value.into_data().unwrap();
        Self::Other {
            r#type: r#type.into(),
            data,
        }
    }
}

pub mod ob11to12 {
    use std::future::Future;

    use ob_types_base::ext::{IntoValue, ValueMapExt};

    use super::*;
    use ob12message::*;

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

        fn into_ob12(self, _param: ()) -> SerResult<Self::Output> {
            Ok(Text {
                text: self.text,
                extra: Default::default(),
            })
        }
    }

    impl<F, R> IntoOB12SegAsync<F> for ob11message::Image
    where
        F: FnOnce(ob11message::Image) -> R,
        R: Future<Output = SerResult<String>>,
    {
        type Output = Image;
        async fn into_ob12(self, trans_fn: F) -> SerResult<Self::Output> {
            let img_ty = self.r#type;
            Ok(ob12message::Image {
                file_id: trans_fn(self).await?,
                extra: [("ob11.type", serde_value::to_value(img_ty)?)].into_map(),
            })
        }
    }

    impl<F, R> IntoOB12SegAsync<F> for ob11message::Record
    where
        F: FnOnce(ob11message::Record) -> R,
        R: Future<Output = SerResult<String>>,
    {
        type Output = Voice;
        async fn into_ob12(self, trans_fn: F) -> SerResult<Self::Output> {
            let magic = self.magic;
            Ok(ob12message::Voice {
                file_id: trans_fn(self).await?,
                extra: [("ob11.magic", magic.into_value())].into_map(),
            })
        }
    }

    impl<F, R> IntoOB12SegAsync<F> for ob11message::Video
    where
        F: FnOnce(ob11message::Video) -> R,
        R: Future<Output = SerResult<String>>,
    {
        type Output = Video;
        async fn into_ob12(self, trans_fn: F) -> SerResult<Self::Output> {
            Ok(ob12message::Video {
                file_id: trans_fn(self).await?,
                extra: Default::default(),
            })
        }
    }

    impl IntoOB12Seg for ob11message::At {
        type Output = OB12Mention;
        fn into_ob12(self, param: ()) -> SerResult<Self::Output> {
            self.qq.into_ob12(param)
        }
    }

    impl IntoOB12Seg for ob11message::AtTarget {
        type Output = OB12Mention;

        fn into_ob12(self, _param: ()) -> SerResult<Self::Output> {
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

        fn into_ob12(self, _param: ()) -> SerResult<Self::Output> {
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

        fn into_ob12(self, param: Option<String>) -> SerResult<Self::Output> {
            Ok(ob12message::Reply {
                message_id: self.id.to_string(),
                user_id: param,
                extra: Default::default(),
            })
        }
    }
}

pub mod ob12to11 {
    use super::*;
    use ob11message::*;
    use serde::de::Error;

    #[inline]
    #[allow(unused)]
    fn remove_field<'a, T: Deserialize<'a>>(map: &mut ValueMap, key: &str) -> DesResult<T> {
        map.remove(key)
            .ok_or_else(|| serde_value::DeserializerError::custom(format!("missing field {}", key)))
            .and_then(|r| T::deserialize(r))
    }

    fn rename_ob12_field(map: ValueMap) -> ValueMap {
        map.into_iter()
            .map(|(k, v)| {
                if k.starts_with("ob11.") {
                    (k[5..].to_owned(), v)
                } else {
                    (k, v)
                }
            })
            .collect::<ValueMap>()
    }

    impl IntoOB11Seg for ob12message::Text {
        type Output = Text;

        fn into_ob11(self) -> DesResult<Self::Output> {
            Ok(Text { text: self.text })
        }
    }

    impl IntoOB11Seg for ob12message::Mention {
        type Output = At;

        #[inline]
        fn into_ob11(self) -> DesResult<Self::Output> {
            Ok(At {
                qq: AtTarget::QQ(self.user_id.parse().map_err(DeserializerError::custom)?),
            })
        }
    }

    impl IntoOB11Seg for ob12message::MentionAll {
        type Output = At;

        #[inline]
        fn into_ob11(self) -> DesResult<Self::Output> {
            Ok(At { qq: AtTarget::All })
        }
    }

    /// get file from cache by file id
    impl<F, R> IntoOB11SegAsync<F> for ob12message::Image
    where
        F: FnOnce(String) -> R,
        R: Future<Output = DesResult<Image>>,
    {
        type Output = Image;

        async fn into_ob11(self, trans_fn: F) -> DesResult<Self::Output> {
            let mut extra = rename_ob12_field(self.extra);
            let mut img = trans_fn(self.file_id).await?;
            if let Some(field) = extra.remove("type") {
                img.r#type = ImageType::deserialize(field)?;
            }

            Ok(img)
        }
    }

    impl<F, R> IntoOB11SegAsync<F> for ob12message::Voice
    where
        F: FnOnce(String) -> R,
        R: Future<Output = DesResult<Record>>,
    {
        type Output = Record;

        async fn into_ob11(self, trans_fn: F) -> DesResult<Self::Output> {
            let mut extra = rename_ob12_field(self.extra);
            let mut record = trans_fn(self.file_id).await?;
            if let Some(field) = extra.remove("magic") {
                record.magic = bool::deserialize(field)?;
            }

            Ok(record)
        }
    }

    impl<F, R> IntoOB11SegAsync<F> for ob12message::Video
    where
        F: FnOnce(String) -> R,
        R: Future<Output = DesResult<Video>>,
    {
        type Output = Video;

        async fn into_ob11(self, trans_fn: F) -> DesResult<Self::Output> {
            Ok(trans_fn(self.file_id).await?)
        }
    }

    impl IntoOB11Seg for ob12message::Location {
        type Output = Location;

        #[inline]
        fn into_ob11(self) -> DesResult<Self::Output> {
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
        fn into_ob11(self) -> DesResult<Self::Output> {
            Ok(Reply {
                id: self.message_id.parse().map_err(DeserializerError::custom)?,
            })
        }
    }
}
