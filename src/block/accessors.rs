use crate::transaction::Transaction;

use super::Block;

impl Block {
    pub fn get_transactions(&self) -> &[Transaction] {
        &self.transactions
    }
}
