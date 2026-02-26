//Merkle Tree Library

//This crate provides a Merkle tree implementation for learning purposes.

// Modules
//'tree': Core Merkle tree structure and building
//'proof': Proof generation and verification
//'hash': Hashing utilities

pub mod hash;
pub mod proof;
pub mod tree;

// Re-export commonly used types at crate root for convenience
pub use hash::Hash;
pub use proof::{MerkleProof, ProofStep};
pub use tree::MerkleTree;