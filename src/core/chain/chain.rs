use std::collections::HashSet;

use super::BlockChain;
use crate::core::block::Block;

// TODO! Handle forks
impl BlockChain {
    pub fn last_block(&self) -> &Block {
        assert!(self.blocks.last().is_some());
        self.blocks.last().unwrap()
    }

    pub fn last_hash(&self) -> String {
        assert!(self.last_block().hash().is_some());
        self.last_block().hash().unwrap()
    }

    #[allow(dead_code, unused_variables)]
    pub fn broadcast_block(&self, block: Block) {
        todo!()
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
}
