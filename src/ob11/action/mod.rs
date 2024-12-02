use ob_types_base::OBRespData;
use ob_types_macro::data;

mod bot;
mod friend;
mod group;

pub use bot::*;
pub use friend::*;
pub use group::*;

pub(crate) type EmptyResp = ();

#[data]
pub struct Action {
    #[serde(flatten)]
    pub action: ActionType,
    pub echo: Option<String>,
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

#[derive(Copy)]
#[data]
#[serde(rename_all = "lowercase")]
pub enum RespStatus {
    Ok,
    Async,
    Failed,
}

#[data]
pub struct RespData<T: OBRespData> {
    pub status: RespStatus,
    pub retcode: i64,
    pub data: T,
    pub echo: Option<String>,
}
