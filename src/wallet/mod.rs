use crate::crypto::generate_key_pair;
use ed25519_dalek::Keypair;

#[derive(Debug)]
pub struct Wallet {
    pub address: String,
    pub keypair: Keypair,
}

impl Clone for Wallet {
    fn clone(&self) -> Self {
        let bytes = self.keypair.to_bytes();
        let keypair = Keypair::from_bytes(&bytes).unwrap();
        Self {
            address: self.address.clone(),
            keypair,
        }
    }
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
