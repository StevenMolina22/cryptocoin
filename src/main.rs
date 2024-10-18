use common::{Date, Transaction, TransactionType, User, Wallet};
use crypto::{generate_signature, verify_signature};

mod blockchain;
pub mod common;
mod crypto;
mod wallet;

fn main() {
    let keypair1 = crypto::generate_key_pair();
    let keypair2 = crypto::generate_key_pair();

    let sender = User {
        id: 1,
        name: String::from("Alice"),
        hashed_password: String::from("hashed_password_1"),
        pk: keypair1.public.to_bytes(),
        sk: keypair1.secret.to_bytes(),
    };

    let wallet = Wallet::new("wallet_1");
    let wallet2 = Wallet::new("wallet_2");

    let transaction = Transaction::new(
        String::from("1"),
        wallet.address.clone(),
        wallet2.address.clone(),
        TransactionType::WireTransfer,
    );

    // Print the transaction details
    println!("Transaction: {:#?}", transaction);

    let signature = generate_signature(&transaction, String::from("secret_key_1"));
    println!("Signature: {:#?}", signature);

    let is_verified = verify_signature(&transaction, signature, sender.pk);
    println!("Is Verified: {}", is_verified);
}
