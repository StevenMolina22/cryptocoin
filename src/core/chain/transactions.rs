use super::BlockChain;
use crate::core::{block::Block, transaction::Transaction};
use ed25519_dalek::{PublicKey, SignatureError};

const MAX_TRANSACTIONS: usize = 4;

impl BlockChain {
    pub fn include_transaction(
        &mut self,
        tx: Transaction,
        pk: &PublicKey,
    ) -> Result<(), SignatureError> {
        // TODO: Implement comprehensive UTXO validation
        // - Verify that transaction inputs reference existing UTXOs
        // - Check that total input amount >= total output amount
        // - Add unit tests for edge cases (zero inputs, very large amounts)
        tx.is_valid(pk)?;
        self.mempool.push(tx);

        if self.mempool.len() == MAX_TRANSACTIONS {
            return Ok(());
        }

        let transactions = self.mempool.drain(..).collect();

        let mut new_block =
            Block::new_template(&self.last_hash(), "miner", self.reward, transactions);
        new_block.mine(self.difficulty);

        self.remove_input_utxos(&new_block);
        self.create_output_utxos(&new_block);
        self.blocks.push(new_block);
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

    // Populates into the blockchain a mined block
    pub fn submit_block(&mut self, block: Block) {
        // ?TODO! block validation ???
        self.blocks.push(block)
    }
}
