use ed25519_dalek::{ed25519::signature::SignerMut, Keypair, PublicKey, Signature, SignatureError};
use serde::{Deserialize, Serialize};

// Serves as a reference to an UTXO
// its used for validation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TxInput {
    pub tx_id: String,
    pub index: usize,
    #[serde(skip_serializing)]
    signature: Signature,
}

// Serves as a blueprint for a new UTXO
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TxOutput {
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
    pub amount: usize, // satoshis: (1 / 1000000) of a coin
    pub recipient: String,
}

impl TxInput {
    pub fn new(tx_id: &str, index: usize, keypair: &mut Keypair) -> Self {
        let serialized_data = format!("{}:{}", tx_id, index).as_bytes().to_vec();

        TxInput {
            tx_id: tx_id.to_string(),
            index,
            signature: keypair.sign(&serialized_data),
        }
    }

    pub fn is_valid(&self, pk: &PublicKey) -> Result<(), SignatureError> {
        let txinput_bytes = format!("{}:{}", self.tx_id, self.index).as_bytes().to_vec();
        pk.verify_strict(&txinput_bytes, &self.signature)
    }
}

impl TxOutput {
    pub fn new(recipient: &str, amount: usize) -> Self {
        TxOutput {
            recipient: recipient.to_string(),
            amount,
        }
    }
}

impl UTXO {
    pub fn new(txid: &str, idx: usize, amount: usize, recipient: &str) -> Self {
        UTXO {
            tx_id: txid.to_string(),
            index: idx,
            amount,
            recipient: recipient.to_string(),
        }
    }
}
