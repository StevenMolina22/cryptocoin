use super::Wallet;
use crate::core::{chain::Chain, transaction::Transaction};

impl Wallet {
    pub fn get_transactions(&self, blockchain: &Chain) -> Vec<Transaction> {
        blockchain.get_transactions()
    }
}
