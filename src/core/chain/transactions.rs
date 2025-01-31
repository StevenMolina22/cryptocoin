use super::Chain;
use crate::{
    core::{block::Block, transaction::Transaction},
    crypto::is_valid_signature,
};
use ed25519_dalek::PublicKey;

// TODO! Handle how to mine and receive transactions at the same time
const MAX_TRANSACTIONS: usize = 4;

impl Chain {
    ///
    pub fn add_deposit(&mut self, tx: Transaction, pk: &PublicKey) -> Result<(), ()> {
        // Step 1: Verify signature
        match tx.signature {
            Some(ref signature) if is_valid_signature(&tx, signature, &pk) => {}
            _ => return Err(()),
        }

        // Step 2: Add to last block
        assert!(self.get_last_block().is_some());
        self.mempool.push(tx);

        if let Some(block) = self.get_last_block_mut() {
            let last_hash = block.get_hash().unwrap();
            if self.mempool.len() == MAX_TRANSACTIONS {
                let mut new_block = Block::new(&last_hash, vec![]);

                for tx_x in self.mempool.drain(..) {
                    let _ = new_block.add_transaction(tx_x.clone()); // invalid txs are skipped
                }
                new_block.mine(self.difficulty as usize);
                self.blocks.push(new_block);
            }
        }
        Ok(())
    }

    pub fn add_transaction(&mut self, tx: Transaction, pk: &PublicKey) -> Result<(), ()> {
        // Step 1: Validations
        if self.balance_of(&tx.from_addr) < tx.amount() {
            return Err(());
        }
        match tx.signature {
            Some(ref signature) if is_valid_signature(&tx, signature, &pk) => {}
            _ => return Err(()),
        }

        // Step 2: Add to last block
        assert!(self.get_last_block().is_some());
        self.mempool.push(tx);

        if let Some(block) = self.get_last_block_mut() {
            let last_hash = block.get_hash().unwrap();
            if self.mempool.len() == MAX_TRANSACTIONS {
                let mut new_block = Block::new(&last_hash, vec![]);

                for tx_x in self.mempool.drain(..) {
                    let _ = new_block.add_transaction(tx_x.clone()); // invalid txs are skipped
                }
                new_block.mine(self.difficulty as usize);
                self.blocks.push(new_block);
            }
        }
        Ok(())
    }

    /// Returns a list of all transactions in the blockchain
    pub fn get_transactions(&self) -> Vec<Transaction> {
        self.blocks
            .iter()
            .flat_map(|block| block.get_transactions().iter().cloned())
            .chain(self.mempool.iter().cloned())
            .collect()
    }
}
