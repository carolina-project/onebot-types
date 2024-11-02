use std::{fmt::Display, str::FromStr, time::Duration};

use ob_types_macro::json_from_str;

use super::MessageSeg;

#[cfg(feature = "json")]
const fn true_value() -> bool {
    true
}

#[json_from_str]
pub struct FileSendOpt {
    #[cfg_attr(feature = "json", serde(default = "true_value"))]
    pub cache: bool,
    #[cfg_attr(feature = "json", serde(default = "true_value"))]
    pub proxy: bool,
    #[cfg_attr(
        feature = "json",
        serde(with = "ob_types_base::tool::duration_str_opt")
    )]
    pub timeout: Option<Duration>,
}

#[json_from_str]
pub struct FileRecvOpt {
    #[allow(dead_code)]
    url: String,
}

#[json_from_str(serde(untagged))]
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
    fn default() -> Self {
        Self::Normal
    }
}

#[cfg(feature = "json")]
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
                Err(e) => Err(e),
            }
        }
    }
}

#[json_from_str]
pub struct Image {
    pub file: String,
    #[cfg_attr(feature = "json", serde(default))]
    pub r#type: ImageType,
    #[cfg_attr(feature = "json", serde(flatten))]
    pub option: Option<FileOption>,
}
#[json_from_str]
pub struct Record {
    pub file: String,
    #[cfg_attr(feature = "json", serde(default))]
    pub magic: bool,
    #[cfg_attr(feature = "json", serde(flatten))]
    pub option: Option<FileOption>,
}
#[json_from_str]
pub struct Video {
    pub file: String,
    #[cfg_attr(feature = "json", serde(flatten))]
    pub option: Option<FileOption>,
}

#[json_from_str]
pub enum AtTarget {
    All,
    QQ(i64),
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
#[json_from_str]
pub struct Poke {
    pub r#type: i32,
    pub id: i32,
    pub name: Option<String>,
}

#[json_from_str]
pub struct Share {
    pub url: String,
    pub title: String,
    pub content: Option<String>,
    pub image: Option<String>,
}

#[json_from_str]
pub struct Contact {
    pub r#type: String,
    pub id: u64,
}

#[json_from_str]
pub struct Location {
    pub lat: f64,
    pub lon: f64,
    pub title: Option<String>,
    pub content: Option<String>,
}

#[json_from_str(serde(tag = "type", rename_all="lowercase"))]
pub enum Music {
    #[cfg_attr(feature = "json", serde(rename = "163"))]
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

#[json_from_str(serde(untagged))]
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
