mod chain;
mod concensus;
mod transactions;
use crate::block::Block;

#[derive(Debug)]
pub struct Blockchain {
    blocks: Vec<Block>,
    // wallets: Vec<Wallet>,
}
