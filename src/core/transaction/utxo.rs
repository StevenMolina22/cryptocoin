use ed25519_dalek::{ed25519::signature::SignerMut, Keypair};

use super::{TransactionInput, TransactionOutput};

impl TransactionInput {
    pub fn new(tx_id: &str, index: usize, keypair: &mut Keypair) -> Self {
        let serialized_data = format!("{}:{}", tx_id, index).as_bytes().to_vec();

        TransactionInput {
            tx_id: tx_id.to_string(),
            index,
            signature: keypair.sign(&serialized_data),
        }
    }
}

impl TransactionOutput {
    pub fn new(recipient: &str, amount: usize) -> Self {
        TransactionOutput {
            recipient: recipient.to_string(),
            amount,
        }
    }
}
