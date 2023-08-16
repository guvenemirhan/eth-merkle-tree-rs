
use crypto::{digest::Digest, sha3::Sha3};
use crate::utils::errors::BytesError;


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
        Ok(t) => { t }
        Err(e) => {println!("{}", e); return Err(BytesError::KeccakError(value)) }
    };
    let mut hasher = Sha3::keccak256();
    hasher.input(&*hash);
    let mut result = [0u8; 32];
    hasher.result(&mut result);
    let hex_string: Vec<String> = result.iter().map(|b| format!("{:02x}", b)).collect();
    Ok(hex_string.concat())
}

fn encode_packed(addr_str: &str, amount_str: &str) -> String {
    let addr_bytes = hex::decode(&addr_str[2..]).expect("Failed to decode address");
    let mut addr_padded = vec![0u8; 32];
    addr_padded[12..].copy_from_slice(&addr_bytes);
    let amount = u64::from_str_radix(amount_str, 10).expect("Failed to parse amount");
    let mut amount_bytes = [0u8; 32];
    for (i, byte) in amount.to_be_bytes().iter().rev().enumerate() {
        amount_bytes[31 - i] = *byte;
    }
    let mut packed = Vec::new();
    packed.extend_from_slice(&addr_padded);
    packed.extend_from_slice(&amount_bytes);

    packed.iter().map(|byte| format!("{:02x}", byte)).collect()
}
