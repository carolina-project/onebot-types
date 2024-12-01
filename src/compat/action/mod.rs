use std::{future::Future, num::ParseIntError};

pub(self) use crate::ob11::action as ob11action;
pub(self) use crate::ob12::action as ob12action;
pub(self) use crate::{ob11, ob12};
use crate::{DesResult, ValueMap};
use ob_types_base::ext::{IntoValue, ValueExt};
use ob_types_base::OBRespData;
use ob_types_macro::data;
pub(self) use serde::de::Error as DeError;
pub(self) use serde_value::DeserializerError;
pub(self) use serde_value::Value;

use super::default_obj;

pub mod bot;
pub mod friend;
pub mod group;

impl TryFrom<ob12::ChatTarget> for ob11action::ChatTarget {
    type Error = ParseIntError;

    fn try_from(value: ob12::ChatTarget) -> Result<Self, Self::Error> {
        match value {
            ob12::ChatTarget::Private { user_id } => user_id.parse().map(Self::Private),
            ob12::ChatTarget::Group {
                group_id,
                user_id: _,
            } => group_id.parse().map(Self::Group),
            ob12::ChatTarget::Channel {
                guild_id: _,
                channel_id: _,
            } => Ok(Self::Unknown),
            ob12::ChatTarget::Other { detail_type: _ } => Ok(Self::Unknown),
        }
    }
}

pub trait IntoOB11Action<P = ()> {
    type Output: TryInto<ob11action::ActionType>;

    fn into_ob11(self, param: P) -> DesResult<Self::Output>;
}

pub trait IntoOB11ActionAsync<P> {
    type Output: TryInto<ob11action::ActionType>;

    fn into_ob11(self, param: P) -> impl Future<Output = DesResult<Self::Output>>;
}

pub trait FromOB11Resp<P = ()>
where
    Self: Sized,
{
    type In: OBRespData;

    fn from_ob11(from: Self::In, param: P) -> DesResult<Self>;
}

pub trait FromOB11RespAsync<P = ()>
where
    Self: Sized,
{
    type In: OBRespData;

    fn from_ob11(from: Self::In, param: P) -> impl Future<Output = DesResult<Self>>;
}

macro_rules! compat_actions {
    ($($ob11action:ident),*) => {
        pub enum CompatAction {
            $($ob11action(ob11action::$ob11action)),*
        }

        impl IntoOB11Action for CompatAction {
            type Output = ob11action::ActionType;

            fn into_ob11(self, _: ()) -> DesResult<Self::Output> {
                Ok(match self {$(
                    CompatAction::$ob11action(action)
                        => ob11action::ActionType::$ob11action(action),
                )*})
            }
        }
    };
}

macro_rules! compat_resp {
    ($($ob11resp:ident),*) => {
        pub enum CompatResp {
            $($ob11resp(ob11action::$ob11resp)),*
        }

        $(
            impl From<ob11action::$ob11resp> for CompatResp {
                fn from(ob11: ob11action::$ob11resp) -> Self {
                    CompatResp::$ob11resp(ob11)
                }
            }
        )*
    };
}

compat_actions!(
    GetMsg,
    GetForwardMsg,
    GetCookies,
    GetCsrfToken,
    GetCredentials,
    CanSendImage,
    CanSendRecord,
    GetStatus,
    GetVersion,
    SetRestart,
    CleanCache
);

#[inline]
pub(self) fn unwrap_value_map(value: Value) -> DesResult<ValueMap> {
    match value {
        serde_value::Value::Map(map) => Ok(map),
        _ => Err(DeserializerError::custom("invalid value, expected map")),
    }
}

#[inline]
fn remove_field_or_default<'a, T: serde::Deserialize<'a> + Default>(
    map: &mut ValueMap,
    key: &str,
) -> DesResult<T> {
    if let Some(r) = map.remove(&Value::String(key.into())) {
        T::deserialize(r)
    } else {
        Ok(T::default())
    }
}

#[data]
pub enum UserInfoResp {
    LoginInfo(ob11action::LoginInfo),
    StrangerInfo(ob11action::StrangerInfoResp),
}

impl FromOB11Resp for ob12::UserInfo {
    type In = UserInfoResp;

    fn from_ob11(from: Self::In, _: ()) -> DesResult<Self> {
        match from {
            UserInfoResp::LoginInfo(ob11action::LoginInfo {
                user_id,
                nickname: user_display_name,
            }) => {
                let user_id = user_id.to_string();
                Ok(Self {
                    user_name: user_id.clone(),
                    user_id,
                    user_display_name,
                    user_remark: None,
                    extra: default_obj(),
                })
            }
            UserInfoResp::StrangerInfo(ob11action::StrangerInfoResp {
                user_id,
                nickname: user_display_name,
                sex,
                age,
            }) => {
                let user_id = user_id.to_string();
                let extra = Value::from_map(
                    [
                        (
                            "ob11.sex",
                            serde_value::to_value(sex).map_err(DeserializerError::custom)?,
                        ),
                        ("ob11.age", age.into_value()),
                    ]
                    .into(),
                );
                Ok(Self {
                    user_name: user_id.clone(),
                    user_id,
                    user_display_name,
                    user_remark: None,
                    extra,
                })
            }
        }
    }
}
