pub mod types;

use crate::cross::Data;
use types::*;

#[derive(Clone, Debug)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct OB11MessageSegRaw {
    pub r#type: String,
    pub data: Data,
}

pub enum MessageSeg {
    /// text message, contains text
    Text(String),
    /// see [表情 CQ 码 ID 表](https://github.com/kyubotics/coolq-http-api/wiki/%E8%A1%A8%E6%83%85-CQ-%E7%A0%81-ID-%E8%A1%A8)
    Face(u16),
    Image(Image),
    Record(Record),
    Video(Video),
    At(AtTarget),
    Rps,
    Dice,
    Shake,
    Poke(Poke),
    Anonymous,
    Share(Share),
    Contact(Contact),
    Location(Location),
    Music(Music),
    /// represents reply message by message id
    Reply(u32),
    /// https://github.com/botuniverse/onebot-11/blob/master/message/segment.md#%E5%90%88%E5%B9%B6%E8%BD%AC%E5%8F%91-
    Forward(u64),
    ForwardNode(ForwardNode),
    XML(String),
    JSON(String),
}
