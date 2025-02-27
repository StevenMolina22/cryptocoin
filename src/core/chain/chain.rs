use crate::core::block::Block;

use super::BlockChain;
use std::collections::HashSet;

// TODO! Handle forks
impl BlockChain {
    pub fn last_hash(&self) -> String {
        self.blocks.last().unwrap().hash().unwrap()
    }

    pub fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
    }

    // TODO! check code
    pub fn _balance_of(&self, addr: &str) -> usize {
        let spent_utxos: HashSet<_> = self
            .mempool
            .iter()
            .flat_map(|tx| {
                tx.inputs
                    .iter()
                    .map(|input| (input.tx_id.clone(), input.index))
            })
            .collect();

        let confirmed_balance: usize = self
            .utxos
            .values()
            .filter(|utxo| {
                utxo.recipient == addr && !spent_utxos.contains(&((utxo.tx_id.clone(), utxo.index)))
            })
            .map(|utxo| utxo.amount)
            .sum();

        let pending_balance: usize = self
            .mempool
            .iter()
            .flat_map(|tx| {
                tx.outputs
                    .iter()
                    .filter(|output| output.recipient == addr)
                    .map(|output| output.amount)
            })
            .sum();

        confirmed_balance + pending_balance
    }

    pub fn update_from(&mut self, block: Block) {
        self.remove_input_utxos(&block);
        self.create_output_utxos(&block);
        self.mempool.retain(|tx| !block.transactions.contains(tx));
        self.blocks.push(block);
    }
}
