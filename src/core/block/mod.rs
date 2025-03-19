use chrono::Utc;
use uuid::Uuid;

use super::transaction::Transaction;
pub mod accessors;
pub mod pow;
pub mod transactions;

// TODO: Change Block.hash from Option<String> to String
// - Remove Option wrapper since hash should always be present after creation
// - Update serialization to include hash field
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Block {
    id: String,
    previous_hash: String,
    timestamp: u64,
    nonce: u64,
    pub transactions: Vec<Transaction>,
    #[serde(skip_serializing)]
    hash: Option<String>,
}

impl Block {
    pub fn new_template(
        previous_hash: &str,
        miner: &str,
        reward: usize,
        mut transactions: Vec<Transaction>,
    ) -> Self {
        transactions.push(Transaction::new_coinbase(miner, reward));
        Block {
            id: Uuid::new_v4().to_string(),
            previous_hash: String::from(previous_hash),
            timestamp: Utc::now().timestamp() as u64,
            nonce: 0,
            transactions,
            hash: None,
        }
    }
}
