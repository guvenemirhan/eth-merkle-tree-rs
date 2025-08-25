//! # Tree
//!
//! Provides the creation of a Merkle Tree, finding the location of a leaf, and generating a proof for a specific leaf.
//!
extern crate petgraph;

use crate::utils::bytes::hash_pair;
use crate::utils::keccak::keccak256;
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::EdgeRef;

/// Represents a node in the Merkle Tree.
pub struct MerkleNode {
    pub data: String,
}

/// Represents a Merkle Tree structure with its root and graph representation.
pub struct MerkleTree {
    pub root: Option<MerkleNode>,
    pub graph: DiGraph<String, ()>,
}

impl MerkleTree {
    /// Creates a new MerkleTree based on the provided data.
    ///
    /// # Errors
    ///
    /// - When there's a problem hashing the data with `keccak256`.
    /// - When the `hash_pair` function encounters issues.
    pub fn new(data: &[String]) -> Result<Self, Box<dyn std::error::Error>> {
        let mut graph = DiGraph::new();
        let mut nodes: Vec<String> = data
            .iter()
            .map(|node| keccak256(node.as_str()).expect("Keccak Error."))
            .collect();
        let mut previous_layer_indices = Vec::new();

        for node_data in &nodes {
            let index = graph.add_node(node_data.clone());
            previous_layer_indices.push(index);
        }

        while nodes.len() > 1 {
            let mut new_level = Vec::new();
            let mut new_indices = Vec::new();

            for i in (0..nodes.len()).step_by(2) {
                let hashed_data: String = if i + 1 < nodes.len() {
                    hash_pair(nodes[i].as_str(), nodes[i + 1].as_str())?
                } else {
                    nodes[i].clone()
                };
                new_level.push(hashed_data.clone());
                let current_index = graph.add_node(hashed_data);
                if let Some(left_node_index) = previous_layer_indices.get(i) {
                    graph.add_edge(current_index, *left_node_index, ());
                }
                if let Some(right_node_index) = previous_layer_indices.get(i + 1) {
                    graph.add_edge(current_index, *right_node_index, ());
                }
                new_indices.push(current_index);
            }
            nodes = new_level;
            previous_layer_indices = new_indices;
        }

        let root_data = nodes[0].clone();
        let root_node = MerkleNode { data: root_data };

        Ok(MerkleTree {
            root: Some(root_node),
            graph,
        })
    }

    /// Locates the index of a specific leaf based on its hash.
    ///
    /// # Returns
    ///
    /// - `Some(index)` if the leaf with the specified hash is found.
    /// - `None` if the leaf with the specified hash is not found.
    pub fn locate_leaf(&self, target_hash: &String) -> Option<usize> {
        for (index, node_data) in self.graph.raw_nodes().iter().enumerate() {
            if &node_data.weight == target_hash {
                return Some(index);
            }
        }
        None
    }

    /// Generates a proof of inclusion for a specific leaf.
    ///
    /// # Panics
    ///
    /// - When the specified leaf index is out of bounds.
    ///
    /// # Returns
    ///
    /// - A vector containing hashes that make up the proof for the specified leaf.
    pub fn generate_proof(&self, leaf_index: usize) -> Vec<String> {
        let mut proof = Vec::new();
        let mut current_index = leaf_index;

        while let Some(parent_edge) = self
            .graph
            .edges_directed(NodeIndex::new(current_index), petgraph::Incoming)
            .next()
        {
            let parent = parent_edge.source();
            for edge in self.graph.edges_directed(parent, petgraph::Outgoing) {
                if edge.target().index() != current_index {
                    proof.push(format!("0x{}", self.graph[edge.target()].clone()));
                }
            }
            current_index = parent.index();
        }
        proof
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
