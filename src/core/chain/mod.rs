use super::transaction::utxo::UTXO;
use crate::core::{block::Block, transaction::Transaction};
use std::collections::HashMap;

mod chain;
mod consensus;
pub mod mempool;
mod transactions;

// TODO! Add coinbase_maturity config (amont of blocks needed to resolve a branch)
#[derive(Debug, Clone)]
pub struct BlockChain {
    pub blocks: Vec<Block>,
    pub mempool: Vec<Transaction>,
    pub utxos: UTXOSet,
    pub difficulty: usize,
    pub reward: usize,
}

/// It is needed to solve the double spending problem inside transactions in the same block.
///
/// the current solution is to have a Canonical and a Dynamic UTXO set
/// - the dynamic is the one to make validations before adding a transaction to a block
/// - the canonical is the one to make validations to transactions in a block in the blockchain
type UTXOSet = HashMap<(String, usize), UTXO>; // (tx_id, idx) -> UTXO

impl BlockChain {
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
