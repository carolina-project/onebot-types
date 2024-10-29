pub mod types;

use ob_types_base::json::JSONValue;
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
    ($typ_name:literal, $r:ident, $inner:ty, $field:literal) => {
        Ok(single_field_seg($typ_name, $field, $r.to_owned()))
    };
    ($typ_name:literal, $r:ident, $inner:ty) => {
        serialze_msg_seg($typ_name, $r.to_owned())
    };
    ($typ_name:literal) => {
        Ok(serde_json::json!({
            "type": $typ_name,
        }))
    };
}

macro_rules! msg_ser_match_rule {
    ($var:ident, $r:ident, $inner:ty) => {
        MessageSeg::$var($r)
    };
    ($var:ident) => {
        MessageSeg::$var
    };
}

#[cfg(feature = "json")]
fn get_json_field<T: serde::de::DeserializeOwned>(
    mut json: serde_json::Value,
    field: &'static str,
) -> serde_json::Result<T> {
    use serde::de::Error;

    serde_json::from_value(
        json.get_mut(field)
            .ok_or_else(|| serde_json::Error::missing_field(field))?
            .take(),
    )
}

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
        pub enum MessageSeg {
            $(
                $(#[doc = $doc])?
                $var$(($inner))?,
            )*
        }

        #[cfg(feature = "json")]
        impl<'de> MessageSeg {
            pub fn to_json(&self) -> serde_json::Result<serde_json::Value> {
                match self {
                    $(
                        msg_ser_match_rule!($var $(, r, $inner)?) => msg_seg_ser_impl!($typ_name $(, r, $inner $(, $field)?)?),
                    )*
                }
            }

            pub fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<MessageSeg, D::Error> {
                use serde::{de::Error, Deserialize};

                #[derive(Deserialize)]
                struct Helper {
                    r#type: String,
                    data: serde_json::Value,
                }

                let helper = Helper::deserialize(deserializer)?;
                match helper.r#type.as_str() {
                    $(
                        $typ_name => msg_seg_deser_impl!($var $(, helper.data, $inner $(, $field)?)?),
                    )*
                    _ => Err(serde_json::Error::custom(format!("Unknown message segment type: {}", helper.r#type) ))
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
    At(AtTarget) "at",
    Rps "rps",
    Dice "dice",
    Shake "shake",
    Poke(Poke) "poke",
    Anonymous "anonymous",
    Share(Share) "share",
    Contact(Contact) "contact",
    Location(Location) "location",
    Music(Music) "music",
    Reply(u32) "reply",
    Forward(u64) "forward",
    ForwardNode(ForwardNode) "node",
    XML(String) "xml",
    JSON(String) "json",
);

#[cfg(feature = "json")]
mod serde_impl {
    use serde::{Deserialize, Serialize};

    impl Serialize for super::MessageSeg {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            let v = self.to_json().map_err(|e| serde::ser::Error::custom(e))?;
            v.serialize(serializer)
        }
    }

    impl<'de> Deserialize<'de> for super::MessageSeg {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            Self::deserialize(deserializer)
        }
    }
}
