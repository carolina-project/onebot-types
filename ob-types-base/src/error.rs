use std::error::Error;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum OBError {
    #[error(transparent)]
    Custom(Box<dyn Error>),
}

pub type OBResult<T> = Result<T, OBError>;
