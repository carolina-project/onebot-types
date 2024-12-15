use ob_types_macro::__data;

#[__data]
pub enum RequestEvent {
    #[serde(untagged)]
    Other(super::EventDetailed),
}

impl From<super::EventDetailed> for RequestEvent {
    fn from(value: super::EventDetailed) -> Self {
        Self::Other(value)
    }
}

impl From<RequestEvent> for super::EventKind {
    fn from(value: RequestEvent) -> Self {
        Self::Request(value)
    }
}
