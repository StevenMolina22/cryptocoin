use super::BlockChain;
use crate::core::block::Block;

impl BlockChain {
    /// Returns the hash of the last block in the chain
    ///
    /// This is used as the previous_hash when creating a new block.
    pub fn last_hash(&self) -> String {
        self.blocks.last().unwrap().hash().unwrap()
    }

    /// Updates the blockchain with a new block and manages the UTXO pool
    pub fn update_from(&mut self, block: Block) {
        self.remove_input_utxos(&block);
        self.create_output_utxos(&block);
        self.mempool.retain(|tx| !block.transactions.contains(tx));
        self.blocks.push(block);
    }
}
