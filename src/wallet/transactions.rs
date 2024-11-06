use uuid::Uuid;

use crate::{
    chain::Blockchain,
    crypto::{sign_transaction, verify_signature},
    transaction::{Transaction, TransactionType},
};

use super::Wallet;

impl Wallet {
    pub fn get_transactions(&self, blockchain: &Blockchain) -> Vec<Transaction> {
        blockchain.get_transaction_list()
    }

    pub fn transfer(
        &mut self,
        amount: usize,
        receiver_addr: &str,
        blockchain: &mut Blockchain,
    ) -> Result<(), ()> {
        let mut tx = Transaction::new(
            amount,
            &self.address,
            receiver_addr,
            TransactionType::DebitCard,
        );
        let signature = sign_transaction(&tx, &self.keypair);
        tx.sign(signature);
        if !verify_signature(&tx, &signature, &self.keypair.public) {
            println!("Signature verification failed");
            return Err(());
        }
        blockchain.add_transaction(tx, &self.keypair.public)
    }
}
