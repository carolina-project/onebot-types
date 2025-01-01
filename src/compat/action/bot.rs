use std::collections::VecDeque;

use ob_types_macro::__data;

use super::*;

use crate::{
    base::{MessageChain, RawMessageSeg},
    compat::compat_self,
    ob11,
};

impl<F, E, R> IntoOB11ActionAsync<F> for ob12action::SendMessage
where
    F: (Fn(RawMessageSeg) -> R) + Send,
    E: std::error::Error,
    R: Future<Output = Result<RawMessageSeg, E>> + Send,
{
    type Output = ob11action::SendMsg;

    async fn into_ob11(self, msg_trans_fn: F) -> CompatResult<Self::Output> {
        let message: VecDeque<_> = {
            let mut transformed = VecDeque::new();
            for ele in self.message.into_inner() {
                transformed.push_back(msg_trans_fn(ele).await.map_err(DeserializerError::custom)?);
            }

            transformed
        };

        Ok(ob11action::SendMsg {
            target: self.target.try_into().map_err(DeserializerError::custom)?,
            message: MessageChain::new(message),
        })
    }
}

impl IntoOB11Action for ob12action::DeleteMessage {
    type Output = ob11action::DeleteMsg;
    fn into_ob11(self, _: ()) -> CompatResult<Self::Output> {
        Ok(ob11action::DeleteMsg {
            message_id: self.message_id.parse().map_err(DeserializerError::custom)?,
        })
    }
}

impl FromOB11Resp<f64> for ob12action::SendMessageResp {
    type In = ob11action::MessageResp;

    fn from_ob11(from: Self::In, time: f64) -> CompatResult<Self> {
        Ok(Self {
            message_id: from.message_id.to_string(),
            time,
            extra: Default::default(),
        })
    }
}

#[__data]
pub enum OB11GetFile {
    GetRecord(ob11action::GetRecord),
    GetImage(ob11action::GetImage),
    GetVideo(ob12action::FileOpt),
}

impl TryFrom<OB11GetFile> for ob11action::ActionType {
    type Error = ob12action::FileOpt;

    fn try_from(value: OB11GetFile) -> Result<Self, Self::Error> {
        match value {
            OB11GetFile::GetRecord(record) => Ok(Self::GetRecord(record)),
            OB11GetFile::GetImage(image) => Ok(Self::GetImage(image)),
            OB11GetFile::GetVideo(video) => Err(video),
        }
    }
}

#[derive(Clone, Debug)]
pub enum OB11File {
    Record(String),
    Image(String),
    Video(ob12action::FileOpt),
}

impl<F, R> IntoOB11ActionAsync<F> for ob12action::GetFile
where
    F: (FnOnce(&str) -> R) + Send,
    R: Future<Output = Option<OB11File>> + Send,
{
    type Output = OB11GetFile;

    async fn into_ob11(self, find_fn: F) -> CompatResult<Self::Output> {
        let ob12action::GetFile {
            file_id,
            r#type: _,
            mut extra,
        } = self;
        let Some(ty) = find_fn(&file_id).await else {
            return Err(CompatError::other(format!("cannot find file {}", file_id)));
        };

        Ok(match ty {
            OB11File::Record(file) => OB11GetFile::GetRecord(ob11action::GetRecord {
                file,
                out_format: remove_field_or(&mut extra, "ob11.out_format", || "mp3".into())?,
            }),
            OB11File::Image(file) => OB11GetFile::GetImage(ob11action::GetImage { file }),
            OB11File::Video(file) => OB11GetFile::GetVideo(file),
        })
    }
}

impl IntoOB11Action for ob12action::GetStatus {
    type Output = ob11action::GetStatus;
    fn into_ob11(self, _: ()) -> CompatResult<Self::Output> {
        Ok(ob11action::GetStatus {})
    }
}

impl FromOB11Resp<String> for ob12::BotState {
    type In = ob11::Status;

    fn from_ob11(from: Self::In, self_id: String) -> CompatResult<Self> {
        let extra: ValueMap = from
            .extra
            .into_iter()
            .map(|(k, v)| Ok(("ob11.extra.".to_owned() + &k, v)))
            .collect::<Result<_, DeserializerError>>()?;
        Ok(ob12::BotState {
            self_: compat_self(self_id),
            online: from.online,
            extra,
        })
    }
}

impl IntoOB11Action for ob12action::GetVersion {
    type Output = ob11action::GetVersionInfo;

    fn into_ob11(self, _: ()) -> CompatResult<Self::Output> {
        Ok(ob11action::GetVersionInfo {})
    }
}

impl FromOB11Resp for ob12::VersionInfo {
    type In = ob11action::VersionInfo;

    fn from_ob11(from: Self::In, _: ()) -> CompatResult<Self> {
        let extra: ValueMap = from
            .extra
            .into_iter()
            .map(|(k, v)| ("ob11.extra.".to_owned() + &k, v))
            .collect();
        Ok(ob12::VersionInfo {
            r#impl: "ob11".into(),
            version: from.app_version,
            onebot_version: "12".into(),
            extra,
        })
    }
}
