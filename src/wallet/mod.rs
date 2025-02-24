use crate::{core::chain::BlockChain, crypto::generate_key_pair};
use ed25519_dalek::Keypair;
pub mod transactions;

#[derive(Debug)]
pub struct Wallet {
    blockchain: BlockChain,
    pub address: String,
    pub keypair: Keypair,
}

impl Wallet {
    pub fn new(blockchain: BlockChain) -> Self {
        let keypair = generate_key_pair();
        // TODO! hash the pk
        Wallet {
            blockchain,
            address: format!("{:?}", keypair.public),
            keypair,
        }
    }
}
