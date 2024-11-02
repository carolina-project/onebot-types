pub mod types;

use ob_types_base::json::JSONValue;
use ob_types_macro::json;

#[allow(unused)]
use std::{fmt::Display, str::FromStr};
use types::*;

#[json]
pub struct OB11MessageSegRaw {
    pub r#type: String,
    pub data: JSONValue,
}

#[cfg(feature = "json")]
fn single_field_seg<S, D>(serializer: S, typ: &str, field: &str, var: D) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
    D: Display,
{
    use std::collections::HashMap;

    use serde::ser::SerializeMap;

    let map = HashMap::from([(field, var.to_string())]);
    let mut ser_map = serializer.serialize_map(Some(2))?;
    ser_map.serialize_entry("type", typ)?;
    ser_map.serialize_entry("data", &map)?;
    ser_map.end()
}

#[cfg(feature = "json")]
fn serialze_msg_seg<S, D>(serializer: S, typ: &str, data: &D) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
    D: serde::Serialize,
{
    use serde::ser::SerializeMap;
    let mut v = serializer.serialize_map(Some(2))?;
    v.serialize_entry("type", typ)?;
    v.serialize_entry("data", data)?;

    v.end()
}

#[allow(unused)]
macro_rules! msg_seg_ser_impl {
    ($ser:expr, $typ_name:literal, $r:ident, $inner:ty, $field:literal) => {
        single_field_seg($ser, $typ_name, $field, $r)
    };
    ($ser:expr, $typ_name:literal, $r:ident, $inner:ty) => {
        serialze_msg_seg($ser, $typ_name, $r)
    };
    ($ser:expr, $typ_name:literal) => {{
        use serde::ser::SerializeMap;
        let mut map = $ser.serialize_map(Some(1))?;
        map.serialize_entry("type", $typ_name)?;
        map.end()
    }};
}

#[allow(unused)]
macro_rules! msg_ser_match_rule {
    ($var:ident, $r:ident, $inner:ty) => {
        MessageSeg::$var($r)
    };
    ($var:ident) => {
        MessageSeg::$var
    };
}

#[cfg(feature = "json")]
fn get_json_field<T>(mut json: serde_json::Value, field: &'static str) -> serde_json::Result<T>
where
    T: serde::de::DeserializeOwned + FromStr,
    <T as FromStr>::Err: Display,
{
    use serde::de::Error;
    use serde_json::Value;

    let v = json
        .get_mut(field)
        .ok_or_else(|| serde_json::Error::missing_field(field))?
        .take();
    match v {
        Value::String(s) => s.parse().map_err(serde_json::Error::custom),
        r => serde_json::from_value(r).map_err(serde_json::Error::custom),
    }
}

#[allow(unused)]
macro_rules! msg_seg_deser_impl {
    ($var:ident, $r:expr, $inner:ty, $field:expr) => {
        get_json_field($r, $field).map(MessageSeg::$var)
    };
    ($var:ident, $r:expr, $inner:ty) => {
        serde_json::from_value($r).map(MessageSeg::$var)
    };
    ($var:ident) => {
        Ok(MessageSeg::$var)
    };
}

macro_rules! message_seg {
    ($($var:ident$(($inner:ty $(= $field:literal)? ))? $typ_name:literal $($doc:expr)?),* $(,)?) => {
        /// Message segment types, only support json messages currently.
        #[derive(Debug, Clone)]
        pub enum MessageSeg {
            $(
                $(#[doc = $doc])?
                $var$(($inner))?,
            )*
            Custom(OB11MessageSegRaw),
        }

        #[cfg(feature = "json")]
        impl serde::Serialize for super::MessageSeg {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                match self {
                    $(
                        msg_ser_match_rule!($var $(, r, $inner)?) => msg_seg_ser_impl!(serializer, $typ_name $(, r, $inner $(, $field)?)?),
                    )*
                    MessageSeg::Custom(r) => r.serialize(serializer),
                }
            }
        }

        #[cfg(feature = "json")]
        impl<'de> serde::Deserialize<'de> for super::MessageSeg {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                use serde::Deserialize;
                use std::borrow::Cow;

                #[derive(Deserialize)]
                struct Helper<'a> {
                    r#type: Cow<'a, str>,
                    data: serde_json::Value,
                }

                let helper = Helper::deserialize(deserializer)?;
                match helper.r#type.as_ref() {
                    $(
                        $typ_name => msg_seg_deser_impl!($var $(, helper.data, $inner $(, $field)?)?),
                    )*
                    typ => Ok(MessageSeg::Custom(OB11MessageSegRaw {
                        r#type: typ.into(),
                        data: helper.data.into(),
                    })),
                }.map_err(serde::de::Error::custom)
            }
        }
    };
}

message_seg!(
    Text(String = "text") "text",
    Face(u16 = "id") "face"
        "see [表情 CQ 码 ID 表](https://github.com/kyubotics/coolq-http-api/wiki/%E8%A1%A8%E6%83%85-CQ-%E7%A0%81-ID-%E8%A1%A8)",
    Image(Image) "image",
    Record(Record) "record",
    Video(Video) "video",
    At(AtTarget = "qq") "at",
    Rps "rps",
    Dice "dice",
    Shake "shake",
    Poke(Poke) "poke",
    Anonymous "anonymous",
    Share(Share) "share",
    Contact(Contact) "contact",
    Location(Location) "location",
    Music(Music) "music",
    Reply(u32 = "id") "reply",
    Forward(u64 = "id") "forward",
    ForwardNode(ForwardNode) "node",
    XML(String = "data") "xml",
    JSON(String = "data") "json",
);
