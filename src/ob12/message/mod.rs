mod types;
use ob_types_macro::__data;
use serde::Deserialize;
use serde_value::{DeserializerError, SerializerError, Value};
pub use types::*;

use crate::base::{ext::{IntoValue, ValueExt}, RawMessageSeg};

#[__data]
#[serde(untagged)]
pub enum MessageChain {
    Array(Vec<RawMessageSeg>),
    String(String),
}

macro_rules! message_seg {
    ($($sg:ident),* $(,)?) => {
        #[__data]
        #[serde(rename_all = "snake_case", tag = "type", content = "data")]
        pub enum MessageSeg {
            $($sg($sg),)*
            #[serde(untagged)]
            /// Extra message types or messages which missing fields.
            Other {
                r#type: String,
                data: serde_value::Value,
            }
        }

        $(
            impl From<$sg> for MessageSeg {
                fn from(sg: $sg) -> Self {
                    Self::$sg(sg)
                }
            }
        )*
    };
}

message_seg!(Text, Mention, MentionAll, Location, Reply, Image, Voice, Audio, Video, File);

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
