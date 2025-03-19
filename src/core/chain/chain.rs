use super::BlockChain;
use crate::core::block::Block;

impl BlockChain {
    pub fn last_hash(&self) -> String {
        self.blocks.last().unwrap().hash().unwrap()
    }

    pub fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
    }

    // Function is not clear enough
    pub fn update_from(&mut self, block: Block) {
        self.remove_input_utxos(&block);
        self.create_output_utxos(&block);
        self.mempool.retain(|tx| !block.transactions.contains(tx));
        self.blocks.push(block);
    }
}
