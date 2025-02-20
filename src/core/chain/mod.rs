use super::transaction::UTXO;
use crate::core::{block::Block, transaction::Transaction};
use std::collections::HashMap;

mod chain;
mod concensus;
mod transactions;

// TODO! Add coinbase_maturity config (amont of blocks needed to resolve a branch)
#[derive(Debug)]
pub struct Chain {
    blocks: Vec<Block>,
    difficulty: u32,
    mempool: Vec<Transaction>,
    utxos: HashMap<(String, usize), UTXO>, // (tx_id, index) -> UTXO
}

impl Chain {
    pub fn new() -> Chain {
        let mut genesis_block = Block::new("", vec![]);
        genesis_block.mine(3);
        Chain {
            blocks: vec![genesis_block],
            difficulty: 3,
            mempool: vec![],
            utxos: HashMap::new(),
            // reward: 10,
        }
    }
}
