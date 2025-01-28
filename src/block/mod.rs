use std::hash;
use std::hash::Hash;
use chrono::Utc;
use uuid::Uuid;

use crate::{crypto::hash_block, transaction::Transaction};
pub mod accessors;
pub mod transactions;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Block {
    id: String,
    previous_hash: Option<String>,
    timestamp: u64,
    nonce: u64,
    pub transactions: Vec<Transaction>,
    #[serde(skip_serializing)]
    hash: Option<String>,
}

impl Block {
    pub fn new(previous_hash: &str, transactions: Vec<Transaction>) -> Self {
        Block {
            id: Uuid::new_v4().to_string(),
            previous_hash: Some(String::from(previous_hash)),
            timestamp: Utc::now().timestamp() as u64,
            // nonce: u64::MAX / 2, // TODO!: calculate nonce
            nonce: 0,
            transactions,
            hash: None,
        }
    }

    pub fn mine(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty);
        // TODO! Choose what happens if hash is None
        if self.hash.is_none() {
            self.hash = Some(hash_block(self));
        }
        if let Some(mut hash) = self.hash.take() {
            while !hash.starts_with(&target) {
                self.nonce += 1;
                hash = hash_block(self)
            }
            self.hash = Some(hash)
        }
    }

    pub fn validate(&self) -> bool {
        match &self.hash {
            Some(hash) => *hash == hash_block(self),
            None => false,
        }
    }
}
