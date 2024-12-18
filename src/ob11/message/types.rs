use std::{collections::BTreeMap, fmt::Display, str::FromStr, time::Duration};

use ob_types_macro::__data;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::base::ext::ValueExt;

use super::MessageSeg;

const fn true_value() -> bool {
    true
}

#[__data(str)]
pub struct Text {
    pub text: String,
}

#[__data(str)]
pub struct Face {
    pub id: u16,
}

#[__data(str)]
pub struct Reply {
    pub id: i32,
}

#[__data(str)]
pub struct FileSendOpt {
    #[serde(default = "true_value")]
    pub cache: bool,
    #[serde(default = "true_value")]
    pub proxy: bool,
    #[serde(with = "crate::base::tool::duration_str_opt", default)]
    pub timeout: Option<Duration>,
}

#[__data(str)]
pub struct FileRecvOpt {
    #[allow(dead_code)]
    pub url: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FileOption {
    Send(FileSendOpt),
    Receive(FileRecvOpt),
}

impl serde::Serialize for FileOption {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            FileOption::Send(s) => s.serialize(serializer),
            FileOption::Receive(r) => r.serialize(serializer),
        }
    }
}

impl<'de> serde::Deserialize<'de> for FileOption {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde_value::Value;
        let value = BTreeMap::<String, Value>::deserialize(deserializer)?;
        if value.contains_key("url") {
            Ok(FileOption::Receive(
                Deserialize::deserialize(Value::from_map(value))
                    .map_err(serde::de::Error::custom)?,
            ))
        } else {
            Ok(FileOption::Send(
                Deserialize::deserialize(Value::from_map(value))
                    .map_err(serde::de::Error::custom)?,
            ))
        }
    }
}

#[__data(default)]
#[serde(rename_all = "lowercase")]
pub enum ImageType {
    Flash,
    #[default]
    Normal,
}

#[serde_as]
#[__data(default, str)]
pub struct Image {
    pub file: String,
    #[serde_as(deserialize_as = "serde_with::DefaultOnNull")]
    pub r#type: ImageType,
    #[serde(flatten)]
    pub option: Option<FileOption>,
}
#[__data(default, str)]
pub struct Record {
    pub file: String,
    #[serde(default)]
    pub magic: bool,
    #[serde(flatten)]
    pub option: Option<FileOption>,
}
#[__data(default, str)]
pub struct Video {
    pub file: String,
    #[serde(flatten)]
    pub option: Option<FileOption>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AtTarget {
    All,
    QQ(i64),
}

impl Serialize for AtTarget {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::All => serializer.serialize_str("all"),
            Self::QQ(qq) => serializer.serialize_str(&qq.to_string()),
        }
    }
}
impl<'de> Deserialize<'de> for AtTarget {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let qq = String::deserialize(deserializer)?;
        if qq == "all" {
            Ok(Self::All)
        } else {
            qq.parse().map_err(serde::de::Error::custom).map(Self::QQ)
        }
    }
}

#[__data(str)]
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
#[__data(str)]
pub struct Poke {
    pub r#type: i32,
    pub id: i32,
    pub name: Option<String>,
}

#[__data(str)]
pub struct Share {
    pub url: String,
    pub title: String,
    pub content: Option<String>,
    pub image: Option<String>,
}

#[__data(str)]
pub struct Contact {
    pub r#type: String,
    pub id: u64,
}

#[__data(str)]
pub struct Location {
    pub lat: f64,
    pub lon: f64,
    pub title: Option<String>,
    pub content: Option<String>,
}

#[__data(str)]
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

#[__data(str)]
pub struct Rps;

#[__data(str)]
pub struct Dice;

#[__data(str)]
pub struct Shake;

#[__data(str)]
pub struct Anonymous;

#[__data(str)]
pub struct Forward {
    pub id: String,
}

#[__data(str)]
pub struct Xml {
    pub data: String,
}

#[__data(str)]
pub struct Json {
    pub data: String,
}

#[__data(str)]
#[serde(untagged)]
pub enum Node {
    Message {
        id: i32,
    },
    Custom {
        user_id: i64,
        nickname: String,
        content: Vec<MessageSeg>,
    },
}
