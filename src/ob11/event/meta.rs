use ob_types_base::json::JSONValue;


pub enum MetaEvent {
    LifeCycle(LifeCycle),
    Heartbeat { status: JSONValue, interval: u64 },
}

pub enum LifeCycle {
    Enable,
    Disable,
    Connect,
}
