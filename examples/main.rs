extern crate merkle_tree_visualizer;

use std::error::Error;
use merkle_tree_visualizer::graph::visualizer::graphviz;
use merkle_tree_visualizer::tree::{MerkleNode, MerkleTree};
use merkle_tree_visualizer::utils::keccak::keccak256;

fn main() {
    let data = vec![
        String::from("0x901Ab22EdCA65188686C9742F2C88c946698bc90"),
        String::from("0x7b95d138cD923476b6e697391DD2aA01D15BAB27"),
        String::from("0xaBA8e3eB6D782e3B85Aa1Dd6E5B07136D4F98236"),
        String::from("0x519cD54891B30157f526485CCA49e9D0fa32BD86"),
        String::from("0xBd5760bf0A1cA1879881351018383c00B126e23D"),
        String::from("0x71a40d4D0110c99fe2f804378DD21D6aed50FFe8"),
        String::from("0x5a3281D2d5b81C0c6591627617d6374fF6D8AD63"),
        String::from("0xb1397d10bd332dbe3b0009DFB1732D86F9dF5653"),
        String::from("0xcD7Ee7cb8A87816ddb21Caec344767Ca8D51902b"),
        String::from("0x110d697D5921d22c3C581eCd660dfb0Cd00d0212"),
        String::from("0x6Ffa3Ff180c26F58aE21aDD80Dd6D3C971c22c6D"),
        String::from("0xd1D0DeD9Bd888F4754CB2fdA8B3250b8b06ac2aF"),
        String::from("0x86015C5C3d6a882B025FA7428BF784B2dAd8e0CE"),
        String::from("0x5271089D698fab4C6400d3BF53b0e9Bd947A5592"),
        String::from("0x324152a714E266f85dBfbeEDe0CE6F1f91D8346f"),
        String::from("0x667aC3f4283aa327D34F8E62742E4759F6ff9E72"),
        String::from("0xEcaaDb6B56601CA05030647dCA9fAaf6426F8FB0"),
        String::from("0xB184FEd855c51245711Ee4F5A3b13B928aE9a9A6"),
        String::from("0x8C8FaB115003EAff16cfF187aFb83c328E50206d"),
    ];

    let tree = create_tree(&data);
    let root = get_root(&tree).expect("Unable to access root");
    println!("Root: {}", root);

    visualize(&tree);

    let target_hash = keccak256(&data[0]).expect("Keccak error.");

    // Getting proof
    if let Some(index) = tree.locate_leaf(&target_hash) {
        let proof = tree.generate_proof(index);
        println!("Merkle proof for {}: {:?} index: {}", target_hash, proof, index);
    } else {
        println!("Leaf not found in the tree");
    }
}

fn create_tree(data: &Vec<String>) -> MerkleTree {
    MerkleTree::new(data.clone()).expect("Tree creation error.")
}

fn get_root(tree: &MerkleTree) -> Result<String, Box<dyn Error>> {
    match &tree.root {
        Some(root) => {Ok(root.data.clone()) },
        None => Err(Box::try_from("No root found").unwrap()),
    }
}

fn visualize(tree: &MerkleTree)  {
    graphviz(tree);
}