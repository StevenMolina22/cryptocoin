use crate::common::Transaction;

mod blocks;
pub use blocks::*;

const MAX_TRANSACTIONS: usize = 10;

#[derive(Debug, Clone)]
pub struct Block {
    id: usize,
    previous_hash: String,
    transactions: Vec<Transaction>,
}

#[derive(Debug)]
pub struct Blockchain {
    blocks: Vec<Block>,
}
