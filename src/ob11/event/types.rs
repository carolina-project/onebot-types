use serde::de::DeserializeOwned;

pub struct Event {
    pub time: i64,
    pub self_id: i64,
    pub kind: EventKind
}

pub trait HasQuickAction {
    type Action: DeserializeOwned;
}

pub enum EventKind {

}
