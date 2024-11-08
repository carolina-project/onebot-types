use std::collections::HashMap;

use ob_types_macro::{json, onebot_action};

use crate::scalable_struct;

#[cfg(feature = "json")]
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

#[json(serde(transparent))]
pub struct UploadData(#[serde(with = "data")] Vec<u8>);

#[json(serde(tag = "type", rename_all = "snake_case"))]
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
    Extra {
        r#type: String,
    },
}

#[json]
pub struct FileOpt {
    #[serde(flatten)]
    kind: UploadKind,
    name: String,
    sha256: Option<String>,
}

scalable_struct! {
    FileUploaded = {
        file_id: String,
    },
    FileFragmented = {
        file_id: Option<String>,
    },
    FileResp = {
        #[serde(flatten)]
        file: FileOpt
    },
}

#[json(serde(rename_all = "snake_case", tag = "stage"))]
pub enum FragmentState {
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

#[json(serde(rename_all = "snake_case", tag = "stage"))]
pub enum FragRequest {
    Prepare {
        file_id: String,
    },
    Transfer {
        file_id: String,
        offset: i64,
        size: i64,
    },
}

#[json(serde(untagged))]
pub enum FragReqResult {
    Prepare {
        name: String,
        total_size: i64,
        sha256: Option<String>,
    },
    Transfer {
        data: UploadData,
    },
}

#[json]
pub enum GetFileType {
    Url,
    Path,
    Data,
    #[serde(untagged)]
    Extra(String),
}

scalable_struct! {
    #[onebot_action(FileUploaded)]
    UploadFile = {
        #[serde(flatten)]
        file: FileOpt
    },
    #[onebot_action(FileFragmented)]
    UploadFileFragmented = {
        #[serde(flatten)]
        state: FragmentState,
    },
    #[onebot_action(FileResp)]
    GetFile = {
        file_id: String,
        r#type: GetFileType,
    }
}
