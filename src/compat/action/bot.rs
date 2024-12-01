use std::time::Duration;

use super::*;

use crate::compat::{compat_self, default_obj};

impl<F, E> IntoOB11Action<F> for ob12action::SendMessage
where
    F: Fn(ob12::MessageSeg) -> Result<ob11::MessageSeg, E>,
    E: std::error::Error,
{
    type Output = ob11action::SendMsg;

    fn into_ob11(self, msg_trans_fn: F) -> DesResult<Self::Output> {
        let message: Vec<_> = {
            let ob12::message::MessageChain::Array(arr) = self.message else {
                unimplemented!("cq code string")
            };

            arr.into_iter()
                .map(|e| msg_trans_fn(e).map_err(DeError::custom))
                .collect::<Result<_, DeserializerError>>()?
        };

        Ok(ob11action::SendMsg {
            target: self.target.try_into().map_err(DeserializerError::custom)?,
            message: ob11::message::MessageChain::Array(message),
        })
    }
}

impl IntoOB11Action for ob12action::DeleteMessage {
    type Output = ob11action::DeleteMsg;
    fn into_ob11(self, _: ()) -> DesResult<Self::Output> {
        Ok(ob11action::DeleteMsg {
            message_id: self.message_id.parse().map_err(DeserializerError::custom)?,
        })
    }
}

impl FromOB11Resp<Duration> for ob12action::SendMessageResp {
    type In = ob11action::MessageResp;

    fn from_ob11(from: Self::In, time: Duration) -> DesResult<Self> {
        Ok(Self {
            message_id: from.message_id.to_string(),
            time,
            extra: default_obj(),
        })
    }
}

pub enum FileType {
    Record(String),
    Image(String),
    Unknown,
}

#[data]
pub enum OB11GetFile {
    GetRecord(ob11action::GetRecord),
    GetImage(ob11action::GetImage),
}

impl From<OB11GetFile> for ob11action::ActionType {
    fn from(value: OB11GetFile) -> Self {
        match value {
            OB11GetFile::GetRecord(record) => Self::GetRecord(record),
            OB11GetFile::GetImage(image) => Self::GetImage(image),
        }
    }
}

type DetectFn = Box<dyn FnOnce(&str) -> FileType>;
impl IntoOB11Action<DetectFn> for ob12action::GetFile {
    type Output = OB11GetFile;

    fn into_ob11(self, detect_fn: DetectFn) -> DesResult<Self::Output> {
        let file_type: FileType = detect_fn(&self.file_id);
        let mut extra = unwrap_value_map(self.extra)?;

        Ok(match file_type {
            FileType::Record(file) => OB11GetFile::GetRecord(ob11action::GetRecord {
                file,
                out_format: remove_field(&mut extra, "ob11.out_format")
                    .unwrap_or_else(|| Ok(Value::String("mp3".into())))?
                    .try_into_string()
                    .ok_or_else(|| DeserializerError::custom("invalid type"))?,
            }),
            FileType::Image(file) => OB11GetFile::GetImage(ob11action::GetImage { file }),
            FileType::Unknown => return Err(DeserializerError::custom("unknown file type")),
        })
    }
}

impl IntoOB11Action for ob12action::GetStatus {
    type Output = ob11action::GetStatus;
    fn into_ob11(self, _: ()) -> DesResult<Self::Output> {
        Ok(ob11action::GetStatus {})
    }
}

impl FromOB11Resp<String> for ob12::BotState {
    type In = ob11action::Status;
    fn from_ob11(from: Self::In, self_id: String) -> DesResult<Self> {
        let extra: ValueMap = unwrap_value_map(from.extra)?
            .into_iter()
            .map(|(k, v)| {
                let k = k
                    .try_into_string()
                    .ok_or_else(|| DeserializerError::custom("invalid type, expected string"))?;
                Ok((Value::String("ob11.extra.".to_owned() + &k), v))
            })
            .collect::<Result<_, DeserializerError>>()?;
        Ok(ob12::BotState {
            self_: compat_self(self_id),
            online: from.online,
            extra: extra.into_value(),
        })
    }
}

impl IntoOB11Action for ob12action::GetVersion {
    type Output = ob11action::GetVersionInfo;

    fn into_ob11(self, _: ()) -> DesResult<Self::Output> {
        Ok(ob11action::GetVersionInfo {})
    }
}

impl FromOB11Resp for ob12::VersionInfo {
    type In = ob11action::VersionInfo;

    fn from_ob11(from: Self::In, _: ()) -> DesResult<Self> {
        let extra: ValueMap = unwrap_value_map(from.extra)?
            .into_iter()
            .map(|(k, v)| {
                let k = k
                    .try_into_string()
                    .ok_or_else(|| DeserializerError::custom("invalid type, expected string"))?;
                Ok((Value::String("ob11.extra.".to_owned() + &k), v))
            })
            .collect::<Result<_, DeserializerError>>()?;
        Ok(ob12::VersionInfo {
            r#impl: "ob11".into(),
            version: from.app_version,
            onebot_version: "12".into(),
            extra: extra.into_value()
        })
    }
}
