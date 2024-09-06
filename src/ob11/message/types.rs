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
    Reply(u64),
    /// https://github.com/botuniverse/onebot-11/blob/master/message/segment.md#%E5%90%88%E5%B9%B6%E8%BD%AC%E5%8F%91-
    Forward(u64),
    Node(u64),
}

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
