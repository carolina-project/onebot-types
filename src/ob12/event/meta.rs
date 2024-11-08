use ob_types_base::JSONValue;
use ob_types_macro::json;

use crate::ob12::{BotSelf, Status, VersionInfo};

#[json]
pub struct MetaEvent {
    #[serde(rename = "self")]
    pub self_: BotSelf,
    pub sub_type: String,
    #[serde(flatten)]
    pub r#kind: MetaKind,
    #[serde(flatten)]
    pub extra: JSONValue,
}

#[json(serde(tag = "detail_type", rename_all = "snake_case"))]
pub enum MetaKind {
    Connect {
        version: VersionInfo,
    },
    Heartbeat {
        interval: i64,
    },
    StatusUpdate {
        status: Status,
    },
    #[serde(untagged)]
    Extra {
        detail_type: String,
    },
}
