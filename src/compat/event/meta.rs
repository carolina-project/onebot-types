use crate::{base::ext::ValueMapExt, DesResult};

use super::*;
use ob12event::meta;
use ob_types_macro::__data;
use serde::Deserialize;
use serde_value::{SerializerError, Value};

#[__data]
pub enum LifeCycle {
    #[serde(rename = "ob11.enable")]
    Enable,
    #[serde(rename = "ob11.disable")]
    Disable,
}

pub enum CompatLifecycle {
    Connect(meta::Connect),
    Lifecycle(LifeCycle),
}

impl From<CompatLifecycle> for ob12event::MetaEvent {
    fn from(value: CompatLifecycle) -> Self {
        match value {
            CompatLifecycle::Connect(c) => ob12event::MetaEvent::Connect(c),
            CompatLifecycle::Lifecycle(cycle) => {
                ob12event::MetaEvent::Other(ob12event::EventDetailed {
                    detail_type: "ob11.lifecycle".to_owned(),
                    detail: [("sub_type".to_owned(), serde_value::to_value(cycle).unwrap())]
                        .into_map(),
                })
            }
        }
    }
}

impl TryFrom<CompatLifecycle> for ob12event::EventDetailed {
    type Error = SerializerError;

    fn try_from(cycle: CompatLifecycle) -> Result<Self, Self::Error> {
        Into::<ob12event::MetaEvent>::into(cycle).try_into()
    }
}

impl TryFrom<CompatLifecycle> for ob12event::EventDetail {
    type Error = SerializerError;

    fn try_from(cycle: CompatLifecycle) -> Result<Self, Self::Error> {
        Into::<ob12event::MetaEvent>::into(cycle).try_into()
    }
}

impl LifeCycle {
    #[inline]
    pub fn lifecycle_from(sub_type: impl Into<String>) -> DesResult<Self> {
        LifeCycle::deserialize(Value::String(sub_type.into()))
    }
}

pub mod ob11to12 {
    use crate::{ob11::Status, ob12};

    use super::*;
    use ob11event::meta;
    use ob12event::meta::*;

    impl IntoOB12Event<&ob12::VersionInfo> for ob11event::MetaEvent {
        type Output = (ob12event::MetaEvent, Option<Status>);

        fn into_ob12(self, param: &ob12::VersionInfo) -> CompatResult<Self::Output> {
            match self {
                meta::MetaEvent::LifeCycle(cycle) => {
                    let cycle = cycle.into_ob12(param)?;
                    Ok((cycle.into(), None))
                }
                ob11event::MetaEvent::Heartbeat(beat) => beat
                    .into_ob12(())
                    .map(|(r, s)| Ok((ob12event::MetaEvent::Heartbeat(r), Some(s))))?,
            }
        }
    }

    impl IntoOB12Event<&ob12::VersionInfo> for meta::LifeCycle {
        type Output = CompatLifecycle;

        fn into_ob12(self, param: &ob12::VersionInfo) -> CompatResult<Self::Output> {
            Ok(match self {
                meta::LifeCycle::Enable => CompatLifecycle::Lifecycle(LifeCycle::Enable),
                meta::LifeCycle::Disable => CompatLifecycle::Lifecycle(LifeCycle::Disable),
                meta::LifeCycle::Connect => CompatLifecycle::Connect(Connect {
                    version: param.clone(),
                    extra: Default::default(),
                }),
            })
        }
    }

    impl IntoOB12Event for meta::Heartbeat {
        type Output = (Heartbeat, Status);

        #[inline]
        fn into_ob12(self, _param: ()) -> CompatResult<Self::Output> {
            Ok((
                Heartbeat {
                    interval: self.interval,
                    extra: Default::default(),
                },
                self.status,
            ))
        }
    }

    impl<T> TryFrom<(ob12event::MetaEvent, T)> for ob12event::EventDetail {
        type Error = SerializerError;

        #[inline]
        fn try_from(value: (ob12event::MetaEvent, T)) -> Result<Self, Self::Error> {
            value.0.try_into()
        }
    }

    impl<T> TryFrom<(Heartbeat, T)> for ob12event::EventDetail {
        type Error = SerializerError;

        #[inline]
        fn try_from(value: (Heartbeat, T)) -> Result<Self, Self::Error> {
            ob12event::MetaEvent::Heartbeat(value.0).try_into()
        }
    }
}
