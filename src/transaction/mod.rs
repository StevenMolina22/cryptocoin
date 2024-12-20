use crate::common::Date;
use ed25519_dalek::Signature;
use uuid::Uuid;

pub mod accessors;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Transaction {
    pub id: String,
    pub from_addr: String,
    pub to_addr: String,
    amount: usize,
    date: Date,
    tx_type: TransactionType,
    status: TransactionStatus,
    #[serde(skip_serializing)]
    pub signature: Option<Signature>,
}

#[derive(Debug, Hash, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub enum TransactionType {
    Cash,
    EFT,
    Check,
    CreditCard,
    DebitCard,
    WireTransfer,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
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
            from_addr: sender_addr.to_string(),
            to_addr: receiver_addr.to_string(),
            signature: None,
            date: Date::new(0, 0, 2000),
            amount,
            tx_type: transaction_type,
            status: TransactionStatus::Pending,
        }
    }
    /// Pre: signatura is valid
    ///
    /// Post: the transaction gets its signature field updated
    pub fn sign(&mut self, signature: Signature) {
        self.signature = Some(signature);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::{generate_key_pair, is_valid_signature, signature_from};

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
        assert!(transaction.signature.is_none());
    }

    #[test]
    fn test_sign_transaction() {
        let sender_addr = "sender_address";
        let receiver_addr = "receiver_address";
        let amount = 100;
        let tx_type = TransactionType::Cash;

        let mut transaction = Transaction::new(amount, sender_addr, receiver_addr, tx_type.clone());
        let keypair = generate_key_pair();
        let signature = signature_from(&transaction, &keypair);

        transaction.sign(signature.clone());

        assert!(transaction.signature.is_some());
        assert_eq!(transaction.signature.unwrap(), signature);
    }

    #[test]
    fn test_verify_signature() {
        let sender_addr = "sender_address";
        let receiver_addr = "receiver_address";
        let amount = 100;
        let tx_type = TransactionType::Cash;

        let transaction = Transaction::new(amount, sender_addr, receiver_addr, tx_type.clone());
        let keypair = generate_key_pair();
        let signature = signature_from(&transaction, &keypair);

        let is_valid = is_valid_signature(&transaction, &signature, &keypair.public);

        assert!(is_valid);
    }

    #[test]
    fn test_verify_invalid_signature() {
        let sender_addr = "sender_address";
        let receiver_addr = "receiver_address";
        let amount = 100;
        let tx_type = TransactionType::Cash;

        let transaction = Transaction::new(amount, sender_addr, receiver_addr, tx_type.clone());
        let keypair = generate_key_pair();
        let signature = signature_from(&transaction, &keypair);

        // Generate a different keypair to create an invalid signature
        let different_keypair = generate_key_pair();
        let is_valid = is_valid_signature(&transaction, &signature, &different_keypair.public);

        assert!(!is_valid);
    }
}
