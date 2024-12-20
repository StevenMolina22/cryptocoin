mod chain;
mod concensus;
mod transactions;
use std::collections::HashMap;

use crate::{block::Block, wallet::Wallet};

#[derive(Debug)]
pub struct Chain {
    blocks: Vec<Block>,
    pub wallets: HashMap<String, Wallet>,
}

impl Chain {
    pub fn new() -> Chain {
        Chain {
            blocks: vec![],
            wallets: HashMap::new(),
        }
    }
}
