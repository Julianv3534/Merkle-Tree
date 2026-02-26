# merkle_tree

A small Rust library that implements a Merkle Tree using SHA-256.

This repository contains code to:
- hash data into fixed-size 32-byte hashes,
- build a Merkle tree from input data,
- compute a root hash,
- generate inclusion proofs,
- verify proofs against an expected root.

## Repository contents

- `src/lib.rs` – crate entry point and re-exports.
- `src/hash.rs` – hash utilities (`Hash`, `hash_data`, `hash_pair`).
- `src/tree.rs` – `MerkleTree` structure and tree construction logic.
- `src/proof.rs` – proof types and verification (`MerkleProof`, `ProofStep`).
- `tests/tree_tests.rs` – tests for tree behavior and indexing helpers.
- `tests/proof_tests.rs` – tests for proof generation and verification.
- `Cargo.toml` – Rust package metadata and dependencies.
- `Makefile` – helper commands for common development tasks.

## Quick start
with Make:

```bash
make check
make test
```
You can also use cargo

## Notes

- Leaves are padded to the next power of two by duplicating the last leaf.
- Hashes are 32-byte arrays (`[u8; 32]`) because SHA-256 output size is fixed.
