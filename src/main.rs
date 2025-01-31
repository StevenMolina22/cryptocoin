use core::chain::Chain;
use core::transaction::transactions::{deposit, transfer};
use wallet::Wallet;
mod common;
mod core;
mod crypto;
mod wallet;

fn main() {
    let mut bc = Chain::new();
    let mut from_wallet = Wallet::new("from_wallet");
    let to_wallet = Wallet::new("to_wallet");

    deposit(100, &from_wallet, &mut bc).unwrap();
    deposit(100, &from_wallet, &mut bc).unwrap();
    deposit(100, &from_wallet, &mut bc).unwrap();
    deposit(100, &from_wallet, &mut bc).unwrap();
    deposit(100, &from_wallet, &mut bc).unwrap();

    transfer(&mut from_wallet, 10, &to_wallet.address, &mut bc).unwrap();

    for transaction in from_wallet.get_transactions(&bc) {
        println!("{:#?}", transaction)
    }
}
