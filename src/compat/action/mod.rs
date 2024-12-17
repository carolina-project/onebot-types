pub(self) use super::*;

use serde::de::IntoDeserializer;
use std::{future::Future, num::ParseIntError};

use crate::base::ext::{IntoValue, ValueMapExt};
use crate::base::RespData;
pub(self) use crate::ob11::action as ob11action;
pub(self) use crate::ob12;
pub(self) use crate::ob12::action as ob12action;
use ob_types_macro::__data;
pub(self) use serde::de::Error as DeError;
pub(self) use serde::ser::Error as SerError;
use serde::Deserialize;
pub(self) use serde_value::Value;
pub(self) use serde_value::{DeserializerError, SerializerError};

use crate::{DesResult, ValueMap};

pub mod bot;
pub mod friend;
pub mod group;

impl TryFrom<ob12::ChatTarget> for ob11action::ChatTarget {
    type Error = ParseIntError;

    fn try_from(value: ob12::ChatTarget) -> Result<Self, Self::Error> {
        match value {
            ob12::ChatTarget::Private { user_id } => {
                user_id.parse().map(|r| Self::Private { user_id: r })
            }
            ob12::ChatTarget::Group {
                group_id,
                user_id: _,
            } => group_id.parse().map(|r| Self::Group { group_id: r }),
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
    type In: RespData;

    fn from_ob11(from: Self::In, param: P) -> DesResult<Self>;
}

pub trait FromOB11RespAsync<P = ()>
where
    Self: Sized,
{
    type In: RespData;

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

        $(impl TryFrom<ob11action::$ob11action> for ob12action::ActionDetail {
            type Error = CompatError;

            fn try_from(value: ob11action::$ob11action) -> Result<Self, Self::Error> {
                CompatAction::$ob11action(value).try_into()
            }
        })*

        impl TryFrom<CompatAction> for ob11action::ActionDetail {
            type Error = SerializerError;

            fn try_from(action: CompatAction) -> Result<Self, Self::Error> {
                let (name, params) = action.into_ob11_data()?;
                Ok(ob11action::ActionDetail {
                    action: name.into(),
                    params
                })
            }
        }

        impl CompatAction {
            pub fn into_data(self) -> Result<(&'static str, ValueMap), SerializerError> {
                match self {
                    $(CompatAction::$ob11action(action)
                        => Ok((concat!("ob11.", $name),
                            serde_value::to_value(action)
                            .and_then(|r| {
                                ValueMap::deserialize(r).map_err(SerializerError::custom)
                            })?)
                        ),
                    )*
                }
            }

            pub fn into_ob11_data(self) -> Result<(&'static str, ValueMap), SerializerError> {
                match self {
                    $(CompatAction::$ob11action(action)
                        => Ok(($name,
                            serde_value::to_value(action)
                            .and_then(|r| {
                                ValueMap::deserialize(r).map_err(SerializerError::custom)
                            })?)),
                    )*
                }
            }

            pub fn from_data(name: impl AsRef<str>, data: ValueMap) -> Result<CompatAction, CompatError> {
                match name.as_ref() {
                    $(concat!("ob11.", $name)
                        => Ok(Deserialize::deserialize(
                                data.into_deserializer()
                            ).map(CompatAction::$ob11action).map_err(DeserializerError::custom)?),)*
                    name => Err(CompatError::UnknownCompat(name.into())),
                }
            }
        }
    };
}

impl TryFrom<CompatAction> for ob12action::ActionDetail {
    type Error = CompatError;

    fn try_from(value: CompatAction) -> Result<Self, Self::Error> {
        let (action, params) = value.into_data()?;
        Ok(ob12action::ActionDetail {
            action: action.into(),
            params,
        })
    }
}

compat_actions!(
    GetMsg "get_msg",
    GetForwardMsg "get_forward_msg",
    SendLike "send_like",
    GetCookies "get_cookies",
    GetCsrfToken "get_csrf_token",
    GetCredentials "get_credentials",
    CanSendImage "can_send_image",
    CanSendRecord "can_send_record",
    SetRestart "set_restart",
    CleanCache "clean_cache",
    SetGroupKick "set_group_kick",
    SetGroupBan "set_group_ban",
    SetGroupAnonymousBan "set_group_anonymous_ban",
    SetGroupWholeBan "set_group_whole_ban",
    SetGroupAdmin "set_group_admin",
    SetGroupAnonymous "set_group_anonymous",
    SetGroupCard "set_group_card",
    SetGroupSpecialTitle "set_group_special_title",
    SetFriendAddRequest "set_friend_add_request",
    SetGroupAddRequest "set_group_add_request",
    GetGroupHonorInfo "get_group_honor_info"
);

pub static SUPPORTED_ACTIONS: &[&str] = &[
    "send_message",
    "delete_message",
    "ob11.get_msg",
    "ob11.get_forward_msg",
    "ob11.send_like",
    "ob11.set_group_kick",
    "ob11.set_group_ban",
    "ob11.set_group_anonymous_ban",
    "ob11.set_group_whole_ban",
    "ob11.set_group_admin",
    "ob11.set_group_anonymous",
    "ob11.set_group_card",
    "set_group_name",
    "leave_group",
    "ob11.set_group_special_title",
    "ob11.set_friend_add_request",
    "ob11.set_group_add_request",
    "get_self_info",
    "get_user_info",
    "get_friend_list",
    "get_group_info",
    "get_group_list",
    "get_group_member_info",
    "get_group_member_list",
    "ob11.get_group_honor_info",
    "ob11.get_cookies",
    "ob11.get_csrf_token",
    "ob11.get_credentials",
    "get_file",
    "ob11.can_send_image",
    "ob11.can_send_record",
    "get_status",
    "get_version",
    "ob11.set_restart",
    "ob11.clean_cache",
];

#[inline]
fn remove_field_or_default<'a, T: serde::Deserialize<'a> + Default>(
    map: &mut ValueMap,
    key: &str,
) -> DesResult<T> {
    if let Some(r) = map.remove(key.into()) {
        T::deserialize(r)
    } else {
        Ok(T::default())
    }
}

#[inline]
#[allow(unused)]
fn remove_field<'a, T: serde::Deserialize<'a>>(
    map: &mut ValueMap,
    key: &str,
) -> Option<DesResult<T>> {
    if let Some(r) = map.remove(key.into()) {
        Some(T::deserialize(r))
    } else {
        None
    }
}

#[inline]
fn remove_field_or<'a, T: serde::Deserialize<'a>>(
    map: &mut ValueMap,
    key: &str,
    or: impl FnOnce() -> T,
) -> DesResult<T> {
    if let Some(r) = map.remove(key.into()) {
        T::deserialize(r)
    } else {
        Ok(or())
    }
}

#[__data]
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
                    extra: Default::default(),
                })
            }
            UserInfoResp::StrangerInfo(ob11action::StrangerInfoResp {
                user_id,
                nickname: user_display_name,
                sex,
                age,
            }) => {
                let user_id = user_id.to_string();
                let extra = [
                    (
                        "ob11.sex",
                        serde_value::to_value(sex).map_err(DeserializerError::custom)?,
                    ),
                    ("ob11.age", age.into_value()),
                ]
                .into_map();
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
                    extra: Default::default(),
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
                let extra = [
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
                .into_map();

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
