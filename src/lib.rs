//! # Fast SMT (Sparse Merkle Tree)
//!
//! Constructs a new `SparseMerkleTree<H, V, S>`, using TurboSHAKE128 as default hash function.
//! You always have the freedom of using your favourite hash function by implementing `Hasher` trait.
//! We provide two `Hasher` implementations - TurboSHAKE128 and BLAKE3. As per benchmarks, both of them stand
//! shoulder-to-shoulder - showing impressive performance, compared to SHA256 or SHA3_256.
//!
//! # Examples
//!
//! ```
//! use fast_sparse_merkle_tree::{
//!     turboshake_hasher::TurboShake128Hasher, default_store::DefaultStore,
//!     error::Error, MerkleProof,
//!     SparseMerkleTree, traits::Value, H256, Hash,
//!     traits::Hasher,
//! };
//!
//! // Type define SMT
//! type SMT = SparseMerkleTree<TurboShake128Hasher, Hash, Word, DefaultStore<Hash, Word, 32>, 32>;
//!
//! // Define SMT value
//! #[derive(Default, Clone, PartialEq)]
//! pub struct Word(String, H256);
//!
//! impl Value for Word {
//!    fn as_slice(&self) -> &[u8] {
//!        self.1.as_slice()
//!    }
//!    fn zero() -> Self {
//!        Default::default()
//!    }
//! }
//!
//! fn construct_smt() {
//!     let mut tree = SMT::default();
//!
//!     for (i, word) in "The quick brown fox jumps over the lazy dog"
//!         .split_whitespace()
//!         .enumerate()
//!     {
//!         let key: Hash = {
//!             let mut hasher = TurboShake128Hasher::default();
//!
//!             hasher.write_bytes(&(i as u32).to_le_bytes());
//!             hasher.finish().into()
//!         };
//!
//!         let hash: H256 = if !word.is_empty() {
//!             let mut hasher = TurboShake128Hasher::default();
//!
//!             hasher.write_bytes(word.as_bytes());
//!             hasher.finish().into()
//!         } else {
//!             H256::zero()
//!         };
//!
//!         let value = Word(word.to_string(), hash);
//!
//!         // insert <key, value> pair into the tree
//!         tree.update(key, value).expect("inserting into SMT must not fail");
//!     }
//!
//!     println!("SMT root is {:?} ", tree.root());
//! }
//! ```
//!
//! ## Installation
//!
//! Add this to your project's `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! fast-sparse-merkle-tree = "=0.1.1"
//! # or (minimal, just `turboshake` for faster hashing, no_std)
//! fast-sparse-merkle-tree = { version = "=0.1.1", default-features = false, features = ["turboshake"] }
//! ```
//!
//! For more see README in `fast-sparse-merkle-tree` repository @ <https://github.com/itzmeanjan/fast-sparse-merkle-tree>.

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "blake3")]
pub mod blake3_hasher;

#[cfg(feature = "turboshake")]
pub mod turboshake_hasher;

pub mod default_store;
pub mod error;
pub mod h256;
pub mod internal_key;
pub mod merge;
pub mod merkle_proof;
pub mod traits;
pub mod tree;

#[cfg(test)]
mod tests;

pub use h256::{H256, Hash};
pub use internal_key::InternalKey;
pub use merkle_proof::{CompiledMerkleProof, MerkleProof};
pub use traits::Key;
pub use tree::SparseMerkleTree;

/// Expected path size: log2(256) * 2, used for hint vector capacity
pub const EXPECTED_PATH_SIZE: usize = (256usize.ilog2() * 2) as usize;
/// Height of sparse merkle tree
pub const TREE_HEIGHT: usize = 256;
/// Key limit size
pub const KEY_LIMIT: usize = u32::MAX as usize;

cfg_if::cfg_if! {
    if #[cfg(feature = "std")] {
        use std::collections;
        use std::vec;
        use std::string;
        use std::vec as vec_macro;
    } else {
        extern crate alloc;
        use alloc::collections;
        use alloc::vec;
        use alloc::string;
        use alloc::vec as vec_macro;
    }
}
