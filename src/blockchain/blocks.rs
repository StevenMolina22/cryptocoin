use crate::common::Transaction;

use super::{Block, Blockchain};

impl Block {
    pub fn new(id: usize, previous_hash: &str, transactions: Vec<Transaction>) -> Result<Self, ()> {
        Ok(Block {
            id,
            previous_hash: String::from(previous_hash),
            transactions,
        })
    }

    fn calculate_hash(&self) -> String {
        String::from("hash")
    }

    fn verify_pow() -> bool {
        false
    }

    pub fn add_transaction(&mut self, transaction: Transaction) -> Result<(), ()> {
        Ok(())
    }
}

impl Blockchain {
    fn new() -> Blockchain {
        Blockchain { blocks: vec![] }
    }

    fn add_block(&mut self, block: Block) {}

    pub fn get_last_block(&self) -> Option<&Block> {
        self.blocks.last()
    }

    pub fn get_block(&self, index: usize) -> Option<&Block> {
        self.blocks.get(index)
    }

    fn mine_block(&mut self, block: Block) {}

    fn get_length(&self) -> usize {
        self.blocks.len()
    }

    fn check_integrity() -> bool {
        false
    }

    fn check_consenus() -> bool {
        false
    }

    pub fn add_transaction(&mut self, transaction: Transaction) -> Result<(), ()> {
        match self.get_last_block() {
            Some(block) => match block.transactions.len() {
                MAX_TRANSACTIONS => Ok(()),
                _ => Ok(()),
            },
            None => {
                self.mine_block(Block::new(0, "", vec![transaction])?);
                Ok(())
            }
        }
    }

    pub fn get_transaction_list(&self) -> Vec<Transaction> {
        self.blocks
            .iter()
            .flat_map(|block| block.transactions.iter().cloned())
            .collect()
    }

    pub fn search_transaction(&self, id: &str) -> Option<&Transaction> {
        self.blocks
            .iter()
            .flat_map(|block| block.transactions.iter())
            .find(|&transaction| transaction.id == id)
    }
}
