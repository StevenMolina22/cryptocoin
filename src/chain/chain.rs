use super::Blockchain;
use crate::block::Block;

impl Blockchain {
    pub fn new() -> Blockchain {
        Blockchain { blocks: vec![] }
    }

    pub fn add_block(&mut self, block: Block) {}

    pub fn get_last_block(&self) -> Option<&Block> {
        self.blocks.last()
    }

    pub fn get_block(&self, index: usize) -> Option<&Block> {
        self.blocks.get(index)
    }

    pub fn broadcast_block(&self, block: Block) {}

    fn length(&self) -> usize {
        self.blocks.len()
    }
}
