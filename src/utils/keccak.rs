//! # Keccak
//!
//! Provides a set of functions to perform Keccak256 hashing,
//!
//! # Example
//! ```
//!  use eth_merkle_tree::utils::keccak::keccak256;
//!  let input = "0x7b95d138cD923476b6e697391DD2aA01D15BAB27";
//!  let hash = keccak256(input).expect("Failed to hash input");
//!  println!("Keccak256 hash: {}", hash);
//!  ```

use crate::utils::errors::BytesError;
use crypto::{digest::Digest, sha3::Sha3};

/// Computes the Keccak256 hash of the given input.
///
///
/// # Errors
/// Returns a [`BytesError::KeccakError`] if there's an issue during the hashing process.
///
pub fn keccak256(input: &str) -> Result<String, BytesError> {
    let mut value;
    if input.contains(",") {
        let inputs: Vec<&str> = input.split(",").collect();
        value = encode_packed(inputs[0].trim(), inputs[1].trim());
    } else {
        value = if input.starts_with("0x") {
            input[2..].to_string()
        } else {
            input.to_string()
        };

        if value.len() % 2 != 0 {
            value.insert(0, '0');
        }
    }
    let hash = match hex::decode(&value) {
        Ok(t) => t,
        Err(e) => {
            println!("{}", e);
            return Err(BytesError::KeccakError(value));
        }
    };
    let mut hasher = Sha3::keccak256();
    hasher.input(&*hash);
    let mut result = [0u8; 32];
    hasher.result(&mut result);
    let hex_string: Vec<String> = result.iter().map(|b| format!("{:02x}", b)).collect();
    Ok(hex_string.concat())
}

/// Encodes two strings in a packed format.
///
///
/// # Panics
///
/// When there's an error in decoding the address or parsing the amount.
///
pub fn encode_packed(addr_str: &str, amount_str: &str) -> String {
    let addr_bytes = hex::decode(&addr_str[2..]).expect("Failed to decode address");
    let mut addr_padded = vec![0u8; 32];
    addr_padded[12..].copy_from_slice(&addr_bytes);
    let amount: u64 = amount_str.parse().expect("Failed to parse amount");
    let mut amount_bytes = [0u8; 32];
    for (i, byte) in amount.to_be_bytes().iter().rev().enumerate() {
        amount_bytes[31 - i] = *byte;
    }
    let mut packed = Vec::new();
    packed.extend_from_slice(&addr_padded);
    packed.extend_from_slice(&amount_bytes);
    packed.iter().map(|byte| format!("{:02x}", byte)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    /// in solidity
    /// keccak256(abi.encodePacked(0x5B38Da6a701c568545dCfcB03FcB875f56beddC4))
    #[test]
    fn test_keccak256_address() {
        let hash = keccak256("0x5B38Da6a701c568545dCfcB03FcB875f56beddC4").expect("Keccak Error.");
        assert_eq!(
            hash,
            "5931b4ed56ace4c46b68524cb5bcbf4195f1bbaacbe5228fbd090546c88dd229"
        );
    }
    /// in solidity
    /// keccak256(abi.encode(0x5B38Da6a701c568545dCfcB03FcB875f56beddC4, 100))
    #[test]
    fn test_keccak256_address_amount() {
        let hash =
            keccak256("0x5B38Da6a701c568545dCfcB03FcB875f56beddC4, 100").expect("Keccak Error.");
        assert_eq!(
            hash,
            "a6fcf6ee01abd319c4dddfb228856dbc7af3d163647fe57d77640604a69167a4"
        );
    }
    #[should_panic(expected = "Failed to parse amount: ParseIntError { kind: InvalidDigit }")]
    #[test]
    fn test_keccak256_address_amount_failed() {
        let hash =
            keccak256("0x5B38Da6a701c568545dCfcB03FcB875f56beddC4, a").expect("Keccak Error.");
        assert_eq!(hash, "no hash");
    }
    /// in solidity
    /// keccak256(abi.encode(0x5B38Da6a701c568545dCfcB03FcB875f56beddC4, 100))
    #[test]
    fn test_encode_packed() {
        assert_eq!(
            encode_packed("0x5B38Da6a701c568545dCfcB03FcB875f56beddC4","100"),
            "0000000000000000000000005b38da6a701c568545dcfcb03fcb875f56beddc40000000000000000000000000000000000000000000000000000000000000064"
        );
    }
}
