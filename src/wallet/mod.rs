use crate::common::Transaction;
use crate::common::{TransactionType, Wallet};

impl Wallet {
    pub fn new(address: &str) -> Wallet {
        Wallet {
            address: address.to_string(),
            balance: 0,
        }
    }

    pub fn deposit(&mut self, amount: usize) -> Result<(), ()> {
        // ignore for now
        Ok(())
    }

    pub fn withdraw(&mut self, amount: usize) -> Result<(), ()> {
        // ignore for now
        Ok(())
    }

    pub fn get_transactions(&self) -> Vec<Transaction> {
        vec![]
    }

    pub fn get_balance(&self) -> usize {
        0
    }

    pub fn transfer(&mut self, amount: usize, receiver_addr: &str) -> Result<(), ()> {
        let transaction = Transaction::new(
            String::from(""),
            self.address.clone(),
            receiver_addr.to_string(),
            TransactionType::DebitCard,
        );

        Ok(())
    }
}
