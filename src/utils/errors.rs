//! # Errors
//!
//! Represents potential errors that can arise while handling byte operations.

use std::fmt::Display;

/// # Variants
///
/// * `ComparisonFailed`: Indicates an error occurred during byte comparison.
/// * `ConcatenateError`: Indicates an error occurred during byte concatenation.
/// * `KeccakError`: Indicates an error occurred during Keccak hashing.
///
/// # Examples
///
/// ```
/// use eth_merkle_tree::utils::errors::BytesError;
/// fn some_function() -> Result<(), BytesError> {
///     Err(BytesError::KeccakError(String::from("Reason")))
/// }
/// ```
#[derive(Debug)]
pub enum BytesError {
    ComparisonFailed(String, String),
    ConcatenateError(String, String),
    KeccakError(String),
}

impl Display for BytesError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BytesError::ComparisonFailed(a, b) => {
                write!(f, "Comparison failed between {} and {}", a, b)
            }
            BytesError::ConcatenateError(a, b) => {
                write!(f, "Concatenate error between {} and {}", a, b)
            }
            BytesError::KeccakError(s) => write!(f, "Keccak error for {}", s),
        }
    }
}

impl std::error::Error for BytesError {}
