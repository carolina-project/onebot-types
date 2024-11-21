mod types;

use ob_types_macro::json;
use serde_value::Value;

#[allow(unused)]
use std::{fmt::Display, str::FromStr};
pub use types::*;

#[json]
pub struct MessageSegRaw {
    pub r#type: String,
    pub data: Value,
}

macro_rules! message_segs {
    ($($typ:ident $($doc:literal)?),* $(,)?) => {
        #[json]
        pub enum MessageSeg {
            $(
            $(#[doc = $doc])?
            $typ($typ),
            )*
            #[serde(untagged)]
            Other(MessageSegRaw),
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
    ForwardNode,
    XML,
    JSON,
}

#[json]
#[serde(untagged)]
pub enum MessageChain {
    Array(Vec<MessageSeg>),
    String(String),
}

impl MessageChain {
    #[allow(unused)]
    fn into_messages(self) -> Vec<MessageSeg> {
        match self {
            Self::Array(s) => s,
            Self::String(_) => unimplemented!("cq code string"),
        }
    }
}
