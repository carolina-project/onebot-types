use super::MessageSeg;

pub enum FileOption {
    Send {
        cache: Option<bool>,
        proxy: Option<bool>,
        timeout: Option<f64>,
    },
    Receive {
        url: String,
    },
}

pub struct Image {
    pub file: String,
    pub is_flash: bool,
    pub option: FileOption,
}
pub struct Record {
    pub file: String,
    pub magic: bool,
    pub option: FileOption,
}
pub struct Video {
    pub file: String,
    pub option: FileOption,
}

pub enum AtTarget {
    All,
    QQ(u64),
}

/// see [Mirai PokeMessage](https://github.com/mamoe/mirai/blob/f5eefae7ecee84d18a66afce3f89b89fe1584b78/mirai-core/src/commonMain/kotlin/net.mamoe.mirai/message/data/HummerMessage.kt#L49)
pub struct Poke {
    pub r#type: i32,
    pub id: i32,
    pub name: String,
}

pub struct Share {
    pub url: String,
    pub title: String,
    pub content: Option<String>,
    pub image: Option<String>,
}

pub struct Contact {
    pub r#type: String,
    pub id: u64,
}

pub struct Location {
    pub lat: f64,
    pub lon: f64,
    pub title: Option<String>,
    pub content: Option<String>,
}

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
pub enum MusicType {
    NCM,
    QQ,
    XM,
}

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
