use super::Block;
use crate::core::transaction::Transaction;

impl Block {
    /// Pre:
    pub fn add_transaction(&mut self, transaction: Transaction) -> Result<(), ()> {
        match self.transactions.len() {
            10 => Err(()),
            _ => {
                self.transactions.push(transaction);
                Ok(())
            }
        }
    }
}
