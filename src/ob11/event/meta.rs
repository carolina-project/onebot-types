use ob_types_macro::data;
use serde_value::Value;

#[data]
#[serde(tag = "meta_event_type", rename_all = "lowercase")]
pub enum MetaEvent {
    LifeCycle(LifeCycle),
    Heartbeat(Heartbeat),
}

#[data]
pub struct Heartbeat {
    pub status: Value,
    pub interval: u64,
}

#[data]
#[serde(tag = "sub_type", rename_all = "lowercase")]
pub enum LifeCycle {
    Enable,
    Disable,
    Connect,
}
