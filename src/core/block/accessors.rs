use crate::core::transaction::Transaction;

use super::Block;

impl Block {
    pub fn transactions(&self) -> &[Transaction] {
        &self.transactions
    }
    pub fn hash(&self) -> Option<String> {
        self.hash.clone()
    }
}
