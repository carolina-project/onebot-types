pub(self) use super::*;

use std::{future::Future, num::ParseIntError};

pub(self) use crate::ob11::action as ob11action;
pub(self) use crate::ob12::action as ob12action;
pub(self) use crate::{ob11, ob12};
use crate::{DesResult, ValueMap};
use ob_types_base::ext::{IntoValue, ValueExt};
use ob_types_base::OBRespData;
use ob_types_macro::data;
pub(self) use serde::de::Error as DeError;
use serde::Deserialize;
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
    ($($ob11action:ident $name:literal),*) => {
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

        impl CompatAction {
            pub fn into_data(self) -> Result<(&'static str, Value), CompatError> {
                match self {
                    $(CompatAction::$ob11action(action)
                        => Ok((concat!("ob11.", $name), serde_value::to_value(action).map_err(DeserializerError::custom)?)),
                    )*
                }
            }

            pub fn from_data(name: &str, data: Value) -> Result<CompatAction, CompatError> {
                match name {
                    $(concat!("ob11.", $name)
                        => Ok(Deserialize::deserialize(data).map(CompatAction::$ob11action).map_err(DeserializerError::custom)?),)*
                    name => Err(CompatError::UnknownCompat(name.into())),
                }
            }
        }
    };
}

impl TryFrom<CompatAction> for ob12action::ActionType {
    type Error = CompatError;

    fn try_from(value: CompatAction) -> Result<Self, Self::Error> {
        let (action, params) = value.into_data()?;
        Ok(Self::Other(ob12action::ActionTypeRaw {
            action: action.into(),
            params,
        }))
    }
}

compat_actions!(
    GetMsg "get_msg",
    GetForwardMsg "get_forward_msg",
    GetCookies "get_cookies",
    GetCsrfToken "get_csrf_token",
    GetCredentials "get_credentials",
    CanSendImage "can_send_image",
    CanSendRecord "can_send_record",
    GetStatus "get_status",
    GetVersionInfo "get_version_info",
    SetRestart "set_restart",
    CleanCache "clean_cache"
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

#[inline]
fn remove_field<'a, T: serde::Deserialize<'a>>(
    map: &mut ValueMap,
    key: &str,
) -> Option<DesResult<T>> {
    if let Some(r) = map.remove(&Value::String(key.into())) {
        Some(T::deserialize(r))
    } else {
        None
    }
}

#[data]
pub enum UserInfoResp {
    LoginInfo(ob11action::LoginInfo),
    StrangerInfo(ob11action::StrangerInfoResp),
    FriendInfo(ob11action::FriendInfo),
    GroupMemberInfo(ob11action::GroupMemberInfo),
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
            UserInfoResp::FriendInfo(ob11action::FriendInfo {
                user_id,
                nickname: user_display_name,
                remark,
            }) => {
                let user_id = user_id.to_string();
                Ok(Self {
                    user_name: user_id.clone(),
                    user_id,
                    user_display_name,
                    user_remark: Some(remark),
                    extra: default_obj(),
                })
            }
            UserInfoResp::GroupMemberInfo(ob11action::GroupMemberInfo {
                group_id: _,
                user_id,
                nickname: user_name,
                card: user_display_name,
                sex,
                age,
                area,
                join_time,
                last_sent_time,
                level,
                role,
                unfriendly,
                title,
                title_expire_time,
                card_changeable,
            }) => {
                #[inline]
                fn to_value<T: serde::Serialize>(
                    v: T,
                ) -> Result<Value, serde_value::DeserializerError> {
                    serde_value::to_value(v).map_err(DeserializerError::custom)
                }
                let extra = Value::from_map(
                    [
                        ("ob11.sex", to_value(sex)?),
                        ("ob11.age", age.into_value()),
                        ("ob11.area", area.into_value()),
                        ("ob11.join_time", join_time.into_value()),
                        ("ob11.last_sent_time", last_sent_time.into_value()),
                        ("ob11.level", level.into_value()),
                        ("ob11.role", role.into_value()),
                        ("ob11.unfriendly", unfriendly.into_value()),
                        ("ob11.title", title.into_value()),
                        ("ob11.title_expire_time", title_expire_time.into_value()),
                        ("ob11.card_changeable", card_changeable.into_value()),
                    ]
                    .into(),
                );

                Ok(Self {
                    user_id: user_id.to_string(),
                    user_name,
                    user_display_name,
                    user_remark: None,
                    extra,
                })
            }
        }
    }
}
