use ed25519_dalek::{PublicKey, SecretKey};

use super::Wallet;

impl Wallet {
    pub fn get_balance(&self) -> usize {
        self.balance
    }
    pub fn get_pk(&self) -> &PublicKey {
        &self.keypair.public
    }
    fn get_sk(&self) -> &SecretKey {
        &self.keypair.secret
    }
    pub fn get_address(&self) -> &str {
        &self.address
    }
    fn get_keypair(&self) -> &ed25519_dalek::Keypair {
        &self.keypair
    }
}
