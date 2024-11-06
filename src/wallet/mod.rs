use ed25519_dalek::Keypair;

use crate::crypto::generate_key_pair;

pub mod accessors;
pub mod funds;
pub mod transactions;

#[derive(Debug)]
pub struct Wallet {
    address: String,
    balance: usize,
    keypair: Keypair,
}

impl Wallet {
    pub fn new(address: &str) -> Self {
        Wallet {
            address: address.to_string(),
            balance: 100,
            keypair: generate_key_pair(),
        }
    }
}
