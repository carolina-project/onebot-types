use std::fmt::Display;

use ob_types_base::OBAction;
use ob_types_macro::data;

mod file;
mod group;
mod guild;
mod message;
mod meta;
mod user;

pub use file::*;
pub use group::*;
pub use guild::*;
pub use message::*;
pub use meta::*;
use thiserror::Error;
pub use user::*;

use super::scalable_struct;

#[data]
pub struct Action {
    #[serde(flatten)]
    pub action: ActionType,
    pub echo: Option<String>,
    #[serde(rename = "self")]
    pub self_: Option<super::BotSelf>,
}

#[data]
pub struct ActionTypeRaw {
    pub action: String,
    pub params: serde_value::Value,
}

impl OBAction for ActionTypeRaw {
    type Resp = serde_value::Value;

    fn action_name(&self) -> &str {
        &self.action
    }
}

macro_rules! actions {
    ($($typ:ident),* $(,)?) => {
        #[data]
        #[serde(tag = "action", rename_all = "snake_case", content = "params")]
        pub enum ActionType {
            $(
                $typ(#[serde(default)] $typ),
            )*
            #[serde(untagged)]
            Other(ActionTypeRaw),
        }

        $(impl From<$typ> for ActionType {
            fn from(from: $typ) -> Self {
                Self::$typ(from)
            }
        })*
    };
}

impl From<ActionTypeRaw> for ActionType {
    fn from(value: ActionTypeRaw) -> Self {
        Self::Other(value)
    }
}

actions!(
    // Meta actions
    GetLatestEvents,
    GetSupportedActions,
    GetStatus,
    GetVersion,
    // User actions
    GetSelfInfo,
    GetUserInfo,
    GetFriendList,
    // Message actions
    SendMessage,
    DeleteMessage,
    // Group actions
    GetGroupInfo,
    GetGroupList,
    GetGroupMemberInfo,
    GetGroupMemberList,
    SetGroupName,
    LeaveGroup,
    // Guild actions
    GetGuildInfo,
    GetGuildList,
    SetGuildName,
    GetGuildMemberInfo,
    GetGuildMemberList,
    LeaveGuild,
    GetChannelInfo,
    GetChannelList,
    SetChannelName,
    GetChannelMemberInfo,
    GetChannelMemberList,
    LeaveChannel,
    // File actions
    UploadFile,
    UploadFileFragmented,
    GetFile,
    GetFileFragmented,
);

#[derive(Copy, PartialEq, Eq)]
#[data]
#[serde(rename_all = "lowercase")]
pub enum RespStatus {
    Ok,
    Failed,
}

#[data]
pub struct RespData {
    pub status: RespStatus,
    pub retcode: RetCode,
    pub data: serde_value::Value,
    pub message: String,
    pub echo: Option<String>,
}

impl RespData {
    pub fn is_success(&self) -> bool {
        matches!(self.status, RespStatus::Ok)
    }
}

#[derive(Debug, Clone, Error)]
pub struct RespError {
    pub retcode: RetCode,
    pub message: String,
    pub echo: Option<String>,
}

impl From<RespData> for RespError {
    fn from(value: RespData) -> Self {
        Self {
            retcode: value.retcode,
            message: value.message,
            echo: value.echo,
        }
    }
}

impl Display for RespError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RespError {{ retcode: {}, message: {}, echo: {} }}",
            self.retcode.to_code(),
            self.message,
            self.echo.clone().unwrap_or_else(|| "null".into())
        )
    }
}

scalable_struct! {
    EmptyResp
}

#[derive(Debug, Clone, Copy)]
pub enum RetCode {
    Success, // 0: 成功（OK）

    // 动作请求错误
    BadRequest,             // 1xxxx: 无效的动作请求
    UnsupportedAction,      // 10002: 不支持的动作请求
    BadParam,               // 10003: 无效的动作请求参数
    UnsupportedParam,       // 10004: 不支持的动作请求参数
    UnsupportedSegment,     // 10005: 不支持的消息段类型
    BadSegmentData,         // 10006: 无效的消息段参数
    UnsupportedSegmentData, // 10007: 不支持的消息段参数
    WhoAmI,                 // 10101: 未指定机器人账号
    UnknownSelf,            // 10102: 未知的机器人账号

    // 动作处理器错误
    BadHandler,           // 20001: 动作处理器实现错误
    InternalHandlerError, // 20002: 动作处理器运行时抛出异常

    // 动作执行错误
    DatabaseError(u32),   // 31xxx: 数据库错误
    FilesystemError(u32), // 32xxx: 文件系统错误
    NetworkError(u32),    // 33xxx: 网络错误
    PlatformError(u32),   // 34xxx: 机器人平台错误
    LogicError(u32),      // 35xxx: 动作逻辑错误
    IAmTired(u32),        // 36xxx: 我不想干了

    // 保留错误段
    ReservedError(u32), // 4xxxx、5xxxx: 保留错误段

    // 其它错误段
    OtherError(u32),
}

impl RetCode {
    pub fn from_code(code: u32) -> Self {
        match code {
            0 => Self::Success,
            // 动作请求错误
            10001 => Self::BadRequest,
            10002 => Self::UnsupportedAction,
            10003 => Self::BadParam,
            10004 => Self::UnsupportedParam,
            10005 => Self::UnsupportedSegment,
            10006 => Self::BadSegmentData,
            10007 => Self::UnsupportedSegmentData,
            10101 => Self::WhoAmI,
            10102 => Self::UnknownSelf,
            // 动作处理器错误
            20001 => Self::BadHandler,
            20002 => Self::InternalHandlerError,
            // 动作执行错误
            31000..=31999 => Self::DatabaseError(code),
            32000..=32999 => Self::FilesystemError(code),
            33000..=33999 => Self::NetworkError(code),
            34000..=34999 => Self::PlatformError(code),
            35000..=35999 => Self::LogicError(code),
            36000..=36999 => Self::IAmTired(code),
            // 保留错误段
            40000..=59999 => Self::ReservedError(code),
            // 其它错误段
            code => Self::OtherError(code),
        }
    }

    pub fn to_code(&self) -> u32 {
        match self {
            Self::Success => 0,
            // 动作请求错误
            Self::BadRequest => 10001,
            Self::UnsupportedAction => 10002,
            Self::BadParam => 10003,
            Self::UnsupportedParam => 10004,
            Self::UnsupportedSegment => 10005,
            Self::BadSegmentData => 10006,
            Self::UnsupportedSegmentData => 10007,
            Self::WhoAmI => 10101,
            Self::UnknownSelf => 10102,
            // 动作处理器错误
            Self::BadHandler => 20001,
            Self::InternalHandlerError => 20002,
            // 动作执行错误
            Self::DatabaseError(code) => *code,
            Self::FilesystemError(code) => *code,
            Self::NetworkError(code) => *code,
            Self::PlatformError(code) => *code,
            Self::LogicError(code) => *code,
            Self::IAmTired(code) => *code,
            // 保留错误段
            Self::ReservedError(code) => *code,
            // 其它错误段
            Self::OtherError(code) => *code,
        }
    }
}

macro_rules! from_impl {
    ($($typ:ty),* $(,)?) => {
        $(
        impl From<$typ> for RetCode {
            fn from(value: $typ) -> Self {
                Self::from_code(value as u32)
            }
        }
        )*
    };
}

from_impl!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize);

impl<'de> serde::Deserialize<'de> for RetCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let code = u32::deserialize(deserializer)?;
        Ok(Self::from(code))
    }
}

impl serde::Serialize for RetCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u32(self.to_code())
    }
}
