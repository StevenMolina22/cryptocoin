use super::Blockchain;
use crate::{block::Block, crypto::verify_signature, transaction::Transaction};
const MAX_TRANSACTIONS: usize = 10;

impl Blockchain {
    pub fn new() -> Blockchain {
        Blockchain { blocks: vec![] }
    }

    pub fn add_block(&mut self, block: Block) {}

    pub fn get_last_block(&self) -> Option<&Block> {
        self.blocks.last()
    }

    pub fn get_block(&self, index: usize) -> Option<&Block> {
        self.blocks.get(index)
    }

    pub fn broadcast_block(&self, block: Block) {}

    pub fn broadcast_transaction(&self, transaction: Transaction) {}

    fn get_length(&self) -> usize {
        self.blocks.len()
    }

    fn check_integrity() -> bool {
        false
    }

    fn check_consenus() -> bool {
        false
    }

    /// Adds transaction to blockchain if it is valid
    ///
    /// Returns Err if transaction is invalid
    pub fn add_transaction(&mut self, transaction: Transaction, pk: [u8; 32]) -> Result<(), ()> {
        // TODO: replace with actual public key
        match transaction.signature {
            Some(ref signature) => {
                if verify_signature(&transaction, signature, pk) {
                    return Ok(());
                }
            }
            None => return Err(()),
        }
        match self.get_last_block() {
            Some(block) => match block.transactions.len() {
                MAX_TRANSACTIONS => Ok(()),
                _ => Ok(()),
            },
            None => {
                Block::new(0, "", vec![transaction], 0).mine(); // TODO: calculate nonce
                Ok(())
            }
        }
    }

    /// Returns a list of all transactions in the blockchain
    pub fn get_transaction_list(&self) -> Vec<Transaction> {
        self.blocks
            .iter()
            .flat_map(|block| block.transactions.iter().cloned())
            .collect()
    }

    pub fn search_transaction(&self, id: &str) -> Option<&Transaction> {
        self.blocks
            .iter()
            .flat_map(|block| block.transactions.iter())
            .find(|&transaction| transaction.id == id)
    }
}
