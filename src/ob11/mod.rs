
pub mod action;
pub mod event;
pub mod message;

pub use message::MessageSeg;
pub use event::RawEvent;
use ob_types_macro::__data;

#[__data]
#[serde(rename_all = "lowercase")]
pub enum Sex {
    Male,
    Female,
    Unknown,
}
