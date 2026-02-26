use crate::hash::{hash_data, hash_pair, Hash};

#[derive(Debug, Clone)]
pub struct MerkleTree {
    /// Dinamic array so it can grow as needed
    nodes: Vec<Hash>,
    /// Number of original data leaves (before duplication)
    pub leaf_count: usize,
    /// Index where leaves start in the nodes array
    leaf_start_index: usize,
}

impl MerkleTree {
    /// Creates an empty Merkle tree
    pub fn new() -> Self {
        MerkleTree {
            nodes: Vec::new(),
            leaf_count: 0,
            leaf_start_index: 0,
        }
    }

    pub fn from_data<T: AsRef<[u8]>>(data: &[T]) -> Self {
        if data.is_empty() {
            return MerkleTree::new();
        }

        let leaf_count = data.len();

        // Step 1: Hash all data items
        let mut leaves: Vec<Hash> = data
            .iter()
            .map(|item| hash_data(item.as_ref()))
            .collect();

        // Step 2: Pad to power of 2 by duplicating last leaf
        // This ensures a complete binary tree
        let padded_len = leaves.len().next_power_of_two();
        while leaves.len() < padded_len {
            leaves.push(*leaves.last().unwrap());
        }

        // Step 3: Build the tree array
        // Total nodes = 2 * padded_len - 1 (for a complete binary tree)
        // Layout: [internal nodes...][leaves...]
        let total_nodes = 2 * padded_len - 1;
        let leaf_start_index = padded_len - 1; // Internal nodes count
        
        let mut nodes = vec![[0u8; 32]; total_nodes];
        
        // Copy leaves to their positions (end of array)
        for (i, leaf_hash) in leaves.iter().enumerate() {
            nodes[leaf_start_index + i] = *leaf_hash;
        }

        // Build internal nodes bottom-up
        // Start from the last internal node and work backwards to root
        // 
        // Why backwards? Each internal node needs its children to exist first.
        // Children are at higher indices, so we process high→low.
        for i in (0..leaf_start_index).rev() {
            let left_child = 2 * i + 1;
            let right_child = 2 * i + 2;
            nodes[i] = hash_pair(&nodes[left_child], &nodes[right_child]);
        }

        MerkleTree {
            nodes,
            leaf_count,
            leaf_start_index,
        }
    }

    /// Returns the root hash of the tree, or `None` if the tree is empty
    pub fn root_hash(&self) -> Option<Hash> {
        self.nodes.first().copied()
    }

    pub fn is_empty(&self) -> bool {
        self.leaf_count == 0
    }

    pub fn len(&self) -> usize {
        self.leaf_count
    }

    pub fn get_node(&self, index: usize) -> Option<&Hash> {
        self.nodes.get(index)
    }

    #[inline]
    pub fn left_child(index: usize) -> usize {
        2 * index + 1
    }

    #[inline]
    pub fn right_child(index: usize) -> usize {
        2 * index + 2
    }

    #[inline]
    pub fn parent(index: usize) -> usize {
        if index == 0 { 0 } else { (index - 1) / 2 }
    }

    #[inline]
    pub fn is_left_child(index: usize) -> bool {
        index % 2 == 1
    }

    #[inline]
    pub fn is_right_child(index: usize) -> bool {
        index % 2 == 0 && index != 0
    }

    /// Returns the sibling index of the node at `index`
    /// 
    /// - If left child (odd): sibling is index + 1
    /// - If right child (even): sibling is index - 1
    #[inline]
    pub fn sibling(index: usize) -> usize {
        if index == 0 {
            0 // Root has no sibling
        } else if Self::is_left_child(index) {
            index + 1 // Left child's sibling is to the right
        } else {
            index - 1 // Right child's sibling is to the left
        }
    }

    /// Gets the leaf hash at the given leaf index (0-based from data order)
    pub fn get_leaf(&self, leaf_index: usize) -> Option<&Hash> {
        if leaf_index < self.leaf_count {
            self.nodes.get(self.leaf_start_index + leaf_index)
        } else {
            None
        }
    }
}

/// Default implementation creates an empty tree
impl Default for MerkleTree {
    fn default() -> Self {
        Self::new()
    }
}

