use ed25519_dalek::Keypair;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utxo::{TxInput, TxOutput, UTXO};
use uuid::Uuid;

pub mod transactions;
pub mod utxo;

/// Represents a blockchain transaction that transfers value between addresses
///
/// A transaction consists of inputs (consumed UTXOs) and outputs (created UTXOs).
/// Each transaction has a unique ID computed from its contents.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Transaction {
    pub inputs: Vec<TxInput>,
    pub outputs: Vec<TxOutput>,
    pub id: String,
}

/// Possible errors that can occur during transaction processing
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TransactionError {
    InsufficientBalance,
    InvalidSignature,
}

/// A pool of unspent transaction outputs indexed by transaction ID and output index
type UTXOPool = HashMap<(String, usize), UTXO>;

impl Transaction {
    /// Creates a new transaction transferring funds from sender to recipient
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

        let tx = Self {
            id: Uuid::new_v4().to_string(),
            inputs,
            outputs,
        };
        Ok(tx)
    }

    /// Creates a new coinbase transaction that generates new coins for a miner
    ///
    /// Coinbase transactions have no inputs and create new currency.
    pub fn new_coinbase(recipient: &str, reward: usize) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            inputs: vec![],
            outputs: vec![TxOutput::new(recipient, reward)],
        }
    }
}
