
pub mod action;
pub mod event;
pub mod message;

pub use message::MessageSeg;
pub use event::Event;
use ob_types_macro::data;

#[data]
#[serde(rename_all = "lowercase")]
pub enum Sex {
    Male,
    Female,
    Unknown,
}
