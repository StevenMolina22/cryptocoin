use super::Chain;
use crate::core::block::Block;

impl Chain {
    pub fn get_last_block(&self) -> Option<&Block> {
        self.blocks.last()
    }

    pub fn get_last_block_mut(&mut self) -> Option<&mut Block> {
        self.blocks.last_mut()
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
}
