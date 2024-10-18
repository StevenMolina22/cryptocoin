use crate::blockchain::Blockchain;
use crate::common::Transaction;
use crate::common::{TransactionType, Wallet};

impl Wallet {
    pub fn new(address: &str) -> Self {
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

    pub fn get_transactions(&self, blockchain: &mut Blockchain) -> Vec<Transaction> {
        blockchain.get_transaction_list()
    }

    pub fn get_balance(&self) -> usize {
        self.balance
    }

    pub fn transfer(
        &mut self,
        amount: usize,
        receiver_addr: &str,
        blockchain: &mut Blockchain,
    ) -> Result<(), ()> {
        let transaction = Transaction::new(
            String::from(""),
            self.address.clone(),
            receiver_addr.to_string(),
            TransactionType::DebitCard,
        );

        blockchain.add_transaction(transaction)
    }
}
