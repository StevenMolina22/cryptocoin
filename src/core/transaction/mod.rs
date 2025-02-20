use chrono::Utc;
use ed25519_dalek::Signature;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod accessors;
pub mod transactions;
pub mod utxo;

// TODO! Add logic for transaction fees to incentivize miners
// TODO! Add amount logic with UTXOs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: String,
    pub from_addr: String,
    pub to_addr: String,
    pub inputs: Vec<TransactionInput>,
    pub outputs: Vec<TransactionOutput>,
    amount: usize,
    timestamp: usize,
    tx_type: TransactionType,
    status: TransactionStatus,
}

// Serves as a reference to an UTXO
// its used for validation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TransactionInput {
    pub tx_id: String,
    pub index: usize,
    #[serde(skip_serializing)]
    signature: Signature,
}

// Serves as a blueprint for a new UTXO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionOutput {
    pub amount: usize,
    pub recipient: String,
}

// Serves as a descrete amount of money own by someone
// its used for transaction creating and validation
// (being stored in the UTXO pool)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UTXO {
    pub tx_id: String,
    pub index: usize,
    pub amount: usize, // satoshis: (1 / 1000000) of a bitcoin
    pub recipient: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TransactionType {
    Cash,
    EFT,
    Check,
    CreditCard,
    DebitCard,
    WireTransfer,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TransactionStatus {
    Pending,
    Completed,
    Failed,
    Expired,
    Confirmed,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TransactionError {
    InsufficientBalance,
    InvalidSignature,
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
            from_addr: sender_addr.to_string(),
            to_addr: receiver_addr.to_string(),
            inputs: vec![],
            outputs: vec![],
            // TODO! choose a timestamp system
            timestamp: Utc::now().timestamp() as usize,
            amount,
            tx_type: transaction_type,
            status: TransactionStatus::Pending,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_transaction() {
        let sender_addr = "sender_address";
        let receiver_addr = "receiver_address";
        let amount = 100;
        let tx_type = TransactionType::Cash;

        let transaction = Transaction::new(amount, sender_addr, receiver_addr, tx_type.clone());

        assert_eq!(transaction.from_addr, sender_addr);
        assert_eq!(transaction.to_addr, receiver_addr);
        assert_eq!(transaction.amount, amount);
        assert_eq!(transaction.tx_type, tx_type);
        assert_eq!(transaction.status, TransactionStatus::Pending);
    }
}
