use ob_types_base::json::JSONValue;
use ob_types_macro::json;

#[json(serde(untagged))]
pub enum MetaEvent {
    LifeCycle(#[cfg_attr(feature = "json", serde(rename = "sub_type"))] LifeCycle),
    Heartbeat { status: JSONValue, interval: u64 },
}

#[json(serde(rename_all = "lowercase"))]
pub enum LifeCycle {
    Enable,
    Disable,
    Connect,
}
