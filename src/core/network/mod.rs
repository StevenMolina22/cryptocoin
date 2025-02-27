use super::chain::BlockChain;

pub mod block;
pub mod blockchain;
pub mod transaction;

#[derive(Debug)]
pub struct Node {
    pub blockchain: BlockChain,
}

impl Node {
    pub fn new(chain: BlockChain) -> Self {
        Node { blockchain: chain }
    }
}
