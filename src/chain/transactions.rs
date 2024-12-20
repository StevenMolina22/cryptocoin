use ed25519_dalek::PublicKey;

use super::Chain;
use crate::{
    block::Block,
    crypto::is_valid_signature,
    transaction::{Transaction, TransactionType},
    wallet::Wallet,
};
const MAX_TRANSACTIONS: usize = 10;

impl Chain {
    ///
    pub fn add_deposit(&mut self, tx: Transaction, pk: &PublicKey) -> Result<(), ()> {
        // Verify signature
        match tx.signature {
            Some(ref signature) => {
                if !is_valid_signature(&tx, signature, &pk) {
                    println!("Signature verification failed in add to chain");
                    return Err(());
                }
            }
            None => {
                println!("No signature in add to chain");
                return Err(());
            }
        }
        // Step 2: Add to last block
        match self.get_last_block_mut() {
            // there is a last block
            Some(block) => match block.get_transactions().len() {
                MAX_TRANSACTIONS => {
                    let mut new_block = Block::new(&block.get_hash().unwrap(), vec![]);

                    new_block.add_transaction(tx.clone())?;
                    self.blocks.push(new_block);
                }
                _ => {
                    block.add_transaction(tx.clone())?;
                }
            },
            // no last block (create genesis block)
            None => {
                let mut genesis_block = Block::new("", vec![]); // TODO!: calculate nonce
                genesis_block.add_transaction(tx.clone())?;
                self.blocks.push(genesis_block);
            }
        }
        Ok(())
    }

    pub fn add_transaction(&mut self, tx: Transaction, pk: &PublicKey) -> Result<(), ()> {
        // Step 1: Validations
        // TODO!: Verify balance
        // if self.balance_of(&tx.from_addr) < tx.amount() {
        //     return Err(());
        // }
        // Verify signature
        match tx.signature {
            Some(ref signature) => {
                if !is_valid_signature(&tx, signature, &pk) {
                    println!("Signature verification failed in add to chain");
                    return Err(());
                }
            }
            None => {
                println!("No signature in add to chain");
                return Err(());
            }
        }
        // Step 2: Add to last block
        match self.get_last_block_mut() {
            // there is a last block
            Some(block) => match block.get_transactions().len() {
                MAX_TRANSACTIONS => {
                    let mut new_block = Block::new(&block.get_hash().unwrap(), vec![]);

                    new_block.add_transaction(tx.clone())?;
                    self.blocks.push(new_block);
                }
                _ => {
                    block.add_transaction(tx.clone())?;
                }
            },
            // no last block (create genesis block)
            None => {
                let mut genesis_block = Block::new("", vec![]); // TODO!: calculate nonce
                genesis_block.add_transaction(tx.clone())?;
                self.blocks.push(genesis_block);
            }
        }
        Ok(())
    }

    // TODO!: Verify this function because it is not done
    pub fn deposit_to(&mut self, wallet: &Wallet, amount: usize) -> usize {
        let tx = Transaction::new(amount, "", &wallet.address, TransactionType::Cash);
        let amount = tx.amount();
        self.add_transaction(tx, wallet.get_pk()).unwrap();
        amount
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
