//! # Cli
//!
//! Command line interface for the Ethereum Merkle tree library.

use colored::*;
use eth_merkle_tree::tree::MerkleTree;
use eth_merkle_tree::utils::keccak::keccak256;
use petgraph::prelude::*;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "eth-merkle-tree-rs", about = "Ethereum Merkle Tree Tool")]
pub struct Opt {
    /// Activate tree visualization
    #[structopt(short, long)]
    visualize: bool,

    /// Provide the leaf for proof retrieval.
    #[structopt(short, long)]
    proof: Option<String>,

    /// Directory Path
    #[structopt(parse(from_os_str))]
    dir: PathBuf,
}

/// Takes user-provided arguments, constructs a Merkle Tree, and prints the results to the console.
///
/// # Examples
///
/// An example of the input format is provided in the example.txt
///
/// When run from the command line:
/// ```text
/// $ emtr ./example.txt --proof 0x7b95d138cD923476b6e697391DD2aA01D15BAB27 -v
/// ```
///
/// # Panics
///
/// When given an input in the incorrect format.
///
/// When provided with an invalid argument.
///
fn main() {
    let opt = Opt::from_args();
    let tree = create_tree(&opt.dir)
        .unwrap_or_else(|_| panic!("{}", "Merkle Tree Creation Error".bright_red()));
    let root = get_root(&tree).unwrap_or_else(|_| panic!("{}", "No root found".bright_red()));
    println!("{}: {}", "Merkle Root".bright_blue(), root.bright_green());
    if let Some(proof_value) = &opt.proof {
        let target_hash = keccak256(proof_value)
            .unwrap_or_else(|_| panic!("{}", "Keccak256 Error!".bright_red()));
        if let Some(index) = tree.locate_leaf(&target_hash) {
            let proof = tree.generate_proof(index);
            println!(
                "{} '{}'= [{}] \n {} = {}",
                "Merkle proof for".bright_blue(),
                proof_value.bright_blue(),
                proof.join(", ").bright_green(),
                "index".bright_blue(),
                index.to_string().bright_green()
            );
        } else {
            println!("Leaf not found in the tree");
        }
    }
    if opt.visualize {
        let root_node = find_root_node(&tree.graph);
        print_tree(&tree.graph, root_node, "".to_string());
    }
}

/// Returns a MerkleTree from the provided file path.
///
///
/// # Arguments
///
/// * `path` - A PathBuf pointing to the location of the file to be processed.
///
/// # Returns
///
/// * `Result<MerkleTree, Box<dyn Error>>` - A `Result` containing the constructed `MerkleTree`
///   if successful, or an error if the operation fails.
///
/// # Errors
///
/// * The file specified by `path` isn't found or read.
/// * The file contents aren't in the expected format for constructing a MerkleTree.
///
/// # Example
///
/// ```ignore
/// let path = PathBuf::from("example.txt");
/// let tree = create_tree(&path)?;
/// ```
///
fn create_tree(path: &PathBuf) -> Result<MerkleTree, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut hex_strings: Vec<String> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if !line.trim().is_empty() {
            hex_strings.push(line);
        }
    }
    MerkleTree::new(&hex_strings)
}

/// Returns the root of the specified `tree` argument.
/// If no root is present in the tree, it returns an error.
///
/// # Arguments
///
/// * `tree` - A reference to the MerkleTree from which the root value will be retrieved.
///
/// # Returns
///
/// * `Result<String, Box<dyn Error>>` - A `Result` containing the root value of the `tree` if successful,
///   otherwise returns an error.
///
/// # Errors
///
/// When there's no root value present in the `tree`.
///
/// # Example
///
/// ```ignore
/// let tree: MerkleTree = merkle_tree;
/// let root_value = get_root(&tree)?;
/// ```
///
fn get_root(tree: &MerkleTree) -> Result<String, Box<dyn Error>> {
    match &tree.root {
        Some(root) => Ok(root.data.clone()),
        None => Err(Box::from("No root found")),
    }
}

/// Finds and returns the root node from a given directed graph.
///
///
/// # Arguments
///
/// * `graph` - A reference to the directed graph (`DiGraph`) where the root node search will be conducted.
///
/// # Returns
///
/// * `NodeIndex` - The index of the identified root node within the `graph`.
///
/// # Panics
///
/// When there's no node within the `graph` that qualifies as the root node (i.e., no node with zero incoming edges).
///
/// # Example
///
/// ```ignore
/// let graph: DiGraph<String, ()> = tree.graph;
/// let root_node_index = find_root_node(&my_graph);
/// ```
///
fn find_root_node(graph: &DiGraph<String, ()>) -> NodeIndex {
    for node in graph.node_indices() {
        if graph.neighbors_directed(node, Incoming).count() == 0 {
            return node;
        }
    }
    panic!("No root node found!");
}

/// Prints the structure of the tree represented by a directed graph in a visual manner.
///
///
/// # Arguments
///
/// * `graph` - A reference to the directed graph (`DiGraph`) representing the tree.
/// * `root` - The starting point (`NodeIndex`) from which the tree printing begins. Typically, this is the root node.
/// * `prefix` - A `String` that serves as the prefix for printing and visually indicates the depth of the current node in the tree.
///
/// # Usage
///
/// This function is generally used for visualization purposes, especially during debugging or when presenting the structure
/// of a tree to a user in a clear and hierarchical manner.
///
/// # Example
///
/// ```ignore
/// let graph: DiGraph<String, ()> = tree.graph;
/// let root_node_index = find_root_node(&my_graph);
/// print_tree(&graph, root_node_index, "".to_string());
/// ```
///
fn print_tree(graph: &DiGraph<String, ()>, root: NodeIndex, prefix: String) {
    let children: Vec<_> = graph.neighbors_directed(root, Outgoing).collect();
    println!("{}", graph[root].bright_green());

    for (i, child) in children.iter().enumerate() {
        let is_last = i == children.len() - 1;
        let next_prefix = if is_last { "   " } else { "│  " };
        let branch = if is_last { "└─ " } else { "├─ " };
        print!("{}{}", prefix.bright_blue(), branch.bright_blue());
        print_tree(graph, *child, prefix.to_string() + next_prefix);
    }
}
