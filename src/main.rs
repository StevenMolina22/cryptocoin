use common::User;
use transaction::{Transaction, TransactionType};
use wallet::Wallet;
mod block;
mod blockchain;
mod common;
mod crypto;
mod transaction;
mod wallet;

fn main() {
    let mut blockchain_instance = blockchain::Blockchain::new();
    let keypair1 = crypto::generate_key_pair();
    // let keypair2 = crypto::generate_key_pair();

    let sender = User {
        id: 1,
        name: String::from("Alice"),
        hashed_password: String::from("hashed_password_1"),
        pk: keypair1.public.to_bytes(),
        sk: keypair1.secret.to_bytes(),
    };

    let mut sender_wallet = Wallet::new("wallet_1");
    let receiver_wallet = Wallet::new("wallet_2");

    let transaction = Transaction::new(
        String::from("1"),
        100,
        sender_wallet.address.clone(),
        receiver_wallet.address.clone(),
        TransactionType::WireTransfer,
    );

    // Print the transaction details
    println!("Transaction: {:#?}", transaction);
    sender_wallet.transfer(100, &receiver_wallet.address, &mut blockchain_instance);
}
