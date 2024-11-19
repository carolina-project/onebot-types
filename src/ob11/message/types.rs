use std::{fmt::Display, str::FromStr, time::Duration};

use ob_types_macro::json;

use super::MessageSeg;

const fn true_value() -> bool {
    true
}

#[json(str)]
pub struct Text {
    pub text: String,
}

#[json(str)]
pub struct Face {
    pub id: u16,
}

#[json(str)]
pub struct Reply {
    pub id: i32,
}

#[json(str)]
pub struct FileSendOpt {
    #[serde(default = "true_value")]
    pub cache: bool,
    #[serde(default = "true_value")]
    pub proxy: bool,
    #[serde(with = "ob_types_base::tool::duration_str_opt")]
    pub timeout: Option<Duration>,
}

#[json(str)]
pub struct FileRecvOpt {
    #[allow(dead_code)]
    url: String,
}

#[json(str)]
pub enum FileOption {
    Send(FileSendOpt),
    Receive(FileRecvOpt),
}

#[derive(Debug, Clone, Copy)]
pub enum ImageType {
    Flash,
    Normal,
}
impl Default for ImageType {
    #[inline(always)]
    fn default() -> Self {
        Self::Normal
    }
}

mod serde_impl {
    use serde::{Deserialize, Serialize};

    use super::ImageType;

    impl Serialize for ImageType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            match self {
                Self::Flash => serializer.serialize_str("flash"),
                Self::Normal => serializer.serialize_none(),
            }
        }
    }
    impl<'de> Deserialize<'de> for ImageType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            match <&str>::deserialize(deserializer) {
                Ok("flash") => Ok(Self::Flash),
                Ok(_) => Err(serde::de::Error::custom("unknown image type")),
                Err(_) => Ok(Self::default()),
            }
        }
    }
}

#[json(str)]
pub struct Image {
    pub file: String,
    #[serde(default)]
    pub r#type: ImageType,
    #[serde(flatten)]
    pub option: Option<FileOption>,
}
#[json(str)]
pub struct Record {
    pub file: String,
    #[serde(default)]
    pub magic: bool,
    #[serde(flatten)]
    pub option: Option<FileOption>,
}
#[json(str)]
pub struct Video {
    pub file: String,
    #[serde(flatten)]
    pub option: Option<FileOption>,
}

#[json(str)]
pub enum AtTarget {
    All,
    QQ(i64),
}

#[json(str)]
pub struct At {
    pub qq: AtTarget,
}


impl FromStr for AtTarget {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "all" => AtTarget::All,
            qq => AtTarget::QQ(qq.parse()?),
        })
    }
}

impl Display for AtTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AtTarget::All => write!(f, "all"),
            AtTarget::QQ(qq) => write!(f, "{}", qq),
        }
    }
}

/// see [Mirai PokeMessage](https://github.com/mamoe/mirai/blob/f5eefae7ecee84d18a66afce3f89b89fe1584b78/mirai-core/src/commonMain/kotlin/net.mamoe.mirai/message/data/HummerMessage.kt#L49)
#[json(str)]
pub struct Poke {
    pub r#type: i32,
    pub id: i32,
    pub name: Option<String>,
}

#[json(str)]
pub struct Share {
    pub url: String,
    pub title: String,
    pub content: Option<String>,
    pub image: Option<String>,
}

#[json(str)]
pub struct Contact {
    pub r#type: String,
    pub id: u64,
}

#[json(str)]
pub struct Location {
    pub lat: f64,
    pub lon: f64,
    pub title: Option<String>,
    pub content: Option<String>,
}

#[json(str)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Music {
    #[serde(rename = "163")]
    NCM {
        id: u64,
    },
    QQ {
        id: u64,
    },
    XM {
        id: u64,
    },
    Custom {
        url: String,
        audio: String,
        title: String,
        content: Option<String>,
        image: Option<String>,
    },
}

#[json(str)]
pub struct Rps;

#[json(str)]
pub struct Dice;

#[json(str)]
pub struct Shake;

#[json(str)]
pub struct Anonymous;

#[json(str)]
pub struct Forward {
    pub id: String,
}

#[json(str)]
pub struct XML {
    pub data: String,
}

#[json(str)]
pub struct JSON {
    pub data: String,
}

#[json(str)]
#[serde(untagged)]
pub enum ForwardNode {
    Message {
        id: i32,
    },
    Custom {
        user_id: i64,
        nickname: String,
        content: Vec<MessageSeg>,
    },
}
