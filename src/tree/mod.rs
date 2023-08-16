extern crate petgraph;

use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::EdgeRef;
use crate::utils::bytes::hash_pair;
use crate::utils::keccak::keccak256;

pub struct MerkleNode {
    pub data: String,
}

pub struct MerkleTree {
    pub root: Option<MerkleNode>,
    pub graph: DiGraph<String, ()>
}


impl MerkleTree {
    pub fn new(data: Vec<String>) -> Result<Self, Box<dyn std::error::Error>> {
        let mut graph = DiGraph::new();
        let mut nodes: Vec<String> =
            data.into_iter().map(|node|
                keccak256(node.as_str()).expect("Keccak Error.")
            ).collect();
        let mut previous_layer_indices = Vec::new();

        for node_data in &nodes {
            let index = graph.add_node(node_data.clone());
            previous_layer_indices.push(index);
        }

        while nodes.len() > 1 {
            let mut new_level = Vec::new();
            let mut new_indices = Vec::new();

            for i in (0..nodes.len()).step_by(2) {

                let hashed_data: String =  if i + 1 < nodes.len() {
                    hash_pair(nodes[i].as_str(), nodes[i+1].as_str())?
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
        let root_node = MerkleNode {
            data: root_data,
        };

        Ok(MerkleTree { root: Some(root_node), graph })
    }

    pub fn locate_leaf(&self, target_hash: &String) -> Option<usize> {
        for (index, node_data) in self.graph.raw_nodes().iter().enumerate() {
            if &node_data.weight == target_hash {
                return Some(index);
            }
        }
        None
    }

    pub fn generate_proof(&self, leaf_index: usize) -> Vec<String> {
        let mut proof = Vec::new();
        let mut current_index = leaf_index;

        while let Some(parent_edge) = self.graph.edges_directed(NodeIndex::new(current_index), petgraph::Incoming).next() {
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