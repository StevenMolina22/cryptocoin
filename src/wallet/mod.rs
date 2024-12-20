use crate::crypto::generate_key_pair;
use ed25519_dalek::Keypair;

pub mod accessors;
pub mod funds;
pub mod transactions;

#[derive(Debug)]
pub struct Wallet {
    pub address: String,
    // balance: usize,
    keypair: Keypair,
}

impl Wallet {
    pub fn new(address: &str) -> Self {
        Wallet {
            address: address.to_string(),
            // balance: 100,
            keypair: generate_key_pair(),
        }
    }
    pub fn get_wallet(wallets: Vec<&Wallet>, address: String) -> Option<&Wallet> {
        for wallet in wallets {
            if wallet.address == address {
                return Some(wallet);
            }
        }
        None
    }
    pub fn get_wallet_mut(wallets: Vec<&mut Wallet>, address: String) -> Option<&mut Wallet> {
        for wallet in wallets {
            if wallet.address == address {
                return Some(wallet);
            }
        }
        None
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
