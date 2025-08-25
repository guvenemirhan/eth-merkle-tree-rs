use crate::utils::{bytes::hash_pair, errors::BytesError, keccak::keccak256};
pub fn verify_proof(proof: Vec<String>, root: &str, leaf_data: String) -> Result<bool, BytesError> {
    let leaf_hash = keccak256(&leaf_data)?;
    proof
        .iter()
        .map(|h| h[2..].to_string())
        .try_fold(leaf_hash.clone(), |acc, sibling_hash| {
            hash_pair(&acc, &sibling_hash)
        })
        .map(|computed_root| computed_root == root)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree::MerkleTree;
    #[test]
    fn test_verify_proof_singleton() {
        let data = vec!["0xabc".to_string()];
        let tree = MerkleTree::new(&data).expect("Failed to create Merkle Tree");
        let root = tree.root.as_ref().expect("No root found").data.clone();
        let leaf_hash = keccak256(&data[0]).expect("Keccak error.");
        let leaf_index = tree.locate_leaf(&leaf_hash).expect("Failed to locate leaf");
        let proof = tree.generate_proof(leaf_index);
        let result = verify_proof(proof, &root, data[0].clone());
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_verify_proof_two_elements() {
        let data = vec!["0xabc".to_string(), "0xdef".to_string()];
        let tree = MerkleTree::new(&data).expect("Failed to create Merkle Tree");
        let root = tree.root.as_ref().expect("No root found").data.clone();
        for i in 0..data.len() {
            let leaf_hash = keccak256(&data[i]).expect("Keccak error.");
            let leaf_index = tree.locate_leaf(&leaf_hash).expect("Failed to locate leaf");
            let proof = tree.generate_proof(leaf_index);
            let result = verify_proof(proof, &root, data[i].clone());
            assert!(result.is_ok());
            assert!(result.unwrap());
        }
    }

    #[test]
    fn test_verify_proof_complex() {
        let data = vec![
            "0xabc".to_string(),
            "0xdef".to_string(),
            "0x123".to_string(),
            "0x456".to_string(),
            "0x789".to_string(),
        ];
        let tree = MerkleTree::new(&data).expect("Failed to create Merkle Tree");
        let root = tree.root.as_ref().expect("No root found").data.clone();
        for i in 0..data.len() {
            let leaf_hash = keccak256(&data[i]).expect("Keccak error.");
            let leaf_index = tree.locate_leaf(&leaf_hash).expect("Failed to locate leaf");
            let proof = tree.generate_proof(leaf_index);
            let result = verify_proof(proof, &root, data[i].clone());
            assert!(result.is_ok());
            assert!(result.unwrap());
        }
    }
}
