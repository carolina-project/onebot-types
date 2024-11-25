pub mod message;
pub mod meta;
pub(self) use crate::ob11::event as ob11event;
pub(self) use crate::ob12::event as ob12event;
use crate::SerResult;

/// Trait to convert an OB11 event to an OB12 event.
/// P is the type of the parameter that the OB12 event requires.
/// The first parameter is always `self_id` in the ob11 event.
pub trait IntoOB12Event<P = ()> {
    type Output: Into<ob12event::EventType>;

    fn into_ob12(self, param: P) -> SerResult<Self::Output>;
}