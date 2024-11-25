use crate::{compat::default_obj, DesResult};

use super::*;
use ob12event::meta::*;
use ob_types_base::ext::ValueExt;
use ob_types_macro::json;
use serde::Deserialize;
use serde_value::Value;

#[json]
pub enum LifeCycle {
    #[serde(rename = "ob11.enable")]
    Enable,
    #[serde(rename = "ob11.disable")]
    Disable,
}

impl LifeCycle {
    pub const TYPE: &str = "ob11.lifecycle";
}

pub enum CompatLifecycle {
    Connect(Connect),
    Lifecycle(LifeCycle),
}

impl From<CompatLifecycle> for ob12event::EventType {
    fn from(value: CompatLifecycle) -> Self {
        Self::Meta(match value {
            CompatLifecycle::Connect(c) => MetaEvent {
                sub_type: Default::default(),
                kind: MetaKind::Connect(c),
            },
            CompatLifecycle::Lifecycle(cycle) => MetaEvent {
                sub_type: serde_value::to_value(cycle)
                    .unwrap()
                    .try_into_string()
                    .expect("invalid type"),
                kind: MetaKind::Other {
                    detail_type: LifeCycle::TYPE.into(),
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
    use serde_value::Value;

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

    impl From<(Heartbeat, Value)> for ob12event::EventType {
        #[inline]
        fn from(value: (Heartbeat, Value)) -> Self {
            ob12event::EventType::Meta(MetaEvent {
                sub_type: Default::default(),
                kind: MetaKind::Heartbeat(value.0),
            })
        }
    }
}
