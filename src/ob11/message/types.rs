use std::time::Duration;


use ob_types_macro::json;

use super::MessageSeg;

#[json]
pub enum FileOption {
    Send {
        cache: Option<bool>,
        proxy: Option<bool>,
        timeout: Option<Duration>,
    },
    Receive {
        url: String,
    },
}

#[json]
pub struct Image {
    pub file: String,
    pub is_flash: bool,
    pub option: FileOption,
}
#[json]
pub struct Record {
    pub file: String,
    pub magic: bool,
    pub option: FileOption,
}
#[json]
pub struct Video {
    pub file: String,
    pub option: FileOption,
}

#[json]
pub enum AtTarget {
    All,
    QQ(u64),
}

/// see [Mirai PokeMessage](https://github.com/mamoe/mirai/blob/f5eefae7ecee84d18a66afce3f89b89fe1584b78/mirai-core/src/commonMain/kotlin/net.mamoe.mirai/message/data/HummerMessage.kt#L49)
#[json]
pub struct Poke {
    pub r#type: i32,
    pub id: i32,
    pub name: String,
}

#[json]
pub struct Share {
    pub url: String,
    pub title: String,
    pub content: Option<String>,
    pub image: Option<String>,
}

#[json]
pub struct Contact {
    pub r#type: String,
    pub id: u64,
}

#[json]
pub struct Location {
    pub lat: f64,
    pub lon: f64,
    pub title: Option<String>,
    pub content: Option<String>,
}

#[json]
pub enum Music {
    Default {
        r#type: MusicType,
        id: u64,
    },
    Custom {
        r#type: String,
        url: String,
        audio: String,
        title: String,
        content: Option<String>,
        image: Option<String>,
    },
}
#[json]
pub enum MusicType {
    NCM,
    QQ,
    XM,
}

#[json]
pub enum ForwardNode {
    Message {
        id: u32,
    },
    Custom {
        user_id: u64,
        nickname: String,
        content: Vec<MessageSeg>,
    },
}

