use crate::transaction::Transaction;

use super::Block;

impl Block {
    /// Pre:

    pub fn add_transaction(&mut self, transaction: Transaction) -> Result<(), ()> {
        let prev_hash = match self.transactions.last() {
            Some(tx) => tx.id.clone(),
            None => String::from(""),
        };
        match self.transactions.len() {
            10 => Err(()),
            _ => {
                self.transactions.push(transaction);
                Ok(())
            }
        }
    }
}
