use ed25519_dalek::SignatureError;

use super::Wallet;
use crate::core::transaction::Transaction;

impl Wallet {
    pub fn get_transactions(&self) -> Vec<Transaction> {
        self.blockchain.get_transactions()
    }

    pub fn transfer(&mut self, receiver: &str, amount: usize) -> Result<(), SignatureError> {
        let tx = Transaction::new(
            amount,
            &self.address,
            receiver,
            &mut self.blockchain.utxos,
            &mut self.keypair,
        )
        .unwrap();
        tx.is_valid(&self.keypair.public)?;
        // TODO! create a system to choose a miner
        self.blockchain
            .include_transaction(tx, &self.keypair.public)
    }
}
