use crate::{base::ext::ValueMapExt, DesResult};

use super::*;
use ob12event::meta;
use ob_types_macro::__data;
use serde::Deserialize;
use serde_value::Value;

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

impl LifeCycle {
    #[inline]
    pub fn lifecycle_from(sub_type: impl Into<String>) -> DesResult<Self> {
        LifeCycle::deserialize(Value::String(sub_type.into()))
    }
}

pub mod ob11to12 {
    use crate::ob12;

    use super::*;
    use ob11event::meta;
    use ob12event::{meta::*, EventType};

    impl IntoOB12Event<&ob12::VersionInfo> for ob11event::MetaEvent {
        type Output = (ob12event::Event, Option<Value>);

        fn into_ob12(self, param: &ob12::VersionInfo) -> SerResult<Self::Output> {
            match self {
                meta::MetaEvent::LifeCycle(cycle) => {
                    let cycle = cycle.into_ob12(param)?;
                    Ok((
                        ob12event::Event {
                            r#type: EventType::Meta,
                            detailed: {
                                let event: ob12event::MetaEvent = cycle.into();
                                event.try_into()?
                            },
                        },
                        None,
                    ))
                }
                ob11event::MetaEvent::Heartbeat(beat) => beat.into_ob12(()).map(|(r, s)| {
                    Ok((
                        ob12event::Event {
                            r#type: EventType::Meta,
                            detailed: ob12event::MetaEvent::Heartbeat(r).try_into()?,
                        },
                        Some(s),
                    ))
                })?,
            }
        }
    }

    impl IntoOB12Event<&ob12::VersionInfo> for meta::LifeCycle {
        type Output = CompatLifecycle;

        fn into_ob12(self, param: &ob12::VersionInfo) -> SerResult<Self::Output> {
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
        type Output = (Heartbeat, Value);

        #[inline]
        fn into_ob12(self, _param: ()) -> SerResult<Self::Output> {
            Ok((
                Heartbeat {
                    interval: self.interval,
                    extra: Default::default(),
                },
                self.status,
            ))
        }
    }

    impl<T> From<(ob12event::EventKind, T)> for ob12event::EventKind {
        #[inline]
        fn from(value: (ob12event::EventKind, T)) -> Self {
            value.0
        }
    }

    impl<T> From<(Heartbeat, T)> for ob12event::EventKind {
        #[inline]
        fn from(value: (Heartbeat, T)) -> Self {
            ob12event::EventKind::Meta(MetaEvent::Heartbeat(value.0))
        }
    }
}
