use crate::{compat::default_obj, DesResult};

use super::*;
use ob12event::meta;
use ob_types_base::ext::ValueExt;
use ob_types_macro::data;
use serde::Deserialize;
use serde_value::Value;

#[data]
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

impl From<CompatLifecycle> for ob12event::EventType {
    fn from(value: CompatLifecycle) -> Self {
        Self::Meta(match value {
            CompatLifecycle::Connect(c) => ob12event::MetaEvent {
                sub_type: Default::default(),
                kind: meta::MetaKind::Connect(c),
            },
            CompatLifecycle::Lifecycle(cycle) => ob12event::MetaEvent {
                sub_type: serde_value::to_value(cycle)
                    .unwrap()
                    .try_into_string()
                    .expect("invalid type"),
                kind: meta::MetaKind::Other {
                    detail_type: "ob11.lifecycle".into(),
                    data: default_obj(),
                },
            },
        })
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
    use ob12event::meta::*;
    use serde_value::Value;

    impl IntoOB12Event<&ob12::VersionInfo> for ob11event::MetaEvent {
        type Output = (ob12event::EventType, Option<Value>);

        fn into_ob12(self, param: &ob12::VersionInfo) -> SerResult<Self::Output> {
            match self {
                meta::MetaEvent::LifeCycle(cycle) => {
                    let cycle = cycle.into_ob12(param)?;
                    match cycle {
                        CompatLifecycle::Connect(connect) => Ok((
                            ob12event::EventType::Meta(MetaEvent {
                                sub_type: Default::default(),
                                kind: MetaKind::Connect(connect),
                            }),
                            None,
                        )),
                        CompatLifecycle::Lifecycle(c) => Ok((
                            ob12event::EventType::Meta(ob12event::MetaEvent {
                                sub_type: serde_value::to_value(c)?
                                    .try_into_string()
                                    .ok_or_else(|| serde::ser::Error::custom("invalid type"))?,
                                kind: MetaKind::Other {
                                    detail_type: "ob11.lifecycle".into(),
                                    data: default_obj(),
                                },
                            }),
                            None,
                        )),
                    }
                }
                ob11event::MetaEvent::Heartbeat(beat) => beat.into_ob12(()).map(|(r, s)| {
                    (
                        ob12event::EventType::Meta(MetaEvent {
                            sub_type: Default::default(),
                            kind: MetaKind::Heartbeat(r),
                        }),
                        Some(s),
                    )
                }),
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
                    extra: default_obj(),
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
                    extra: default_obj(),
                },
                self.status,
            ))
        }
    }

    impl<T> From<(ob12event::EventType, T)> for ob12event::EventType {
        #[inline]
        fn from(value: (ob12event::EventType, T)) -> Self {
            value.0
        }
    }

    impl<T> From<(Heartbeat, T)> for ob12event::EventType {
        #[inline]
        fn from(value: (Heartbeat, T)) -> Self {
            ob12event::EventType::Meta(MetaEvent {
                sub_type: Default::default(),
                kind: MetaKind::Heartbeat(value.0),
            })
        }
    }
}
