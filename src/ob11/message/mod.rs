pub mod types;

use std::collections::HashMap;

use ob_types_base::json::{JSONMap, JSONValue};
use ob_types_macro::json;
use types::*;

#[derive(Clone, Debug)]
#[json]
pub struct OB11MessageSegRaw {
    pub r#type: String,
    pub data: JSONValue,
}

#[cfg(feature = "json")]
fn single_field_seg(
    typ: &str,
    field: &str,
    var: impl Into<serde_json::Value>,
) -> serde_json::Value {
    use serde_json::{Map, Value};

    let mut v = Map::new();
    v.insert("type".into(), typ.into());
    v.insert(
        "data".into(),
        Value::Object({
            let mut map = serde_json::Map::new();
            map.insert(field.into(), var.into());
            map
        }),
    );

    serde_json::Value::Object(v)
}

#[cfg(feature = "json")]
fn serialze_msg_seg(
    typ: &str,
    data: impl serde::Serialize,
) -> serde_json::Result<serde_json::Value> {
    use serde_json::Map;
    let mut v = Map::new();
    v.insert("type".into(), typ.into());
    v.insert("data".into(), serde_json::to_value(data)?);
    Ok(serde_json::Value::Object(v))
}

macro_rules! msg_seg_ser_impl {
    ($name:ident, $var:ident($inner:ty = $field:literal) $typ_name:literal) => {
        $name::$var(r) => single_field_seg($typ_name, $field, r),
    };
    ($name:ident, $var:ident($inner:ty) $typ_name:literal) => {
        $name::$var(r) => serialze_msg_seg($typ_name, r),
    };
}

macro_rules! message_seg {
    ($vi:vis enum $name:ident {
        $($var:tt,)*,
    }) => {
        $vi enum $name {
            $(message_seg!($var), )*
        }

        #[cfg(feature = "json")]
        impl $name {
            pub fn to_json(&self) -> serde_json::Result<serde_json::Value> {
                match self {
                    $(msg_seg_ser_impl!($name, $var))*
                }
            }
        }
    };
    ($var:ident($inner:ty = $field:literal) $typ_name:literal) => {
        $var($inner),
    };
    ($var:ident($inner:ty) $typ_name:literal) => {
        $var($inner),
    };
}

message_seg!(pub enum MessageSeg {
    Text(String = "text") "text",
});

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

macro_rules! msg_ele_match {
    // Match the main structure, taking self and a variable number of type-field pairs
    ($self:ident $(, $typ:ident: $field:literal)*) => {
        match $self {
            // Expand each type-field pair into a match pattern
            $(MessageSeg::$typ(r) => {
            },)*
            // Default case
            _ => None,
        }
    };
}

impl MessageSeg {
    pub(crate) fn into_single_ele_json(&self) -> Option<(&str, JSONValue)> {
        msg_ele_match!(self, Text: "text",);
        todo!("read the doc")
    }
}

#[cfg(feature = "json")]
mod serde_impl {
    use serde::Serialize;

    impl Serialize for super::MessageSeg {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
        }
    }
}
