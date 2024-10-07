use transactions::{Date, Transaction, TransactionType, User};

mod transactions;

fn main() {
    // Create some users
    let sender = User {
        id: 1,
        name: String::from("Alice"),
        hashed_password: String::from("hashed_password_1"),
        pk: String::from("public_key_1"),
        sk: String::from("secret_key_1"),
    };

    let receiver = User {
        id: 2,
        name: String::from("Bob"),
        hashed_password: String::from("hashed_password_2"),
        pk: String::from("public_key_2"),
        sk: String::from("secret_key_2"),
    };

    // Create a date
    let date = Date::new(15, 10, 23);

    // Create a transaction
    let transaction = Transaction::new(
        String::from("1"),
        sender,
        receiver,
        date,
        100,
        100,
        1,
        TransactionType::WireTransfer,
    );

    // Print the transaction details
    println!("Transaction: {:#?}", transaction);

    // Get the balance of the transaction
    let balance = transaction.get_balance();
    println!("Balance: {}", balance);

    // Generate a signature for the transaction
    let signature = transaction.generate_signature(String::from("secret_key_1"));
    println!("Signature: {:#?}", signature);

    // Verify the transaction (currently always returns false)
    let is_verified = transaction.verify();
    println!("Is Verified: {}", is_verified);
}
