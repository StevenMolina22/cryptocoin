use ed25519_dalek::PublicKey;

use super::Blockchain;
use crate::{block::Block, crypto::verify_signature, transaction::Transaction};
const MAX_TRANSACTIONS: usize = 10;

impl Blockchain {
    /// Adds transaction to blockchain if it is valid
    ///
    /// Returns Err if transaction is invalid
    pub fn add_transaction(&mut self, transaction: Transaction, pk: &PublicKey) -> Result<(), ()> {
        match transaction.signature {
            Some(ref signature) => {
                if !verify_signature(&transaction, signature, &pk) {
                    println!("Signature verification failed in add to chain");
                    return Err(());
                }
            }
            None => {
                println!("No signature in add to chain");
                return Err(());
            }
        }
        match self.get_last_block() {
            Some(block) => match block.get_transactions().len() {
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
            .flat_map(|block| block.get_transactions().iter().cloned())
            .collect()
    }

    pub fn search_transaction(&self, id: &str) -> Option<&Transaction> {
        self.blocks
            .iter()
            .flat_map(|block| block.get_transactions().iter())
            .find(|&transaction| transaction.id == id)
    }
}
