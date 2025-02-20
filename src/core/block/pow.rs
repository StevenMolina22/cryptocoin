use super::Block;
use crate::crypto::hash_block;

impl Block {
    // TODO! Simplify this function
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
}
