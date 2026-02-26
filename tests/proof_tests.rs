use merkle_tree::tree::MerkleTree;

#[test]
fn test_generate_proof_empty_tree() {
    let tree = MerkleTree::new();
    assert!(tree.generate_proof(0).is_none());
}

#[test]
fn test_generate_proof_out_of_bounds() {
    let tree = MerkleTree::from_data(&["a", "b"]);
    assert!(tree.generate_proof(2).is_none());
}

#[test]
fn test_verify_proof_two_leaves() {
    let tree = MerkleTree::from_data(&["a", "b"]);
    let root = tree.root_hash().unwrap();

    let proof_a = tree.generate_proof(0).unwrap();
    let proof_b = tree.generate_proof(1).unwrap();

    assert!(proof_a.verify("a", root));
    assert!(proof_b.verify("b", root));
}

#[test]
fn test_verify_proof_odd_leaf_count() {
    // 3 leaves -> padded to 4 internally.
    let tree = MerkleTree::from_data(&["a", "b", "c"]);
    let root = tree.root_hash().unwrap();

    let proof_c = tree.generate_proof(2).unwrap();
    assert!(proof_c.verify("c", root));
}

#[test]
fn test_verify_proof_fails_with_wrong_data() {
    let tree = MerkleTree::from_data(&["a", "b", "c", "d"]);
    let root = tree.root_hash().unwrap();

    let proof_b = tree.generate_proof(1).unwrap();
    assert!(!proof_b.verify("x", root));
}

#[test]
fn test_verify_proof_fails_with_wrong_root() {
    let tree1 = MerkleTree::from_data(&["a", "b", "c"]);
    let tree2 = MerkleTree::from_data(&["a", "b", "d"]);

    let wrong_root = tree2.root_hash().unwrap();
    let proof = tree1.generate_proof(2).unwrap();

    assert!(!proof.verify("c", wrong_root));
}

#[test]
fn test_single_leaf_proof() {
    let tree = MerkleTree::from_data(&["only"]);
    let root = tree.root_hash().unwrap();

    let proof = tree.generate_proof(0).unwrap();
    assert!(proof.steps.is_empty());
    assert!(proof.verify("only", root));
}
