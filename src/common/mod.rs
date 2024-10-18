#[derive(Debug, Hash)]
pub struct Date {
    day: u8,
    month: u8,
    year: u32,
}

#[derive(Debug)]
pub struct Signature {
    hash: String,
    sk: String,
}

#[derive(Debug, Hash, Clone)]
pub struct User {
    pub id: usize,
    pub name: String,
    pub hashed_password: String,
    pub pk: [u8; 32],
    pub sk: [u8; 32],
}

#[derive(Debug, Hash)]
pub struct Transaction {
    id: String,
    sender_addr: String,
    receiver_addr: String,
    date: Date,
    debit: usize,
    credit: usize,
    transaction_type: TransactionType,
}

#[derive(Debug, Hash)]
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
    pub address: String,
    pub balance: usize,
}

impl Signature {
    pub fn new(message: String, sk: String) -> Signature {
        Signature {
            hash: String::from(message),
            sk,
        }
    }
}

impl Date {
    pub fn new(day: u8, month: u8, year: u32) -> Date {
        Date { day, month, year }
    }
}

impl User {}

impl Transaction {
    pub fn new(
        id: String,
        sender_addr: String,
        receiver_addr: String,
        transaction_type: TransactionType,
    ) -> Self {
        Self {
            id,
            sender_addr,
            receiver_addr,
            date: Date::new(0, 0, 2000),
            debit: 0,
            credit: 0,
            transaction_type,
        }
    }

    pub fn get_status(&self) -> TransactionStatus {
        TransactionStatus::Pending
    }
}
