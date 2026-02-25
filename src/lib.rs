//! # Merkle Tree Library
//!
//! This crate provides a Merkle tree implementation for learning purposes.
//!
//! ## Modules TODO
//! - `tree`: Core Merkle tree structure and building
//! - `proof`: Proof generation and verification (coming soon)
//! - `hash`: Hashing utilities

pub mod tree;
pub mod proof;
pub mod hash;

// Re-export commonly used types at crate root for convenience
pub use hash::Hash;
pub use tree::MerkleTree;
// pub use proof::TODO;