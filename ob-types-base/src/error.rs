use thiserror::Error;

#[derive(Error, Debug)]
pub enum OBError {
    #[cfg(feature = "json")]
    #[error(transparent)]
    Serde(#[from] serde_json::Error)
}

pub type OBResult<T> = Result<T, OBError>;
