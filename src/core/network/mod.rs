use crate::wallet::Wallet;

pub mod block;
pub mod blockchain;
pub mod transaction;

// Simulated module till it is implemented
#[derive(Debug)]
pub struct Node {
    pub wallet: Wallet,
}

impl Node {
    pub fn new(wallet: Wallet) -> Self {
        Node { wallet }
    }
}
