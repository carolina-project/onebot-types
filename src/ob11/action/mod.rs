mod bot;
mod friend;
mod group;

pub use bot::*;
pub use friend::*;
pub use group::*;
use ob_types_macro::__data;
use serde::{ser::Error, Deserialize};
use serde_value::{DeserializerError, SerializerError, Value};

use crate::{
    base::ext::{IntoValue, ValueExt},
    ValueMap,
};

pub(crate) type EmptyResp = ();

#[__data]
pub struct RawAction {
    #[serde(flatten)]
    pub detail: ActionDetail,
    pub echo: Option<String>,
}

#[__data]
pub struct ActionDetail {
    pub action: String,
    pub params: ValueMap,
}

macro_rules! actions {
    ($($typ:ident),* $(,)?) => {
        #[__data]
        #[serde(tag = "action", rename_all = "snake_case", content = "params")]
        pub enum ActionType {$(
            $typ($typ),
        )*}

        $(
            impl From<$typ> for ActionType {
                fn from(action: $typ) -> Self {
                    Self::$typ(action)
                }
            }
        )*
    };
}

actions!(
    // from bot.rs
    SendMsg,
    DeleteMsg,
    GetMsg,
    GetForwardMsg,
    GetLoginInfo,
    GetCookies,
    GetCsrfToken,
    GetCredentials,
    GetRecord,
    GetImage,
    CanSendImage,
    CanSendRecord,
    GetStatus,
    GetVersionInfo,
    SetRestart,
    CleanCache,
    // from friend.rs
    SendPrivateMsg,
    SendLike,
    SetFriendAddRequest,
    GetStrangerInfo,
    GetFriendList,
    // from group.rs
    SendGroupMsg,
    SetGroupKick,
    SetGroupBan,
    SetGroupAnonymousBan,
    SetGroupWholeBan,
    SetGroupAdmin,
    SetGroupSpecialTitle,
    SetGroupAnonymous,
    SetGroupName,
    SetGroupLeave,
    SetGroupCard,
    GetGroupMemberInfo,
    GetGroupMemberList,
    GetGroupList,
    GetGroupInfo,
    SetGroupAddRequest,
    GetGroupHonorInfo
);

impl TryFrom<ActionDetail> for ActionType {
    type Error = DeserializerError;

    fn try_from(detail: ActionDetail) -> Result<Self, Self::Error> {
        let ActionDetail { action, params } = detail;
        Deserialize::deserialize(Value::from_map(
            [
                ("action", action.into_value()),
                ("params", Value::from_map(params)),
            ]
            .into(),
        ))
    }
}

impl TryInto<ActionDetail> for ActionType {
    type Error = SerializerError;

    fn try_into(self) -> Result<ActionDetail, Self::Error> {
        ActionDetail::deserialize(serde_value::to_value(self)?).map_err(SerializerError::custom)
    }
}

#[derive(Copy)]
#[__data]
#[serde(rename_all = "lowercase")]
pub enum RespStatus {
    Ok,
    Async,
    Failed,
}

#[__data]
pub struct RespData {
    pub status: RespStatus,
    pub retcode: i64,
    pub data: serde_value::Value,
    pub echo: Option<String>,
}

impl RespData {
    pub fn is_success(&self) -> bool {
        matches!(self.status, RespStatus::Async | RespStatus::Ok)
    }

    pub fn success(data: Value, echo: Option<String>) -> Self {
        Self {
            status: RespStatus::Ok,
            retcode: 0,
            data,
            echo,
        }
    }
}
