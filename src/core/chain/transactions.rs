use super::BlockChain;
use crate::core::{block::Block, transaction::Transaction};
use ed25519_dalek::{PublicKey, SignatureError};

// TODO! Handle how to mine and receive transactions at the same time
const MAX_TRANSACTIONS: usize = 4;

impl BlockChain {
    // TODO! Handle errors different from signature ones
    pub fn include_transaction(
        &mut self,
        tx: Transaction,
        pk: &PublicKey,
    ) -> Result<(), SignatureError> {
        // TODO! Add enough UTXOs validation
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
        // TODO! block validation?
        self.blocks.push(block)
    }
}
