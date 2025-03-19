use ed25519_dalek::{ed25519::signature::SignerMut, Keypair, PublicKey, Signature, SignatureError};
use serde::{Deserialize, Serialize};

/// Transaction input that references an existing UTXO to be spent
///
/// Used to prove ownership of an existing UTXO by including a signature
/// created with the owner's private key.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TxInput {
    pub tx_id: String,
    pub index: usize,
    #[serde(skip_serializing)]
    signature: Signature,
}

/// Transaction output that defines a new UTXO to be created
///
/// Specifies an amount and recipient for a new unspent transaction output.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TxOutput {
    pub amount: usize,
    pub recipient: String,
}

/// Unspent Transaction Output (UTXO) representing a discrete amount of currency owned by someone
///
/// UTXOs are stored in the UTXO pool and are consumed as inputs in new transactions.
/// The UTXO model is fundamental to how transactions work in this blockchain.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UTXO {
    pub tx_id: String,
    pub index: usize,
    pub amount: usize, // satoshis: (1 / 1000000) of a coin
    pub recipient: String,
}

impl TxInput {
    /// Creates a new transaction input by signing the reference to a UTXO
    pub fn new(tx_id: &str, index: usize, keypair: &mut Keypair) -> Self {
        let serialized_data = format!("{}:{}", tx_id, index).as_bytes().to_vec();

        TxInput {
            tx_id: tx_id.to_string(),
            index,
            signature: keypair.sign(&serialized_data),
        }
    }

    /// Validates that this input was signed by the owner of the public key
    pub fn is_valid(&self, pk: &PublicKey) -> Result<(), SignatureError> {
        let txinput_bytes = format!("{}:{}", self.tx_id, self.index).as_bytes().to_vec();
        pk.verify_strict(&txinput_bytes, &self.signature)
    }
}

impl TxOutput {
    /// Creates a new transaction output
    ///
    /// # Arguments
    /// * `recipient` - The address of the recipient
    /// * `amount` - The amount to transfer
    pub fn new(recipient: &str, amount: usize) -> Self {
        TxOutput {
            recipient: recipient.to_string(),
            amount,
        }
    }
}

impl UTXO {
    /// Creates a new Unspent Transaction Output
    ///
    /// # Arguments
    /// * `txid` - The transaction ID that created this UTXO
    /// * `idx` - The index of this output in the transaction
    /// * `amount` - The amount of currency in this UTXO
    /// * `recipient` - The address of the owner
    pub fn new(txid: &str, idx: usize, amount: usize, recipient: &str) -> Self {
        UTXO {
            tx_id: txid.to_string(),
            index: idx,
            amount,
            recipient: recipient.to_string(),
        }
    }
}
