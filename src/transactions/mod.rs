use std::hash::{DefaultHasher, Hash, Hasher};

#[derive(Debug, Hash)]
struct Date {
    days: u8,
    month: u8,
    year: u8,
}

#[derive(Debug)]
struct Signature {
    hash: String,
    sk: String,
}

#[derive(Debug, Hash)]
struct User {
    id: usize,
    name: String,
    hashed_password: String,
    pk: String,
    sk: String,
}

#[derive(Debug, Hash)]
struct Transaction {
    id: usize,
    sender: User,
    receiver: User,
    date: Date,
    debit: usize,
    credit: usize,
    signature: usize,
    transaction_type: TransactionType,
}

#[derive(Debug, Hash)]
enum TransactionType {
    Cash,
    EFT,
    Check,
    CreditCard,
    DebitCard,
    WireTransfer,
}

impl Transaction {
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
