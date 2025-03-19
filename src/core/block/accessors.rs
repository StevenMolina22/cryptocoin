use crate::core::transaction::Transaction;

use super::Block;

impl Block {
    pub fn hash(&self) -> Option<String> {
        self.hash.clone()
    }
}
