use chain::Chain;
use wallet::{
    transactions::{deposit, transfer},
    Wallet,
};
mod block;
mod chain;
mod common;
mod crypto;
mod transaction;
mod wallet;

fn main() {
    // TODO!: Investigate about bootstrapping a blockchain
    let mut bc = Chain::new();

    let mut from_wallet = Wallet::new("from_wallet");
    let to_wallet = Wallet::new("to_wallet");

    deposit(1000, &mut from_wallet, &mut bc).unwrap();

    match transfer(&mut from_wallet, 10, &to_wallet.address, &mut bc) {
        // calls unwrap on a None
        Err(_) => println!("Transaction failed"),
        Ok(_) => println!("Transaction successful"),
    }

    for transaction in from_wallet.get_transactions(&bc) {
        println!("{:?}", transaction)
    }

    from_wallet.deposit_funds(400).unwrap();
}
