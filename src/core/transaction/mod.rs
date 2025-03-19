use ed25519_dalek::Keypair;
use serde::{Deserialize, Serialize};
use sha3::{Digest, Sha3_256};
use std::collections::HashMap;
use utxo::{TxInput, TxOutput, UTXO};

pub mod transactions;
pub mod utxo;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Transaction {
    #[serde(skip_serializing)]
    pub id: String,
    pub inputs: Vec<TxInput>,
    pub outputs: Vec<TxOutput>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TransactionError {
    InsufficientBalance,
    InvalidSignature,
}

type UTXOPool = HashMap<(String, usize), UTXO>;

impl Transaction {
    pub fn new(
        amount: usize,
        sender: &str,
        recipient: &str,
        utxos: &UTXOPool,
        keypair: &mut Keypair,
    ) -> Result<Self, ()> {
        // Search for available inputs
        // choose smaller utxos
        let mut available_utxos: Vec<_> = utxos
            .iter()
            .filter(|(_, utxo)| utxo.recipient == sender)
            .collect();
        available_utxos.sort_by_key(|(_, utxo)| utxo.amount);

        let mut acc_amount = 0;
        let mut inputs = vec![];
        for ((txid, idx), utxo) in available_utxos {
            if acc_amount >= amount {
                break;
            }
            acc_amount += utxo.amount;
            inputs.push(TxInput::new(txid, *idx, keypair));
        }

        if acc_amount < amount {
            return Err(());
        }

        // Create outputs
        let mut outputs = vec![TxOutput::new(recipient, amount)];
        if acc_amount > amount {
            outputs.push(TxOutput::new(sender, acc_amount - amount))
        }

        let mut tx = Self {
            id: String::from(""), // temporary
            inputs,
            outputs,
        };
        tx.id = tx.compute_id();
        Ok(tx)
    }

    fn compute_id(&self) -> String {
        let serialized = serde_json::to_vec(self).unwrap();
        let mut hasher = Sha3_256::new();
        hasher.update(&serialized);

        format!("{:x}", hasher.finalize())
    }

    pub fn new_coinbase(recipient: &str, reward: usize) -> Self {
        let mut tx = Self {
            id: String::from(""), // temporary
            inputs: vec![],
            outputs: vec![TxOutput::new(recipient, reward)],
        };
        tx.id = tx.compute_id();
        tx
    }
}
