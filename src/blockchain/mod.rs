mod chain;
mod concensus;

use crate::block::Block;

#[derive(Debug)]
pub struct Blockchain {
    blocks: Vec<Block>,
    // wallets: Vec<Wallet>,
}
