use std::fs;
use std::env;
use std::process::Command;
use petgraph::dot::{Config, Dot};
use crate::tree::MerkleTree;


pub fn graphviz(tree: &MerkleTree) -> std::io::Result<()> {
    let graph = &tree.graph;
    let dot = format!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
    let dot_file = "temp.dot";
    fs::write(dot_file, dot).expect("Unable to write to file");
    let output_directory = "./output";
    if !fs::metadata(output_directory).is_ok() {
        fs::create_dir_all(output_directory.clone()).expect("Failed to create directory");
    }
    let current_dir = env::current_dir()?;
    let output_path = current_dir.join("output/merkle_tree.png");
    let output = output_path.to_str().ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, "Failed to convert path to string"))?;

    let status = Command::new("dot")
        .args(&["-Tpng", dot_file, "-o", output])
        .status()
        .expect("Failed to execute command");

    if status.success() {
        println!("PNG file saved to /output/merkle_tree.png");
    } else {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "Command failed"));
    }
    fs::remove_file(dot_file).expect("Failed to remove temporary dot file");
    Ok(())
}