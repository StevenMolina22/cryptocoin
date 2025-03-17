use super::transaction::utxo::UTXO;
use crate::core::{block::Block, transaction::Transaction};
use std::collections::HashMap;

mod chain;
mod concensus;
pub mod mempool;
mod transactions;

// TODO! Add coinbase_maturity config (amont of blocks needed to resolve a branch)
#[derive(Debug, Clone)]
pub struct BlockChain {
    pub blocks: Vec<Block>,
    pub mempool: Vec<Transaction>,
    pub utxos: UTXOPool,
    pub difficulty: usize,
    pub reward: usize,
}

type UTXOPool = HashMap<(String, usize), UTXO>; // (tx_id, idx) -> UTXO

impl BlockChain {
    // TODO! verify that two transactions in the mempool do not use the same UTXO
    pub fn new() -> BlockChain {
        let reward = 50 * 1_000_000; // 50 coins
        let mut genesis_block = Block::new_template("", "", reward, vec![]);
        genesis_block.mine(3);
        BlockChain {
            blocks: vec![genesis_block],
            difficulty: 3,
            mempool: vec![],
            utxos: HashMap::new(),
            reward,
        }
    }
}
