use ed25519_dalek::{PublicKey, SecretKey};

use crate::chain::Chain;

use super::Wallet;

impl Wallet {
    pub fn balance(&self, bc: &Chain) -> usize {
        let n = bc.balance_of(&self.address);
        n
    }
    pub fn get_pk(&self) -> &PublicKey {
        &self.keypair.public
    }
    fn get_sk(&self) -> &SecretKey {
        &self.keypair.secret
    }
    pub fn get_addr(&self) -> &str {
        &self.address
    }
    fn get_keypair(&self) -> &ed25519_dalek::Keypair {
        &self.keypair
    }
    // pub fn update_balance(&mut self, amount: isize) {
    //     self.balance = self.balance.saturating_add(amount as usize)
    // }
}
