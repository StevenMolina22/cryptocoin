use crate::core::{block::Block, transaction::utxo::UTXO};

use super::BlockChain;

impl BlockChain {
    pub fn setup_miner(&mut self, miner: &str) {
        let mut coinbase_block = Block::new_template(&self.last_hash(), miner, self.reward, vec![]);
        coinbase_block.mine(self.difficulty as usize);
        self.create_output_utxos(&coinbase_block);
    }

    pub fn remove_input_utxos(&mut self, block: &Block) {
        for tx in block.transactions.iter() {
            for input in tx.inputs.iter() {
                self.utxos.remove(&(input.tx_id.clone(), input.index));
            }
        }
    }

    pub fn create_output_utxos(&mut self, block: &Block) {
        for tx in block.transactions.iter() {
            for (i, output) in tx.outputs.iter().enumerate() {
                self.utxos.insert(
                    (tx.id.clone(), i),
                    UTXO::new(&tx.id, i, output.amount, &output.recipient),
                );
            }
        }
    }
}
