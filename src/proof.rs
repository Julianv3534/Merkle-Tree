use crate::hash::{Hash, hash_data, hash_pair};
use crate::tree::MerkleTree;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProofStep {
    pub hash: Hash,
    pub is_left: bool,
}

/// Inclusion proof for one leaf in a Merkle tree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MerkleProof {
    pub leaf_index: usize,
    pub steps: Vec<ProofStep>,
}

impl MerkleProof {
    /// Verifies that 'data' belongs to a tree with 'expected_root'.
    pub fn verify<T: AsRef<[u8]>>(&self, data: T, expected_root: Hash) -> bool {
        let leaf_hash = hash_data(data.as_ref());
        self.verify_leaf_hash(leaf_hash, expected_root)
    }

    /// Same as 'verify', but accepts a pre-hashed leaf.
    pub fn verify_leaf_hash(&self, leaf_hash: Hash, expected_root: Hash) -> bool {
        let mut current = leaf_hash;

        for step in &self.steps {
            current = if step.is_left {
                hash_pair(&step.hash, &current)
            } else {
                hash_pair(&current, &step.hash)
            };
        }

        current == expected_root
    }
}

impl MerkleTree {
    /// Generates an inclusion proof for the leaf at `leaf_index`

    /// Returns 'None' when the tree is empty or the index is out of bounds.
    pub fn generate_proof(&self, leaf_index: usize) -> Option<MerkleProof> {
        if self.is_empty() || leaf_index >= self.leaf_count {
            return None;
        }

        // Leaves start at index (padded_leaf_count - 1)
        let padded_leaf_count = self.leaf_count.next_power_of_two();
        let mut node_index = (padded_leaf_count - 1) + leaf_index;

        let mut steps = Vec::new();

        while node_index > 0 {
            let sibling_index = MerkleTree::sibling(node_index);
            let sibling_hash = self.get_node(sibling_index).copied()?;

            steps.push(ProofStep {
                hash: sibling_hash,
                is_left: MerkleTree::is_left_child(sibling_index),
            });

            node_index = MerkleTree::parent(node_index);
        }

        Some(MerkleProof { leaf_index, steps })
    }
}
