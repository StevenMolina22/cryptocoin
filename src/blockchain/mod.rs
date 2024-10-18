use crate::common::Transaction;

const MAX_TRANSACTIONS: usize = 10;

#[derive(Debug)]
pub struct Block {
    id: usize,
    previous_hash: String,
    transactions: Vec<Transaction>,
}

#[derive(Debug)]
pub struct Blockchain {
    blocks: Vec<Block>,
}

impl Block {
    fn new(id: usize, previous_hash: &str, transactions: Vec<Transaction>) -> Result<Block, ()> {
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

    fn add_transaction(&mut self, transaction: Transaction) -> Result<(), ()> {
        Ok(())
    }
}

impl Blockchain {
    fn new() -> Blockchain {
        Blockchain { blocks: vec![] }
    }

    fn add_block(&mut self, block: Block) {}

    fn get_last_block(&self) -> Option<&Block> {
        self.blocks.last()
    }

    fn get_block(&self, index: usize) -> Option<&Block> {
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

    fn add_transaction(&mut self, transaction: Transaction) -> Result<(), ()> {
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

    fn get_transaction_list(&self) -> Vec<Transaction> {
        vec![]
    }

    fn search_transaction(&self, id: &str) -> Option<Transaction> {
        None
    }
}

impl Iterator for Blockchain {
    type Item = Block;

    fn next(&mut self) -> Option<Block> {
        None
    }
}
