use ob_types_macro::__data;

use super::{impl_from_into, EventType};

#[__data]
pub enum RequestEvent {
    #[serde(untagged)]
    Other(super::EventDetailed),
}

impl_from_into!(RequestEvent, EventType::Request);
