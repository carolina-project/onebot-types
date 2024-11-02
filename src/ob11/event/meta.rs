use ob_types_base::json::JSONValue;
use ob_types_macro::json;

#[json(serde(tag = "meta_event_type", rename_all = "lowercase"))]
pub enum MetaEvent {
    LifeCycle(LifeCycle),
    Heartbeat {
        status: JSONValue,
        interval: u64,
    },
}

#[json(serde(tag = "sub_type", rename_all = "lowercase"))]
pub enum LifeCycle {
    Enable,
    Disable,
    Connect,
}
