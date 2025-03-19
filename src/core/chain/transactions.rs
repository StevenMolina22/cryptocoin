use crate::core::transaction::utxo::UTXO;

use super::BlockChain;

impl BlockChain {
    // pub fn include_transaction(
    //     &mut self,
    //     tx: Transaction,
    //     pk: &PublicKey,
    // ) -> Result<(), SignatureError> {
    //     // TODO: Implement comprehensive UTXO validation
    //     // - Verify that transaction inputs reference existing UTXOs
    //     // - Check that total input amount >= total output amount
    //     // - Add unit tests for edge cases (zero inputs, very large amounts)
    //     tx.is_valid(pk)?;
    //     self.mempool.push(tx);

    //     if self.mempool.len() == MAX_TRANSACTIONS {
    //         return Ok(());
    //     }

    //     let transactions = self.mempool.drain(..).collect();

    //     let mut new_block =
    //         Block::new_template(&self.last_hash(), "miner", self.reward, transactions);
    //     new_block.mine(self.difficulty);

    //     self.remove_input_utxos(&new_block);
    //     self.create_output_utxos(&new_block);
    //     self.blocks.push(new_block);
    //     Ok(())
    // }
    //
    pub fn utxos_of(&self, addr: &str) -> Vec<UTXO> {
        let mut utxos = vec![];
        for utxo in self.utxos.values() {
            if utxo.recipient == addr {
                utxos.push(utxo.clone())
            }
        }
        utxos
    }
}
