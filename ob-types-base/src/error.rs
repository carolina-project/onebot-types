use thiserror::Error;

#[derive(Error, Debug)]
pub enum OBError {
    #[error(transparent)]
    #[cfg(not(target_arch = "wasm32"))]
    Data(#[from] serde_json::Error),
}

pub type OBResult<T> = Result<T, OBError>;
