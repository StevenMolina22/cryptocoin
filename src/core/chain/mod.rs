use super::transaction::utxo::UTXO;
use crate::core::{block::Block, transaction::Transaction};
use std::collections::HashMap;

mod chain;
mod concensus;
mod transactions;

// TODO! Add coinbase_maturity config (amont of blocks needed to resolve a branch)
#[derive(Debug)]
pub struct BlockChain {
    blocks: Vec<Block>,
    mempool: Vec<Transaction>,
    pub utxos: UTXOPool,
    pub difficulty: u32,
    reward: usize,
}

type UTXOPool = HashMap<(String, usize), UTXO>; // (tx_id, idx) -> UTXO

impl BlockChain {
    pub fn new() -> BlockChain {
        let mut genesis_block = Block::new("", vec![]);
        genesis_block.mine(3);
        BlockChain {
            blocks: vec![genesis_block],
            difficulty: 3,
            mempool: vec![],
            utxos: HashMap::new(),
            reward: 50 * 1_000_000, // 50 coins
        }
    }
}
