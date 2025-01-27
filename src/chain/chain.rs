use super::Chain;
use crate::block::Block;

impl Chain {
    pub fn get_last_block(&self) -> Option<&Block> {
        self.blocks.last()
    }

    pub fn get_last_block_mut(&mut self) -> Option<&mut Block> {
        self.blocks.last_mut()
    }

    pub fn get_block(&self, index: usize) -> Option<&Block> {
        self.blocks.get(index)
    }

    pub fn broadcast_block(&self, block: Block) {
        todo!()
    }

    pub fn balance_of(&self, addr: &str) -> usize {
        let mut balance = 0;
        for block in &self.blocks {
            for tx in &block.transactions {
                if tx.from_addr == addr {
                    balance -= tx.amount();
                } else if tx.to_addr == addr {
                    balance += tx.amount();
                }
            }
        }
        balance
    }

    fn len(&self) -> usize {
        self.blocks.len()
    }
}
