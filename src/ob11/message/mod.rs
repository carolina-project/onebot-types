mod types;

use serde_value::Value;
use ob_types_macro::json;

#[allow(unused)]
use std::{fmt::Display, str::FromStr};
pub use types::*;

#[json]
pub struct MessageSegRaw {
    pub r#type: String,
    pub data: Value,
}

#[json]
pub enum MessageSeg {
    Text(Text),
    /// see [表情 CQ 码 ID 表](https://github.com/kyubotics/coolq-http-api/wiki/%E8%A1%A8%E6%83%85-CQ-%E7%A0%81-ID-%E8%A1%A8)
    Face(Face),
    Image(Image) ,
    Record(Record) ,
    Video(Video) ,
    At(At),
    Rps(Rps) ,
    Dice(Dice) ,
    Shake(Shake) ,
    Poke(Poke) ,
    Anonymous(Anonymous) ,
    Share(Share) ,
    Contact(Contact) ,
    Location(Location) ,
    Music(Music),
    Reply(Reply),
    Forward(Forward) ,
    ForwardNode(ForwardNode) ,
    XML(XML) ,
    JSON(JSON),
    #[serde(untagged)]
    Custom(MessageSegRaw),
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
