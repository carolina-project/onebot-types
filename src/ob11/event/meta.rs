use ob_types_macro::json;
use serde_value::Value;

#[json]
#[serde(tag = "meta_event_type", rename_all = "lowercase")]
pub enum MetaEvent {
    LifeCycle(LifeCycle),
    Heartbeat(Heartbeat),
}

#[json]
pub struct Heartbeat {
    pub status: Value,
    pub interval: u64,
}

#[json]
#[serde(tag = "sub_type", rename_all = "lowercase")]
pub enum LifeCycle {
    Enable,
    Disable,
    Connect,
}
