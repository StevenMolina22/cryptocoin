use wallet::Wallet;
mod block;
mod blockchain;
mod common;
mod crypto;
mod transaction;
mod wallet;

fn main() {
    let mut bc = blockchain::Blockchain::new();
    let mut sender_wallet = Wallet::new("wallet_1");
    let receiver_wallet = Wallet::new("wallet_2");

    // Print the transaction details
    match sender_wallet.transfer(100, receiver_wallet.get_address(), &mut bc) {
        Ok(_) => {
            println!("Transaction successful");
        }
        Err(_) => {
            println!("Transaction failed");
        }
    }

    for transaction in bc.get_transaction_list() {
        println!("{:?}", transaction)
    }
}
