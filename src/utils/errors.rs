use std::fmt::{Display};

#[derive(Debug)]
pub enum BytesError {
    ComparisonFailed(String, String),
    ConcatenateError(String, String),
    KeccakError(String)
}

impl Display for BytesError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BytesError::ComparisonFailed(a, b) => write!(f, "Comparison failed between {} and {}", a, b),
            BytesError::ConcatenateError(a, b) => write!(f, "Concatenate error between {} and {}", a, b),
            BytesError::KeccakError(s) => write!(f, "Keccak error for {}", s),
        }
    }
}

impl std::error::Error for BytesError {}
