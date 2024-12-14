mod types;

use ob_types_macro::__data;
use serde::Deserialize;
use serde_value::{DeserializerError, SerializerError, Value};

pub use types::*;

use crate::base::RawMessageSeg;

macro_rules! message_segs {
    ($($(#[$meta:meta])* $typ:ident $($doc:literal)?),* $(,)?) => {
        #[__data]
        #[serde(tag = "type", content = "data", rename_all = "snake_case")]
        pub enum MessageSeg {
            $(
            $(#[doc = $doc])?
            $(#[$meta])*
            $typ($typ),
            )*
        }

        $(impl From<$typ> for MessageSeg {
            fn from(sg: $typ) -> Self {
                Self::$typ(sg)
            }
        })*
    };
}

message_segs! {
    Text,
    Face "see [表情 CQ 码 ID 表](https://github.com/kyubotics/coolq-http-api/wiki/%E8%A1%A8%E6%83%85-CQ-%E7%A0%81-ID-%E8%A1%A8)",
    Image,
    Record,
    Video,
    At,
    Rps,
    Dice,
    Shake,
    Poke,
    Anonymous,
    Share,
    Contact,
    Location,
    Music,
    Reply,
    Forward,
    Node,
    Xml,
    Json,
}

impl TryFrom<RawMessageSeg> for MessageSeg {
    type Error = DeserializerError;

    fn try_from(seg: RawMessageSeg) -> Result<Self, Self::Error> {
        let RawMessageSeg { r#type, data } = seg;
        Deserialize::deserialize(Value::from_map(
            [
                ("type", r#type.into_value()),
                ("data", Value::from_map(data)),
            ]
            .into(),
        ))
    }
}

impl TryFrom<MessageSeg> for RawMessageSeg {
    type Error = SerializerError;

    fn try_from(seg: MessageSeg) -> Result<Self, Self::Error> {
        use serde::ser::Error;
        Ok(Self::deserialize(serde_value::to_value(seg)?).map_err(Error::custom)?)
    }
}

#[__data]
#[serde(untagged)]
pub enum MessageChain {
    Array(Vec<RawMessageSeg>),
    /// DO NOT USE, CQ code has not been implemented yet
    String(String),
}
impl Default for MessageChain {
    fn default() -> Self {
        Self::Array(vec![])
    }
}

impl MessageChain {
    #[allow(unused)]
    pub fn into_messages(self) -> Vec<RawMessageSeg> {
        match self {
            Self::Array(s) => s,
            Self::String(_) => unimplemented!("cq code string"),
        }
    }

    pub fn append<T: TryInto<RawMessageSeg>>(self, segs: Vec<T>) -> Result<Self, T::Error> {
        match self {
            MessageChain::Array(mut arr) => {
                arr.extend(
                    segs.into_iter()
                        .map(|r| r.try_into())
                        .collect::<Result<Vec<_>, _>>()?,
                );
                Ok(Self::Array(arr))
            }
            MessageChain::String(_) => unimplemented!("cq code string"),
        }
    }
}
