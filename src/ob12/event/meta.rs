use ob_types_macro::json;

use crate::{ob12::{Status, VersionInfo}, scalable_struct};

scalable_struct! {
    MetaEvent = {
        sub_type: Option<String>,
        #[serde(flatten)]
        kind: MetaKind,
    },
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
