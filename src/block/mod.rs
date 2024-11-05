use crate::transaction::Transaction;

pub mod accessors;

#[derive(Debug, Clone)]
pub struct Block {
    id: usize,
    previous_hash: String,
    timestamp: u64,
    nonce: u64,
    transactions: Vec<Transaction>,
}

impl Block {
    pub fn new(id: usize, previous_hash: &str, transactions: Vec<Transaction>, nonce: u64) -> Self {
        Block {
            id,
            previous_hash: String::from(previous_hash),
            timestamp: 0, // TODO: get current time
            nonce,        // TODO: calculate nonce
            transactions,
        }
    }

    fn calculate_hash(&self) -> String {
        // let mut hasher = DefaultHasher::new();
        // self.hash(&mut hasher);
        // let hash_value = hasher.finish();
        // format!("{:016x}", hash_value)
        String::from("") // TODO: implement
    }

    fn verify_pow() -> bool {
        false
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
        Ok(())
    }

    pub fn mine(&mut self) {}
}
