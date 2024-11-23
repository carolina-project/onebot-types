pub mod event;
pub mod message;

#[inline]
pub(self) fn default_obj() -> serde_value::Value {
    serde_value::Value::Map(Default::default())
}
