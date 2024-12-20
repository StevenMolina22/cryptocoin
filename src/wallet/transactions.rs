use crate::{
    chain::Chain,
    crypto::{is_valid_signature, signature_from},
    transaction::{Transaction, TransactionType},
};

use super::Wallet;

impl Wallet {
    pub fn get_transactions(&self, blockchain: &Chain) -> Vec<Transaction> {
        blockchain.get_transaction_list()
    }
}

pub fn transfer(
    from_wallet: &mut Wallet,
    amount: usize,
    to_addr: &str,
    blockchain: &mut Chain,
) -> Result<(), ()> {
    // TODO!: Is it necessary to make a balance check here?
    if amount > from_wallet.balance(blockchain) {
        return Err(());
    }
    let mut tx = Transaction::new(
        amount,
        &from_wallet.address,
        to_addr,
        TransactionType::DebitCard,
    );
    // hashes tx without signature field
    let signature = signature_from(&tx, &from_wallet.keypair);
    tx.sign(signature);
    if !is_valid_signature(&tx, &signature, &from_wallet.keypair.public) {
        println!("Signature verification failed");
        return Err(());
    }
    blockchain.add_transaction(tx, &from_wallet.keypair.public)
}

pub fn deposit(amount: usize, wallet: &mut Wallet, blockchain: &mut Chain) -> Result<(), ()> {
    let mut tx = Transaction::new(amount, "", &wallet.address, TransactionType::DebitCard);
    let signature = signature_from(&tx, &wallet.keypair);
    tx.sign(signature);
    if !is_valid_signature(&tx, &signature, &wallet.keypair.public) {
        println!("Signature verification failed");
        return Err(());
    }
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

        deposit(1000, &mut sender_wallet, &mut blockchain).unwrap();
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
        let transactions = blockchain.get_transaction_list();
        assert_eq!(transactions.len(), 2);
        let tx = &transactions[1];
        assert_eq!(tx.amount(), transfer_amount);
        assert_eq!(tx.get_sender(), sender_wallet.address);
        assert_eq!(tx.get_receiver(), receiver_wallet.address);
    }
}
