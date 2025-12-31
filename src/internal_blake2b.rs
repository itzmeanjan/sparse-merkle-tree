use crate::{H256, traits::Hasher};
use blake2::Blake2bVar;
use blake2::digest::{Update, VariableOutput};

const BLAKE2B_DIGEST_BYTE_LEN: usize = 32;
const BLAKE2B_PERSONALIZATION_STRING: &[u8] = b"sparsemerkletree";

pub struct Blake2bHasher(Blake2bVar);

impl Default for Blake2bHasher {
    fn default() -> Self {
        unsafe {
            let mut hasher = Blake2bVar::new(BLAKE2B_DIGEST_BYTE_LEN).unwrap_unchecked();
            hasher.update(BLAKE2B_PERSONALIZATION_STRING);

            Blake2bHasher(hasher)
        }
    }
}

impl Hasher for Blake2bHasher {
    fn write_bytes(&mut self, bytes: &[u8]) {
        self.0.update(bytes);
    }

    fn finish(self) -> H256 {
        unsafe {
            let mut digest = [0u8; BLAKE2B_DIGEST_BYTE_LEN];
            self.0.finalize_variable(&mut digest).unwrap_unchecked();

            digest.into()
        }
    }
}
