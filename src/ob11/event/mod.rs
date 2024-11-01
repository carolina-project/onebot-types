use meta::MetaEvent;
use notice::NoticeEvent;
use ob_types_base::JSONValue;
use ob_types_macro::json;
use request::RequestEvent;

use self::message::MessageEvent;

pub mod message;
pub mod meta;
pub mod notice;
pub mod request;

#[derive(Clone, Copy)]
#[json(serde(rename_all = "snake_case"))]
pub enum OB11PostType {
    MetaEvent,
    Message,
    Notice,
    Request,
}

#[derive(Clone)]
#[json]
pub struct OB11EventRaw {
    pub time: u64,
    pub self_id: u64,
    pub post_type: OB11PostType,
    #[cfg_attr(feature = "json", serde(flatten))]
    pub extra: JSONValue,
}

#[json]
pub struct Event {
    pub time: i64,
    pub self_id: i64,
    #[cfg_attr(feature = "json", serde(flatten))]
    pub kind: EventKind,
}

#[derive(Debug)]
pub enum EventKind {
    Message(MessageEvent),
    Meta(MetaEvent),
    Request(RequestEvent),
    Notice(NoticeEvent),
}

impl EventKind {
    pub const MESSAGE: &'static str = "message";
    pub const META: &'static str = "meta_event";
    pub const REQUEST: &'static str = "request";
    pub const NOTICE: &'static str = "notice";

    pub fn post_type(&self) -> &'static str {
        match self {
            Self::Message(_) => Self::MESSAGE,
            Self::Request(_) => Self::REQUEST,
            Self::Meta(_) => Self::META,
            Self::Notice(_) => Self::NOTICE,
        }
    }
}

#[cfg(feature = "json")]
mod serde_impl {
    use super::*;
    use serde::{de::Error, Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "snake_case")]
    enum PostType {
        Message,
        MetaEvent,
        Notice,
        Request,
    }

    #[derive(Serialize)]
    struct SerHelper<'a> {
        post_type: &'static str,
        #[serde(flatten)]
        data: &'a EventKind,
    }

    #[derive(Deserialize)]
    struct DeHelper {
        post_type: PostType,
        #[serde(flatten)]
        data: serde_json::Value,
    }

    impl Serialize for EventKind {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::ser::Serializer,
        {
            SerHelper {
                post_type: self.post_type(),
                data: self,
            }
            .serialize(serializer)
        }
    }

    impl<'de> Deserialize<'de> for EventKind {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let v = DeHelper::deserialize(deserializer)?;
            match v.post_type {
                PostType::Message => MessageEvent::deserialize(v.data).map(EventKind::Message),
                PostType::MetaEvent => MetaEvent::deserialize(v.data).map(EventKind::Meta),
                PostType::Notice => NoticeEvent::deserialize(v.data).map(EventKind::Notice),
                PostType::Request => RequestEvent::deserialize(v.data).map(EventKind::Request),
            }
            .map_err(D::Error::custom)
        }
    }
}
