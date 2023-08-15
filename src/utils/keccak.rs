
use crypto::{digest::Digest, sha3::Sha3};
use crate::utils::errors::BytesError;


pub fn keccak256(input: &str) -> Result<String, BytesError> {

    let mut value  = if input.starts_with("0x") {
        input[2..].to_string()
    } else {
        input.to_string()
    };

    if value.len() % 2 != 0 {
        value.insert(0, '0');
    }

    let hash = match hex::decode(&value) {
        Ok(t) => { t }
        Err(_) => { return Err(BytesError::KeccakError(value)) }
    };

    let mut hasher = Sha3::keccak256();
    hasher.input(&*hash);
    let mut result = [0u8; 32];
    hasher.result(&mut result);
    let hex_string: Vec<String> = result.iter().map(|b| format!("{:02x}", b)).collect();

    Ok(hex_string.concat())
}