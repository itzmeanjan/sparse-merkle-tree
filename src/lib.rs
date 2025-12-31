//! Constructs a new `SparseMerkleTree<H, V, S>`.
//!
//! # Examples
//!
//! ```
//! use nam_sparse_merkle_tree::{
//!     internal_blake2b::Blake2bHasher, default_store::DefaultStore,
//!     error::Error, MerkleProof,
//!     SparseMerkleTree, traits::Value, H256, Hash,
//!     traits::Hasher,
//! };
//!
//! // define SMT
//! type SMT = SparseMerkleTree<Blake2bHasher, Hash, Word, DefaultStore<Hash, Word, 32>, 32>;
//!
//! // define SMT value
//! #[derive(Default, Clone, PartialEq)]
//! pub struct Word(String, H256);
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
//!     for (i, word) in "The quick brown fox jumps over the lazy dog"
//!         .split_whitespace()
//!         .enumerate()
//!     {
//!         let key: Hash = {
//!             let mut hasher = Blake2bHasher::default();
//!             hasher.write_bytes(&(i as u32).to_le_bytes());
//!             hasher.finish().into()
//!         };
//!
//!         let hash: H256 = if !word.is_empty() {
//!             let mut hasher = Blake2bHasher::default();
//!             hasher.write_bytes(word.as_bytes());
//!             hasher.finish().into()
//!         } else {
//!             H256::zero()
//!         };
//!         let value = Word(word.to_string(), hash);
//!         // insert key value into tree
//!         tree.update(key, value).expect("update");
//!     }
//!
//!     println!("SMT root is {:?} ", tree.root());
//! }
//! ```

#![cfg_attr(not(feature = "std"), no_std)]

pub mod default_store;
pub mod error;
pub mod h256;
#[cfg(feature = "blake2b")]
pub mod internal_blake2b;
pub mod internal_key;
pub mod merge;
pub mod merkle_proof;
pub mod proof_ics23;
pub mod sha256;
#[cfg(test)]
mod tests;
pub mod traits;
pub mod tree;

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
pub const KEY_LIMIT: usize = 4_294_967_295u32 as usize;

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
