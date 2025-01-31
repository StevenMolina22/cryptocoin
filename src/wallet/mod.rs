use crate::crypto::generate_key_pair;
use ed25519_dalek::Keypair;
pub mod accessors;
pub mod funds;
pub mod transactions;

#[derive(Debug)]
pub struct Wallet {
    pub address: String,
    pub keypair: Keypair,
}

impl Wallet {
    pub fn new(address: &str) -> Self {
        Wallet {
            address: address.to_string(),
            keypair: generate_key_pair(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_wallet_new() {
        let wallet = Wallet::new("test_adress");
        assert_eq!(wallet.address, "test_adress");
        assert!(wallet.keypair.public.as_bytes().len() > 0);
    }
}
