use merkle_tree::hash::{hash_data, hash_pair};
use merkle_tree::tree::MerkleTree;

#[test]
fn test_empty_tree() {
    let tree = MerkleTree::new();
    assert!(tree.is_empty());
    assert_eq!(tree.len(), 0);
    assert!(tree.root_hash().is_none());
}

#[test]
fn test_single_leaf() {
    let tree = MerkleTree::from_data(&["hello"]);
    assert_eq!(tree.len(), 1);
    assert!(tree.root_hash().is_some());

    let expected_hash = hash_data(b"hello");
    assert_eq!(tree.root_hash().unwrap(), expected_hash);
}

#[test]
fn test_two_leaves() {
    let tree = MerkleTree::from_data(&["a", "b"]);
    assert_eq!(tree.len(), 2);

    let hash_a = hash_data(b"a");
    let hash_b = hash_data(b"b");
    let expected_root = hash_pair(&hash_a, &hash_b);

    assert_eq!(tree.root_hash().unwrap(), expected_root);
}

#[test]
fn test_four_leaves() {
    let tree = MerkleTree::from_data(&["a", "b", "c", "d"]);
    assert_eq!(tree.len(), 4);
    assert!(tree.root_hash().is_some());
}

#[test]
fn test_odd_leaves_duplicates_last() {
    let tree = MerkleTree::from_data(&["a", "b", "c"]);
    assert_eq!(tree.len(), 3);
    assert!(tree.root_hash().is_some());
}

#[test]
fn test_same_data_same_root() {
    let tree1 = MerkleTree::from_data(&["a", "b", "c"]);
    let tree2 = MerkleTree::from_data(&["a", "b", "c"]);

    assert_eq!(tree1.root_hash(), tree2.root_hash());
}

#[test]
fn test_different_data_different_root() {
    let tree1 = MerkleTree::from_data(&["a", "b", "c"]);
    let tree2 = MerkleTree::from_data(&["a", "b", "d"]);

    assert_ne!(tree1.root_hash(), tree2.root_hash());
}

#[test]
fn test_hash_determinism() {
    let hash1 = hash_data(b"test");
    let hash2 = hash_data(b"test");
    assert_eq!(hash1, hash2);
}

#[test]
fn test_hash_avalanche() {
    let hash1 = hash_data(b"test");
    let hash2 = hash_data(b"Test");

    let differences: u32 = hash1
        .iter()
        .zip(hash2.iter())
        .map(|(a, b)| (a ^ b).count_ones())
        .sum();

    assert!(
        differences > 100,
        "Avalanche effect too weak: {} bits differ",
        differences
    );
}

#[test]
fn test_left_child_index() {
    assert_eq!(MerkleTree::left_child(0), 1);
    assert_eq!(MerkleTree::left_child(1), 3);
    assert_eq!(MerkleTree::left_child(2), 5);
    assert_eq!(MerkleTree::left_child(3), 7);
}

#[test]
fn test_right_child_index() {
    assert_eq!(MerkleTree::right_child(0), 2);
    assert_eq!(MerkleTree::right_child(1), 4);
    assert_eq!(MerkleTree::right_child(2), 6);
    assert_eq!(MerkleTree::right_child(3), 8);
}

#[test]
fn test_parent_index() {
    assert_eq!(MerkleTree::parent(1), 0);
    assert_eq!(MerkleTree::parent(2), 0);
    assert_eq!(MerkleTree::parent(3), 1);
    assert_eq!(MerkleTree::parent(4), 1);
    assert_eq!(MerkleTree::parent(5), 2);
    assert_eq!(MerkleTree::parent(6), 2);
}

#[test]
fn test_is_left_child() {
    assert!(!MerkleTree::is_left_child(0));
    assert!(MerkleTree::is_left_child(1));
    assert!(!MerkleTree::is_left_child(2));
    assert!(MerkleTree::is_left_child(3));
    assert!(!MerkleTree::is_left_child(4));
    assert!(MerkleTree::is_left_child(5));
}

#[test]
fn test_is_right_child() {
    assert!(!MerkleTree::is_right_child(0));
    assert!(!MerkleTree::is_right_child(1));
    assert!(MerkleTree::is_right_child(2));
    assert!(!MerkleTree::is_right_child(3));
    assert!(MerkleTree::is_right_child(4));
    assert!(!MerkleTree::is_right_child(5));
    assert!(MerkleTree::is_right_child(6));
}

#[test]
fn test_sibling_index() {
    assert_eq!(MerkleTree::sibling(1), 2);
    assert_eq!(MerkleTree::sibling(2), 1);
    assert_eq!(MerkleTree::sibling(3), 4);
    assert_eq!(MerkleTree::sibling(4), 3);
    assert_eq!(MerkleTree::sibling(5), 6);
    assert_eq!(MerkleTree::sibling(6), 5);
}

#[test]
fn test_get_leaf() {
    let tree = MerkleTree::from_data(&["a", "b", "c", "d"]);

    assert_eq!(tree.get_leaf(0), Some(&hash_data(b"a")));
    assert_eq!(tree.get_leaf(1), Some(&hash_data(b"b")));
    assert_eq!(tree.get_leaf(2), Some(&hash_data(b"c")));
    assert_eq!(tree.get_leaf(3), Some(&hash_data(b"d")));
    assert_eq!(tree.get_leaf(4), None);
}

#[test]
fn test_tree_structure_four_leaves() {
    let tree = MerkleTree::from_data(&["a", "b", "c", "d"]);

    let h_a = hash_data(b"a");
    let h_b = hash_data(b"b");
    let h_c = hash_data(b"c");
    let h_d = hash_data(b"d");

    assert_eq!(tree.get_node(3), Some(&h_a));
    assert_eq!(tree.get_node(4), Some(&h_b));
    assert_eq!(tree.get_node(5), Some(&h_c));
    assert_eq!(tree.get_node(6), Some(&h_d));

    let h_01 = hash_pair(&h_a, &h_b);
    let h_23 = hash_pair(&h_c, &h_d);
    assert_eq!(tree.get_node(1), Some(&h_01));
    assert_eq!(tree.get_node(2), Some(&h_23));

    let root = hash_pair(&h_01, &h_23);
    assert_eq!(tree.get_node(0), Some(&root));
    assert_eq!(tree.root_hash(), Some(root));
}
