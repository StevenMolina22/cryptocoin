use ed25519_dalek::Keypair;

use crate::blockchain::Blockchain;
use crate::crypto::{generate_key_pair, sign_transaction, verify_signature};
use crate::transaction::{Transaction, TransactionType};
pub mod accessors;

#[derive(Debug)]
pub struct Wallet {
    address: String,
    balance: usize,
    keypair: Keypair,
}

impl Wallet {
    pub fn new(address: &str) -> Self {
        let keypair = generate_key_pair();
        Wallet {
            address: address.to_string(),
            balance: 0,
            keypair,
        }
    }

    /// ignore for now
    pub fn deposit_funds(&mut self, amount: usize) -> Result<(), ()> {
        Ok(())
    }

    /// ignore for now
    pub fn withdraw_funds(&mut self, amount: usize) -> Result<(), ()> {
        Ok(())
    }

    pub fn get_transactions(&self, blockchain: &Blockchain) -> Vec<Transaction> {
        blockchain.get_transaction_list()
    }

    pub fn transfer(
        &mut self,
        amount: usize,
        receiver_addr: &str,
        blockchain: &mut Blockchain,
    ) -> Result<(), ()> {
        let tx = Transaction::new(
            String::from(""),
            amount,
            self.address.clone(),
            receiver_addr.to_string(),
            TransactionType::DebitCard,
        );
        let signature = sign_transaction(&tx, &self.keypair);
        if !verify_signature(&tx, &signature, &self.keypair.public) {
            return Err(());
        }
        blockchain.add_transaction(tx, &self.keypair.public)
    }
}
