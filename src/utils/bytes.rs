//! # Bytes
//!
//! Provides utility functions for handling and manipulating hexadecimal strings representing byte data.
//!

use crate::utils::errors::BytesError;
use crate::utils::keccak::keccak256;

/// Hashes a pair of hexadecimal strings after sorting them in lexicographic order.
///
///
/// # Arguments
///
/// * `a` - The first hexadecimal string.
/// * `b` - The second hexadecimal string.
///
/// # Returns
///
/// Returns a `Result` containing the hexadecimal hash string, or a `BytesError` on failure.
///
pub fn hash_pair(a: &str, b: &str) -> Result<String, BytesError> {
    let sorted = match compare_bytes(a, b) {
        Ok(t) => {
            if t == std::cmp::Ordering::Greater {
                (b, a)
            } else {
                (a, b)
            }
        }
        Err(_) => {
            return Err(BytesError::ComparisonFailed(a.to_string(), b.to_string()));
        }
    };
    let concatenated = match concat_hex_strings(sorted.0, sorted.1) {
        Ok(t) => {
            format!("0x{}", t)
        }
        Err(_) => return Err(BytesError::ConcatenateError(a.to_string(), b.to_string())),
    };
    keccak256(concatenated.as_str())
}

/// Concatenates two hexadecimal strings.
///
///
/// # Arguments
///
/// * `a` - The first hexadecimal string.
/// * `b` - The second hexadecimal string.
///
/// # Returns
///
/// Returns a `Result` containing the concatenated hexadecimal string.
///
/// # Errors
///
/// Returns a `hex::FromHexError` if there's an error during the decoding of the hexadecimal strings.
///
pub fn concat_hex_strings(a: &str, b: &str) -> Result<String, hex::FromHexError> {
    let a_bytes = hex::decode(a)?;
    let b_bytes = hex::decode(b)?;

    let mut concatenated = a_bytes;
    concatenated.extend(b_bytes);

    Ok(hex::encode(concatenated))
}

/// Compares two hexadecimal strings lexicographically.
///
///
/// # Arguments
///
/// * `a` - The first hexadecimal string.
/// * `b` - The second hexadecimal string.
///
/// # Returns
///
/// Returns a `Result` containing the comparison result (`Ordering`).
///
/// # Errors
///
/// Returns a `hex::FromHexError` if there's an error during the decoding of the hexadecimal strings.
///
pub fn compare_bytes(a: &str, b: &str) -> Result<std::cmp::Ordering, hex::FromHexError> {
    let a_bytes = hex::decode(a)?;
    let b_bytes = hex::decode(b)?;

    Ok(a_bytes.cmp(&b_bytes))
}

#[cfg(test)]
mod tests {
    use super::*;

    // in solidity
    // keccak256(abi.encodePacked(address1, address2));
    #[test]
    fn test_hash_pair() {
        let address = "5B38Da6a701c568545dCfcB03FcB875f56beddC4";
        let address2 = "901Ab22EdCA65188686C9742F2C88c946698bc90";
        let result = hash_pair(address, address2).expect("Hash Pair Error");
        assert_eq!(
            result,
            "4f1bf293e60209d407901e10ebbdb0da6faaf7ae860c3816321563708f6619bf"
        )
    }
}
