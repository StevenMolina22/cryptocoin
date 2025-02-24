use super::BlockChain;
use crate::core::{block::Block, transaction::Transaction};
use ed25519_dalek::{PublicKey, SignatureError};

// TODO! Handle how to mine and receive transactions at the same time
const MAX_TRANSACTIONS: usize = 4;

impl BlockChain {
    ///
    pub fn add_deposit(&mut self, tx: Transaction, pk: &PublicKey) -> Result<(), SignatureError> {
        // Step 1: Verify signature for transaction inputs
        tx.is_valid(pk)?;

        // Step 2: Add to last block
        self.mempool.push(tx);

        let last_hash = self.last_hash();

        if self.mempool.len() == MAX_TRANSACTIONS {
            let transactions = self.mempool.drain(..).collect();
            let mut new_block = Block::new(&last_hash, transactions);

            new_block.mine(self.difficulty as usize);
            self.blocks.push(new_block);
        }
        Ok(())
    }

    // TODO! Handle errors different from signature ones
    pub fn add_transaction(
        &mut self,
        tx: Transaction,
        pk: &PublicKey,
    ) -> Result<(), SignatureError> {
        // Validations
        // TODO! Add enough UTXOs validation
        tx.is_valid(pk)?;

        // Add to last block
        self.mempool.push(tx);

        let last_hash = self.last_hash();

        if self.mempool.len() == MAX_TRANSACTIONS {
            let transactions = self.mempool.drain(..).collect();
            let mut new_block = Block::new(&last_hash, transactions);

            new_block.mine(self.difficulty as usize);
            self.blocks.push(new_block);
        }
        Ok(())
    }

    /// Returns a list of all transactions in the blockchain
    pub fn get_transactions(&self) -> Vec<Transaction> {
        self.blocks
            .iter()
            .flat_map(|block| block.transactions().iter().cloned())
            .chain(self.mempool.iter().cloned())
            .collect()
    }
}
