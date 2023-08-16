use crate::utils::errors::BytesError;
use crate::utils::keccak::keccak256;

pub fn hash_pair(a: &str, b: &str) -> Result<String, BytesError> {
    let sorted = match compare_bytes(a, b) {
        Ok(t) => {
            if t == std::cmp::Ordering::Greater {
                (b, a)
            } else {
                (a, b)
            }
        }
        Err(_) => { return Err(BytesError::ComparisonFailed(a.to_string(), b.to_string())); }
    };
    let concatenated = match concat_hex_strings(sorted.0, sorted.1) {
        Ok(t) => { format!("0x{}", t) }
        Err(_) => { return Err(BytesError::ConcatenateError(a.to_string(), b.to_string()))}
    };
    keccak256(concatenated.as_str())
}

fn concat_hex_strings(a: &str, b: &str) -> Result<String, hex::FromHexError> {
    let a_bytes = hex::decode(a)?;
    let b_bytes = hex::decode(b)?;

    let mut concatenated = a_bytes;
    concatenated.extend(b_bytes);

    Ok(hex::encode(concatenated))
}

fn compare_bytes(a: &str, b: &str) -> Result<std::cmp::Ordering, hex::FromHexError> {
    let a_bytes = hex::decode(a)?;
    let b_bytes = hex::decode(b)?;

    Ok(a_bytes.cmp(&b_bytes))
}

