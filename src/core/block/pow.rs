use ed25519_dalek::SignatureError;

use super::Block;
use crate::crypto::hash_block;

impl Block {
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

    pub fn is_valid(&self, difficulty: usize) -> Result<(), SignatureError> {
        // TODO! Validate the transactions inside the block
        let target = "0".repeat(difficulty);
        match self.hash.as_ref().unwrap().starts_with(&target) {
            true => Ok(()),
            false => Err(SignatureError::new()),
        }
    }
}
