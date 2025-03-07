use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use ob_types_macro::{OBAction, __data};

use crate::{scalable_struct, ValueMap};

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
        headers: Option<HashMap<String, String>>,
        url: String,
        #[serde(flatten)]
        extra: ValueMap,
    },
    Path {
        path: String,
        #[serde(flatten)]
        extra: ValueMap,
    },
    Data {
        data: UploadData,
        #[serde(flatten)]
        extra: ValueMap,
    },
    #[serde(untagged)]
    Other {
        r#type: String,
        #[serde(flatten)]
        extra: ValueMap,
    },
}

impl UploadKind {
    pub fn url(headers: Option<HashMap<String, String>>, url: String) -> Self {
        Self::Url {
            headers,
            url,
            extra: Default::default(),
        }
    }

    pub fn path(path: String) -> Self {
        Self::Path {
            path,
            extra: Default::default(),
        }
    }

    pub fn data(data: UploadData) -> Self {
        Self::Data {
            data,
            extra: Default::default(),
        }
    }

    pub fn other(r#type: String) -> Self {
        Self::Other {
            r#type,
            extra: Default::default(),
        }
    }

    pub fn extra(&self) -> &ValueMap {
        match self {
            UploadKind::Url { extra, .. } => extra,
            UploadKind::Path { extra, .. } => extra,
            UploadKind::Data { extra, .. } => extra,
            UploadKind::Other { extra, .. } => extra,
        }
    }

    pub fn extra_mut(&mut self) -> &mut ValueMap {
        match self {
            UploadKind::Url { extra, .. } => extra,
            UploadKind::Path { extra, .. } => extra,
            UploadKind::Data { extra, .. } => extra,
            UploadKind::Other { extra, .. } => extra,
        }
    }
}

#[__data]
pub struct FileOpt {
    #[serde(flatten)]
    pub kind: UploadKind,
    pub name: String,
    #[serde(default)]
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
        #[serde(flatten)]
        extra: ValueMap,
    },
    Transfer {
        file_id: String,
        offset: i64,
        data: UploadData,
        #[serde(flatten)]
        extra: ValueMap,
    },
    Finish {
        file_id: String,
        sha256: Option<String>,
        #[serde(flatten)]
        extra: ValueMap,
    },
}

impl UploadFileReq {
    pub fn prepare(name: String, total_size: i64) -> Self {
        Self::Prepare {
            name,
            total_size,
            extra: Default::default(),
        }
    }

    pub fn transfer(file_id: String, offset: i64, data: UploadData) -> Self {
        Self::Transfer {
            file_id,
            offset,
            data,
            extra: Default::default(),
        }
    }

    pub fn finish(file_id: String, sha256: Option<String>) -> Self {
        Self::Finish {
            file_id,
            sha256,
            extra: Default::default(),
        }
    }

    pub fn extra(&self) -> &ValueMap {
        match self {
            UploadFileReq::Prepare { extra, .. } => extra,
            UploadFileReq::Transfer { extra, .. } => extra,
            UploadFileReq::Finish { extra, .. } => extra,
        }
    }

    pub fn extra_mut(&mut self) -> &mut ValueMap {
        match self {
            UploadFileReq::Prepare { extra, .. } => extra,
            UploadFileReq::Transfer { extra, .. } => extra,
            UploadFileReq::Finish { extra, .. } => extra,
        }
    }
}

#[__data]
#[serde(rename_all = "snake_case", tag = "stage")]
pub enum GetFileReq {
    Prepare {
        #[serde(flatten)]
        extra: ValueMap,
    },
    Transfer {
        offset: i64,
        size: i64,
        #[serde(flatten)]
        extra: ValueMap,
    },
}

impl GetFileReq {
    pub fn prepare() -> Self {
        Self::Prepare {
            extra: Default::default(),
        }
    }

    pub fn transfer(offset: i64, size: i64) -> Self {
        Self::Transfer {
            offset,
            size,
            extra: Default::default(),
        }
    }

    pub fn extra(&self) -> &ValueMap {
        match self {
            GetFileReq::Prepare { extra } => extra,
            GetFileReq::Transfer { extra, .. } => extra,
        }
    }

    pub fn extra_mut(&mut self) -> &mut ValueMap {
        match self {
            GetFileReq::Prepare { extra } => extra,
            GetFileReq::Transfer { extra, .. } => extra,
        }
    }
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
    #[resp(GetFileResp)]
    GetFile = {
        file_id: String,
        r#type: GetFileType,
    },
}

impl GetFile {
    pub fn new(file_id: impl Into<String>) -> Self {
        Self {
            file_id: file_id.into(),
            r#type: GetFileType::Url,
            extra: Default::default(),
        }
    }

    pub fn set_type(&mut self, file_type: GetFileType) {
        self.r#type = file_type;
    }
}

#[__data]
#[derive(OBAction)]
#[action(resp = GetFileFrag, __crate_path = crate)]
pub struct GetFileFragmented {
    pub file_id: String,
    #[serde(flatten)]
    pub req: GetFileReq,
}

impl GetFileFragmented {
    pub fn new(file_id: impl Into<String>, req: GetFileReq) -> Self {
        Self {
            file_id: file_id.into(),
            req,
        }
    }
}

#[__data]
#[derive(OBAction)]
#[action(resp = Uploaded, __crate_path = crate)]
#[serde(transparent)]
pub struct UploadFile(pub FileOpt);

impl UploadFile {
    pub fn new(opt: FileOpt) -> Self {
        Self(opt)
    }
}

#[__data]
#[derive(OBAction)]
#[action(resp = UploadFragmented, __crate_path = crate)]
#[serde(transparent)]
pub struct UploadFileFragmented(pub UploadFileReq);

impl UploadFileFragmented {
    pub fn new(req: UploadFileReq) -> Self {
        Self(req)
    }
}
