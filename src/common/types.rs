#[derive(Debug, Hash, Clone)]
pub struct Date {
    day: u8,
    month: u8,
    year: u32,
}

#[derive(Debug)]
pub struct Signature {
    hash: Vec<u8>,
    sk: [u8; 32],
}

#[derive(Debug, Hash, Clone)]
pub struct User {
    pub id: usize,
    pub name: String,
    pub hashed_password: String,
    pub pk: [u8; 32],
    pub sk: [u8; 32],
}

#[derive(Debug, Hash, Clone)]
pub struct Transaction {
    pub id: String,
    pub sender_addr: String,
    pub receiver_addr: String,
    date: Date,
    debit: usize,
    credit: usize,
    transaction_type: TransactionType,
}

#[derive(Debug, Hash, Clone)]
pub enum TransactionType {
    Cash,
    EFT,
    Check,
    CreditCard,
    DebitCard,
    WireTransfer,
}

#[derive(Debug)]
pub enum TransactionStatus {
    Pending,
    Completed,
    Failed,
}

#[derive(Debug)]
pub struct Wallet {
    // user: User,
    pub address: String,
    pub balance: usize,
}
