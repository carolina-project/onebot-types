use ob_types_base::ext::{IntoValue, ValueExt};
use ob_types_macro::data;

mod bot;
mod friend;
mod group;

pub use bot::*;
pub use friend::*;
pub use group::*;
use serde::Deserialize;
use serde_value::{DeserializerError, Value};

use crate::ValueMap;

pub(crate) type EmptyResp = ();

#[data]
pub struct Action {
    #[serde(flatten)]
    pub detail: ActionDetail,
    pub echo: Option<String>,
}

#[data]
pub struct ActionDetail {
    pub action: String,
    pub params: ValueMap,
}

macro_rules! actions {
    ($($typ:ident),* $(,)?) => {
        #[data]
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

#[derive(Copy)]
#[data]
#[serde(rename_all = "lowercase")]
pub enum RespStatus {
    Ok,
    Async,
    Failed,
}

#[data]
pub struct RespData {
    pub status: RespStatus,
    pub retcode: i64,
    pub data: serde_value::Value,
    pub echo: Option<String>,
}
