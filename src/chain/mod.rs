mod chain;
mod concensus;
mod transactions;
use std::collections::HashMap;

use crate::{block::Block, transaction::Transaction, wallet::Wallet};

#[derive(Debug)]
pub struct Chain {
    blocks: Vec<Block>,
    difficulty: u32,
    pub wallets: HashMap<String, Wallet>,
    mempool: Vec<Transaction>,
}

impl Chain {
    pub fn new() -> Chain {
        let mut genesis_block = Block::new("", vec![]);
        genesis_block.mine(3);
        Chain {
            blocks: vec![genesis_block],
            difficulty: 3,
            wallets: HashMap::new(),
            mempool: vec![],
        }
    }
}
