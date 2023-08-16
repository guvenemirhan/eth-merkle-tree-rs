# Ethereum Merkle Tree Library (`eth-merkle-tree-rs`)
[![Static Badge](https://img.shields.io/badge/crates.io-black?logo=rust&labelColor=blue&link=https%3A%2F%2Fcrates.io%2Fcrates%2Feth_merkle_tree)](https://crates.io/crates/eth_merkle_tree)
[![Static Badge](https://img.shields.io/badge/Docs-black?logo=rust&labelColor=black&link=https%3A%2F%2Fdocs.rs%2Feth_merkle_tree%2F)](https://docs.rs/eth_merkle_tree/)
[![MIT License](https://img.shields.io/badge/License-MIT-green.svg)](https://choosealicense.com/licenses/mit/)

A Rust library and command-line interface (CLI) for working with Ethereum's Merkle tree structure.

## Table of Contents

- [Installation](#installation)
- [Library Usage](#library-usage)
    - [Constructing the Merkle Tree](#constructing-the-merkle-tree)
    - [Visualizing the Tree](#visualizing-the-tree)
- [CLI Usage](#cli-usage)
    - [Visualize the Tree](#visualize-the-tree)
    - [Generate a Merkle Proof](#generate-a-merkle-proof)
-  [Validating a Proof in Solidity](#validating-a-proof-in-solidity) 
-  [License](#license)

## Installation

To include `eth-merkle-tree-rs` in your project, add it to your `Cargo.toml`:

```toml
[dependencies.lib]
petgraph = "0.6"
hex = "0.4"
rust-crypto = "0.2"
```

```toml
[dependencies.bin]
petgraph = "0.6"
hex = "0.4"
rust-crypto = "0.2"
structopt = "0.3"
csv = "1.1"
colored = "2.0"
```

## Library Usage

### Constructing the Merkle Tree

```rust
use eth_merkle_tree::tree::MerkleTree;

let data = vec![
    String::from("0x901Ab22EdCA65188686C9742F2C88c946698bc90, 100"),
    String::from("0x7b95d138cD923476b6e697391DD2aA01D15BAB27, 100"),
];

let tree = MerkleTree::new(data).expect("Tree creation error.");
let root = tree.root.expect("Unable to access root");
println!("Root: {}", root.data);

```
or

```rust
use eth_merkle_tree::tree::MerkleTree;

let data = vec![
    String::from("0x901Ab22EdCA65188686C9742F2C88c946698bc90"),
    String::from("0x7b95d138cD923476b6e697391DD2aA01D15BAB27"),
];

let tree = MerkleTree::new(data).expect("Tree creation error.");
let root = tree.root.expect("Unable to access root");
println!("Root: {}", root.data);

```

### Visualizing the Tree

Can visualize the tree structure using the provided visualization tools:

Note: To visualize the Merkle Tree, ensure that [Graphviz](https://www.graphviz.org/)
is installed on your system.

```rust
use eth_merkle_tree::graph::visualizer::graphviz;

graphviz(&tree).expect("Visualization Error!");
```


## CLI Usage

The library also comes with a command-line interface for interacting with Merkle trees.

### Visualize the Tree

To visualize the tree, use the -v or --visualize flag:


```bash
$ emtr -- ./example.txt -v
```

### Generate a Merkle Proof

To generate a Merkle proof for a specific leaf:

```bash
$ emtr -- ./example.txt --proof 0x901Ab22EdCA65188686C9742F2C88c946698bc90
```

## Validating a proof in Solidity 

Once the proof has been generated, it can be validated in Solidity using MerkleProof as in the following example:

```solidity
pragma solidity >=0.8.0 <0.9.0;

import "@openzeppelin/contracts/utils/cryptography/MerkleProof.sol";

contract Verifier {

  bytes32 private root;

  constructor(bytes32 _root) {
    root = _root;
  }

  function verify(
    bytes32[] memory proof,
    address addr,
    uint256 amount
  ) public view {
    bytes32 leaf = keccak256(abi.encode(addr, amount));
    require(MerkleProof.verify(proof, root, leaf), "Invalid proof");

  }

  function verify(
    bytes32[] memory proof,
    address addr
  ) public view {
    bytes32 leaf = keccak256(abi.encodePacked(addr));
    require(MerkleProof.verify(proof, root, leaf), "Invalid proof");

  }
}
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
