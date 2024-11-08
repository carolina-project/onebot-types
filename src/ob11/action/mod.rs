use ob_types_base::OBRespData;
use ob_types_macro::json;

mod bot;
mod friend;
mod group;

pub use bot::*;
pub use friend::*;
pub use group::*;

pub type EmptyResp = ();

#[json]
pub struct Action {
    #[serde(flatten)]
    pub action: ActionType,
    pub echo: Option<String>,
}

macro_rules! actions {
    ($($typ:ident),* $(,)?) => {
        #[json(serde(tag = "action", rename_all = "snake_case", content = "params"))]
        pub enum ActionType {$(
            $typ($typ),
        )*}
    };
}

actions!(
    // from bot.rs
    SendMessage,
    DeleteMessage,
    GetMessage,
    GetForwardMessage,
    GetLoginInfo,
    GetCookies,
    GetCsrfToken,
    GetCredentials,
    GetRecord,
    GetImage,
    CanSendImage,
    CanSendRecord,
    GetStatus,
    GetVersion,
    SetRestart,
    CleanCache,
    // from friend.rs
    SendPrivateMessage,
    SendLike,
    SetFriendAddRequest,
    GetStrangerInfo,
    GetFriendList,
    // from group.rs
    SendGroupMessage,
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
    SetGroupAddRequest
);

#[derive(Copy)]
#[json(serde(rename_all = "lowercase"))]
pub enum RespStatus {
    Ok,
    Async,
    Failed,
}

#[json]
pub struct RespData<T: OBRespData> {
    pub status: RespStatus,
    pub retcode: i64,
    pub data: T,
    pub echo: Option<String>,
}
