use std::collections::HashSet;

use super::Chain;
use crate::core::block::Block;

// TODO! Handle forks
impl Chain {
    pub fn last_block(&self) -> Option<&Block> {
        self.blocks.last()
    }

    pub fn last_hash(&self) -> Option<String> {
        match self.blocks.last() {
            Some(last) => last.hash(),
            None => None,
        }
    }

    #[allow(dead_code, unused_variables)]
    pub fn broadcast_block(&self, block: Block) {
        todo!()
    }

    pub fn balance_of(&self, addr: &str) -> usize {
        self.blocks
            .iter()
            .flat_map(|block| &block.transactions)
            .chain(&self.mempool)
            .fold(0, |balance, tx| {
                match (tx.from_addr == addr, tx.to_addr == addr) {
                    (true, _) => balance - tx.amount(),
                    (_, true) => balance + tx.amount(),
                    _ => balance,
                }
            })
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
