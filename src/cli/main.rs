use structopt::StructOpt;
use std::error::Error;
use std::path::PathBuf;
use eth_merkle_tree_rs::tree::MerkleTree;
use colored::*;
use petgraph::prelude::*;
use eth_merkle_tree_rs::utils::keccak::keccak256;
use std::fs::File;
use std::io::{BufRead, BufReader};


#[derive(StructOpt, Debug)]
#[structopt(name = "eth-merkle-tree-rs", about = "Ethereum Merkle Tree Tool")]
pub struct Opt {
    #[structopt(short, long)]
    visualize: bool,
    #[structopt(short, long)]
    proof: Option<String>,
    #[structopt(parse(from_os_str))]
    dir: PathBuf,
}

fn main() {
    let opt = Opt::from_args();
    let tree = create_tree(&opt.dir).unwrap_or_else(|_| panic!("{}", "Merkle Tree Creation Error".bright_red()));
    let root = get_root(&tree).unwrap_or_else(|_| panic!("{}", "No root found".bright_red()));
    println!("{}: {}", "Merkle Root".bright_blue(), root.bright_green());
    if opt.visualize {
        let root_node = find_root_node(&tree.graph);
        print_tree(&tree.graph, root_node, "".to_string());
    }
    if let Some(proof_value) = &opt.proof {
        let target_hash = keccak256(proof_value).unwrap_or_else(|_| panic!("{}", "Keccak256 Error!".bright_red()));
        if let Some(index) = tree.locate_leaf(&target_hash) {
             let proof = tree.generate_proof(index);
            println!("{} '{}'= [{}] \n {} = {}", "Merkle proof for".bright_blue(), proof_value.bright_blue(), proof.join(", ").bright_green(), "index".bright_blue(), index.to_string().bright_green());
         } else {
             println!("Leaf not found in the tree");
         }
     }
}

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
    match MerkleTree::new(hex_strings) {
        Ok(tree) => {Ok(tree)}
        Err(e) => {return Err(e)}
    }
}


fn get_root(tree: &MerkleTree) -> Result<String, Box<dyn Error>> {
    match &tree.root {
        Some(root) => {Ok(root.data.clone()) },
        None => Err(Box::try_from("No root found").unwrap()),
    }
}

fn find_root_node(graph: &DiGraph<String, ()>) -> NodeIndex {
    for node in graph.node_indices() {
        if graph.neighbors_directed(node, Incoming).count() == 0 {
            return node;
        }
    }
    panic!("No root node found!");
}


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
