use ed25519_dalek::Keypair;
use serde::{Deserialize, Serialize};
use sha3::{Digest, Sha3_256};
use std::collections::HashMap;
use utxo::{TxInput, TxOutput, UTXO};

pub mod transactions;
pub mod utxo;

/// Represents a blockchain transaction that transfers value between addresses
/// 
/// A transaction consists of inputs (consumed UTXOs) and outputs (created UTXOs).
/// Each transaction has a unique ID computed from its contents.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Transaction {
    #[serde(skip_serializing)]
    pub id: String,
    pub inputs: Vec<TxInput>,
    pub outputs: Vec<TxOutput>,
}

/// Possible errors that can occur during transaction processing
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TransactionError {
    /// Indicates the sender doesn't have enough funds to complete the transaction
    InsufficientBalance,
    /// Indicates the transaction signature verification failed
    InvalidSignature,
}

/// A pool of unspent transaction outputs indexed by transaction ID and output index
type UTXOPool = HashMap<(String, usize), UTXO>;

impl Transaction {
    /// Creates a new transaction transferring funds from sender to recipient
    ///
    /// # Arguments
    /// * `amount` - The amount to transfer
    /// * `sender` - The sender's address
    /// * `recipient` - The recipient's address
    /// * `utxos` - The current UTXO pool to draw funds from
    /// * `keypair` - The sender's keypair for signing inputs
    ///
    /// # Returns
    /// * `Ok(Transaction)` - The created transaction if successful
    /// * `Err(())` - If there are insufficient funds
    pub fn new(
        amount: usize,
        sender: &str,
        recipient: &str,
        utxos: &UTXOPool,
        keypair: &mut Keypair,
    ) -> Result<Self, ()> {
        // Find available UTXOs belonging to the sender
        // Sort by amount to minimize the number of inputs needed
        let mut available_utxos: Vec<_> = utxos
            .iter()
            .filter(|(_, utxo)| utxo.recipient == sender)
            .collect();
        available_utxos.sort_by_key(|(_, utxo)| utxo.amount);

        // Collect enough inputs to cover the amount
        let mut acc_amount = 0;
        let mut inputs = vec![];
        for ((txid, idx), utxo) in available_utxos {
            if acc_amount >= amount {
                break;
            }
            acc_amount += utxo.amount;
            inputs.push(TxInput::new(txid, *idx, keypair));
        }

        // Return error if insufficient funds
        if acc_amount < amount {
            return Err(());
        }

        // Create outputs: one for the recipient and one for change (if any)
        let mut outputs = vec![TxOutput::new(recipient, amount)];
        if acc_amount > amount {
            outputs.push(TxOutput::new(sender, acc_amount - amount))
        }

        let mut tx = Self {
            id: String::from(""), // temporary ID, will be computed
            inputs,
            outputs,
        };
        tx.id = tx.compute_id();
        Ok(tx)
    }

    /// Computes the unique ID for this transaction by hashing its serialized form
    fn compute_id(&self) -> String {
        let serialized = serde_json::to_vec(self).unwrap();
        let mut hasher = Sha3_256::new();
        hasher.update(&serialized);

        format!("{:x}", hasher.finalize())
    }

    /// Creates a new coinbase transaction that generates new coins for a miner
    ///
    /// Coinbase transactions have no inputs and create new currency.
    ///
    /// # Arguments
    /// * `recipient` - The miner's address to receive the reward
    /// * `reward` - The mining reward amount
    pub fn new_coinbase(recipient: &str, reward: usize) -> Self {
        let mut tx = Self {
            id: String::from(""), // temporary ID, will be computed
            inputs: vec![],
            outputs: vec![TxOutput::new(recipient, reward)],
        };
        tx.id = tx.compute_id();
        tx
    }
}
