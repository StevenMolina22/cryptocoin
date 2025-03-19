use ed25519_dalek::SignatureError;

use super::Block;
use crate::crypto::hash_block;

impl Block {
    /// Mines this block by finding a nonce that produces a hash with the required difficulty
    ///
    /// The mining process involves incrementing the nonce until the block hash
    /// starts with the specified number of zeros.
    pub fn mine(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty);
        if self.hash.is_none() {
            self.hash = Some(hash_block(self));
        }
        while !self.hash.as_ref().unwrap().starts_with(&target) {
            self.nonce += 1;
            self.hash = Some(hash_block(self));
        }
    }

    /// Validates that this block meets the proof-of-work requirement
    /// # TODO
    /// * Implement validation of the transactions inside the block
    pub fn is_valid(&self, difficulty: usize) -> Result<(), SignatureError> {
        let target = "0".repeat(difficulty);
        match self.hash.as_ref().unwrap().starts_with(&target) {
            true => Ok(()),
            false => Err(SignatureError::new()),
        }
    }
}
