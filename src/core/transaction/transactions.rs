use ed25519_dalek::{PublicKey, SignatureError};

use super::{Transaction, TransactionType};
use crate::{core::chain::Chain, wallet::Wallet};

impl Transaction {
    pub fn is_valid(&self, pk: &PublicKey) -> Result<(), SignatureError> {
        self.inputs
            .iter()
            .try_for_each(|txinput| txinput.is_valid(&pk))
    }
}

// TODO! Manage transactions states
pub fn transfer(
    from_wallet: &mut Wallet,
    amount: usize,
    to_addr: &str,
    blockchain: &mut Chain,
) -> Result<(), SignatureError> {
    let tx = Transaction::new(
        amount,
        &from_wallet.address,
        to_addr,
        TransactionType::DebitCard,
    );
    tx.is_valid(&from_wallet.keypair.public)?;
    blockchain.add_transaction(tx, &from_wallet.keypair.public)
}

pub fn deposit(
    amount: usize,
    wallet: &Wallet,
    blockchain: &mut Chain,
) -> Result<(), SignatureError> {
    let tx = Transaction::new(amount, "", &wallet.address, TransactionType::DebitCard);

    tx.is_valid(&wallet.keypair.public)?;
    blockchain.add_deposit(tx, &wallet.keypair.public)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_empty_transactions() {
        let wallet = Wallet::new("test_adress");
        let blockchain = Chain::new();
        let transactions = wallet.get_transactions(&blockchain);
        assert_eq!(transactions.len(), 0);
    }

    #[test]
    fn test_transfer() {
        let mut sender_wallet = Wallet::new("sender_address");
        let receiver_wallet = Wallet::new("receiver_address");
        let mut blockchain = Chain::new();

        deposit(1000, &sender_wallet, &mut blockchain).unwrap();

        let initial_sender_balance = sender_wallet.balance(&blockchain);
        let initial_receiver_balance = receiver_wallet.balance(&blockchain);

        let transfer_amount = 50;
        let result = transfer(
            &mut sender_wallet,
            transfer_amount,
            &receiver_wallet.address,
            &mut blockchain,
        );
        assert!(result.is_ok());

        // Check balances
        assert_eq!(
            sender_wallet.balance(&blockchain),
            initial_sender_balance - transfer_amount
        );
        assert_eq!(
            receiver_wallet.balance(&blockchain),
            initial_receiver_balance + transfer_amount
        );

        // Check transaction in blockchain
        let transactions = blockchain.get_transactions();
        assert_eq!(transactions.len(), 2);
        let tx = &transactions[1];
        assert_eq!(tx.amount(), transfer_amount);
        assert_eq!(tx.sender(), sender_wallet.address);
        assert_eq!(tx.receiver(), receiver_wallet.address);
    }
}
