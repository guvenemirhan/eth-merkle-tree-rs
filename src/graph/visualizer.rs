//! # Graphviz Utility for Merkle Trees
//!
//! Provides utility functions to generate graphical representations of Merkle Trees using the Graphviz.
//!
//! ## Prerequisites
//!
//! * Relies on the external [Graphviz](https://www.graphviz.org/download/) software, specifically the `dot` command.
//!
//! * Ensure that Graphviz is installed on the machine and the `dot` command is accessible from the command line.
//!
//! * The generated visual representation is saved as a PNG file in the `/output` directory.
//!
//! * Note: Before using the utility functions from this module, make sure Graphviz is correctly installed and configured.

use crate::tree::MerkleTree;
use petgraph::dot::{Config, Dot};
use std::env;
use std::fs;
use std::process::Command;

/// Generates a graphical representation of the given `MerkleTree` using Graphviz's `dot` command.
/// # Prerequisites
///
/// This function relies on the external [Graphviz](https://www.graphviz.org/download/) software. Ensure that the `dot` command is
/// accessible from the command line before calling this function.
///
/// # Errors
///
/// - When writing to the intermediate `.dot` file fails.
/// - When creating the output directory fails.
/// - When converting the output path to a string encounters issues.
/// - When the `dot` command execution fails.
///
pub fn graphviz(tree: &MerkleTree) -> std::io::Result<()> {
    let graph = &tree.graph;
    let dot = format!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
    let dot_file = "temp.dot";
    fs::write(dot_file, dot).expect("Unable to write to file");
    let output_directory = "./output";
    if fs::metadata(output_directory).is_err() {
        fs::create_dir_all(output_directory).expect("Failed to create directory");
    }
    let current_dir = env::current_dir()?;
    let output_path = current_dir.join("output/merkle_tree.png");
    let output = output_path.to_str().ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to convert path to string",
        )
    })?;

    let status = Command::new("dot")
        .args(["-Tpng", dot_file, "-o", output])
        .status()
        .expect("Failed to execute command");

    if status.success() {
        println!("PNG file saved to /output/merkle_tree.png");
    } else {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Command failed",
        ));
    }
    fs::remove_file(dot_file).expect("Failed to remove temporary dot file");
    Ok(())
}
