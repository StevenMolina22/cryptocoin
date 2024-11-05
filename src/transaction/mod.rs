use crate::common::Date;
use ed25519_dalek::Signature;

#[derive(Debug, Clone, serde::Serialize)]
pub struct Transaction {
    pub id: String,
    pub sender_addr: String,
    pub receiver_addr: String,
    pub signature: Option<Signature>,
    amount: usize,
    date: Date,
    transaction_type: TransactionType,
}

#[derive(Debug, Hash, Clone, serde::Serialize)]
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
    Expired,
    Confirmed,
}

impl Transaction {
    pub fn new(
        id: String,
        amount: usize,
        sender_addr: String,
        receiver_addr: String,
        transaction_type: TransactionType,
    ) -> Self {
        Self {
            id,
            sender_addr,
            receiver_addr,
            signature: None,
            date: Date::new(0, 0, 2000),
            amount,
            transaction_type,
        }
    }

    pub fn get_status(&self) -> TransactionStatus {
        TransactionStatus::Pending
    }

    pub fn validate(&self) -> bool {
        true
    }
}
