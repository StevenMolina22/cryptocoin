use crate::crypto::generate_key_pair;
use ed25519_dalek::Keypair;
pub mod transactions;

#[derive(Debug)]
pub struct Wallet {
    pub address: String,
    pub keypair: Keypair,
}

impl Wallet {
    pub fn new() -> Self {
        let keypair = generate_key_pair();
        // TODO! Add the hashed pk as an address
        Wallet {
            address: format!("{:?}", keypair.public),
            keypair,
        }
    }
}
