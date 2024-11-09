use ed25519_dalek::PublicKey;

use super::Blockchain;
use crate::{block::Block, crypto::verify_signature, transaction::Transaction};
const MAX_TRANSACTIONS: usize = 10;

impl Blockchain {
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
                MAX_TRANSACTIONS => {
                    let mut new_block = Block::new(&block.get_hash().unwrap(), vec![]);
                    new_block.add_transaction(transaction).unwrap();
                    Ok(())
                }
                _ => Ok(()),
            },
            None => {
                let mut genesis_block = Block::new("", vec![]); // TODO: calculate nonce
                genesis_block.add_transaction(transaction).unwrap();
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
