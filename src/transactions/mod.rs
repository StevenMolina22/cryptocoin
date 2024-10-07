use std::hash::{DefaultHasher, Hash, Hasher};

#[derive(Debug, Hash)]
pub struct Date {
    day: u8,
    month: u8,
    year: u8,
}

#[derive(Debug)]
pub struct Signature {
    hash: String,
    sk: String,
}

#[derive(Debug, Hash)]
pub struct User {
    pub id: usize,
    pub name: String,
    pub hashed_password: String,
    pub pk: String,
    pub sk: String,
}

#[derive(Debug, Hash)]
pub struct Transaction {
    id: String,
    sender: User,
    receiver: User,
    date: Date,
    debit: usize,
    credit: usize,
    signature: usize,
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

impl Transaction {
    pub fn new(
        id: String,
        sender: User,
        receiver: User,
        date: Date,
        debit: usize,
        credit: usize,
        signature: usize,
        transaction_type: TransactionType,
    ) -> Self {
        Self {
            id,
            sender,
            receiver,
            date,
            debit,
            credit,
            signature,
            transaction_type,
        }
    }

    pub fn get_balance(&self) -> isize {
        self.debit as isize - self.credit as isize
    }
    pub fn generate_signature(&self, sk: String) -> Signature {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        let hash_value = hasher.finish();
        let hash_value = format!("{:016x}", hash_value);
        Signature::new(hash_value, sk)
    }

    pub fn verify(&self) -> bool {
        false
    }
}

impl Signature {
    pub fn new(message: String, sk: String) -> Signature {
        Signature {
            hash: String::from(message),
            sk,
        }
    }
    pub fn is_valid(hash: &str) -> bool {
        false
    }
}

impl Date {
    pub fn new(day: u8, month: u8, year: u8) -> Date {
        Date { day, month, year }
    }
}
