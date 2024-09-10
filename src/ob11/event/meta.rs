use ob_types_base::cross::Data;

pub enum MetaEvent {
    LifeCycle(LifeCycle),
    Heartbeat { status: Data, interval: u64 },
}

pub enum LifeCycle {
    Enable,
    Disable,
    Connect,
}
