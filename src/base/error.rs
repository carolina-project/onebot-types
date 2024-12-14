use std::fmt::Display;

#[derive(Debug)]
pub struct TypeMismatchError {
    pub expected: String,
    pub got: String,
}

impl TypeMismatchError {
    pub fn new(expected: impl Display, got: impl Display) -> Self {
        Self {
            expected: expected.to_string(),
            got: got.to_string(),
        }
    }
}

impl std::fmt::Display for TypeMismatchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "expected type {}, got {}", self.expected, self.got)
    }
}
