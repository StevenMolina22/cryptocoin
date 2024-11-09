use uuid::Uuid;

use crate::{crypto::hash_block, transaction::Transaction};

pub mod accessors;

#[derive(Debug, Clone, serde::Serialize)]
pub struct Block {
    id: String,
    previous_hash: Option<String>,
    timestamp: u64,
    nonce: u64,
    transactions: Vec<Transaction>,
    #[serde(skip_serializing)]
    hash: Option<String>,
}

impl Block {
    pub fn new(previous_hash: &str, transactions: Vec<Transaction>) -> Self {
        Block {
            id: Uuid::new_v4().to_string(),
            previous_hash: Some(String::from(previous_hash)),
            timestamp: 0, // TODO: get current time
            nonce: 0,     // TODO: calculate nonce
            transactions,
            hash: None,
        }
    }

    /// Adds transaction to already create block
    ///
    /// Pre:
    ///      - Block is not full
    ///      - Transaction is valid
    ///
    /// Post:
    ///     - Transaction is added to block
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

    pub fn mine(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty);
        while !target.starts_with(&target) {
            self.nonce += 1;
            self.hash = Some(hash_block(self))
        }
    }

    pub fn validate(&self) -> bool {
        match &self.hash {
            Some(hash) => *hash == hash_block(self),
            None => false,
        }
    }
}
