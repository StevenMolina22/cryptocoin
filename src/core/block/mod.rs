use chrono::Utc;
use uuid::Uuid;

use super::transaction::Transaction;
pub mod accessors;
pub mod pow;
pub mod transactions;

/// TODO: Change Block.hash from Option<String> to String
/// - Remove Option wrapper since hash should always be present after creation
/// - Update serialization to include hash field
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
/// Represents a block in the blockchain.
///
/// A block contains a list of transactions, a unique identifier, a reference to the previous block,
/// a timestamp, and a nonce for proof-of-work.
pub struct Block {
    /// Unique identifier for this block.
    id: String,
    /// Hash of the previous block in the chain.
    previous_hash: String,
    /// Timestamp when this block was created.
    timestamp: u64,
    /// Nonce for proof-of-work.
    nonce: u64,
    /// List of transactions included in this block.
    pub transactions: Vec<Transaction>,
    /// Hash of this block's contents.
    #[serde(skip_serializing)]
    hash: Option<String>,
}

impl Block {
    /// Creates a new block template ready for mining
    ///
    /// Includes a coinbase transaction that rewards the miner.
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
