use std::collections::HashMap;

use ob_types_macro::{data, onebot_action};

use crate::scalable_struct;

#[cfg(feature = "base64")]
mod data {
    use base64::prelude::*;
    use serde::Deserialize;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let b64 = String::deserialize(deserializer)?;
        BASE64_STANDARD
            .decode(b64)
            .map_err(serde::de::Error::custom)
    }

    pub fn serialize<S>(input: &Vec<u8>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&BASE64_STANDARD.encode(input))
    }
}

#[data]
#[serde(transparent)]
pub struct UploadData(#[cfg_attr(feature = "base64", serde(with = "data"))] pub Vec<u8>);

#[data]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum UploadKind {
    Url {
        headers: HashMap<String, String>,
        url: String,
    },
    Path {
        path: String,
    },
    Data {
        data: UploadData,
    },
    #[serde(untagged)]
    Other {
        r#type: String,
    },
}

#[data]
pub struct FileOpt {
    #[serde(flatten)]
    pub kind: UploadKind,
    pub name: String,
    pub sha256: Option<String>,
}

scalable_struct! {
    Uploaded = {
        file_id: String,
    },
    UploadFragmented = {
        file_id: Option<String>,
    },
    GetFileResp = {
        #[serde(flatten)]
        file: FileOpt
    },
}

#[data]
#[serde(rename_all = "snake_case", tag = "stage")]
pub enum UploadFileReq {
    Prepare {
        name: String,
        total_size: i64,
    },
    Transfer {
        file_id: String,
        offset: i64,
        data: UploadData,
    },
    Finish {
        file_id: String,
        sha256: Option<String>,
    },
}

#[data]
#[serde(rename_all = "snake_case", tag = "stage")]
pub enum GetFileReq {
    Prepare,
    Transfer { offset: i64, size: i64 },
}

#[data]
#[serde(untagged)]
pub enum GetFileFrag {
    Prepare {
        name: String,
        total_size: i64,
        sha256: Option<String>,
    },
    Transfer {
        data: UploadData,
    },
}

#[data]
pub enum GetFileType {
    Url,
    Path,
    Data,
    #[serde(untagged)]
    Other(String),
}

scalable_struct! {
    #[onebot_action(Uploaded)]
    UploadFile = {
        #[serde(flatten)]
        file: FileOpt
    },
    #[onebot_action(UploadFragmented)]
    UploadFileFragmented = {
        #[serde(flatten)]
        state: UploadFileReq,
    },
    #[onebot_action(GetFileResp)]
    GetFile = {
        file_id: String,
        r#type: GetFileType,
    },
    #[onebot_action(GetFileFrag)]
    GetFileFragmented = {
        file_id: String,
        #[serde(flatten)]
        req: GetFileReq,
    },
}
