use crate::common::Date;
use ed25519_dalek::{PublicKey, Signature};
use uuid::Uuid;

pub mod accessors;

#[derive(Debug, Clone, serde::Serialize)]
pub struct Transaction {
    pub id: String,
    pub sender_addr: String,
    pub receiver_addr: String,
    amount: usize,
    date: Date,
    transaction_type: TransactionType,
    #[serde(skip_serializing)]
    pub signature: Option<Signature>,
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
        amount: usize,
        sender_addr: &str,
        receiver_addr: &str,
        transaction_type: TransactionType,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            sender_addr: sender_addr.to_string(),
            receiver_addr: receiver_addr.to_string(),
            signature: None,
            date: Date::new(0, 0, 2000),
            amount,
            transaction_type,
        }
    }
    pub fn sign(&mut self, signature: Signature) {
        self.signature = Some(signature);
    }
    pub fn validate(&self, pk: &PublicKey) -> bool {
        todo!()
    }
}
