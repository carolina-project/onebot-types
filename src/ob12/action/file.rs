use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use ob_types_macro::__data;

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

#[__data]
#[serde(transparent)]
pub struct UploadData(#[cfg_attr(feature = "base64", serde(with = "data"))] pub Vec<u8>);

impl Deref for UploadData {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for UploadData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[__data]
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

#[__data]
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

#[__data]
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

#[__data]
#[serde(rename_all = "snake_case", tag = "stage")]
pub enum GetFileReq {
    Prepare,
    Transfer { offset: i64, size: i64 },
}

#[__data]
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

#[__data]
pub enum GetFileType {
    Url,
    Path,
    Data,
    #[serde(untagged)]
    Other(String),
}

scalable_struct! {
    #[resp(Uploaded)]
    UploadFile = {
        #[serde(flatten)]
        file: FileOpt
    },
    #[resp(UploadFragmented)]
    UploadFileFragmented = {
        #[serde(flatten)]
        state: UploadFileReq,
    },
    #[resp(GetFileResp)]
    GetFile = {
        file_id: String,
        r#type: GetFileType,
    },
    #[resp(GetFileFrag)]
    GetFileFragmented = {
        file_id: String,
        #[serde(flatten)]
        req: GetFileReq,
    },
}
