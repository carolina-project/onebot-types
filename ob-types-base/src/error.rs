use std::error::Error;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum OBError {
    #[error(transparent)]
    Custom(Box<dyn Error>),
}

#[derive(Debug)]
pub struct TypeMismatchError {
    pub expected: String,
    pub got: String,
}

impl std::fmt::Display for TypeMismatchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "expected type {}, got {}", self.expected, self.got)
    }
}

pub type OBResult<T> = Result<T, OBError>;
