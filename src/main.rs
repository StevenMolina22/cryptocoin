use core::{
    network::{
        node::{Message, Node},
        Network,
    },
    transaction::Transaction,
};
use ed25519_dalek::SignatureError;
use thiserror::Error;

mod core;
mod crypto;
mod wallet;

/// # Description
/// Bitcoin-like cryptocurrency implementation
///
/// This project implements a simplified cryptocurrency with the following features:
/// - Proof of Work (PoW) consensus mechanism
/// - Unspent Transaction Output (UTXO) model for balance and funds handling
///
/// The main function simulates different nodes working in the network:
/// - Node listening is represented by taking information as input
/// - Node broadcasting is represented by the output of broadcast methods
///
/// This simulation approach is used for testing purposes while proper
/// network management is being developed.
///
/// # Implementation Notes
/// - Validation is intentionally redundant to simulate multiple validations
///   that would occur in a real network
///
/// # TODO
/// - Simulate fraudulent transactions and nodes
/// - Implement Canonical and Dynamic UTXO set
/// - Implement a fork handling system
/// - Improve error handling
/// - Improve testing
/// - Define clearer interfaces
fn main() -> Result<(), MyError> {
    let mut network = Network::new();

    // add nodes
    for _ in 0..3 {
        network.add_node(Node::new());
    }

    let [sender_id, receiver_id, miner_id] = network
        .nodes
        .keys()
        .cloned()
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    network
        .get_mut(&sender_id)
        .ok_or(MyError::NodeNotFound(sender_id.clone()))?
        .chain
        .mine_funds_for(&sender_id);

    // Create and broadcast transactions
    let transaction_amounts = [3 * 1_000_000, 5 * 1_000_000, 10 * 1_000_000];
    for amount in transaction_amounts.iter() {
        let receiver_addr = network
            .get(&receiver_id)
            .ok_or(MyError::NodeNotFound(receiver_id.clone()))?
            .wallet
            .address
            .clone(); // Clone to get an owned String

        let tx = transfer(
            network
                .get_mut(&sender_id)
                .ok_or(MyError::NodeNotFound(sender_id.clone()))?,
            &receiver_addr, // Pass the owned address
            *amount,
        )
        .unwrap();
        network.broadcast_transaction(tx);
    }

    network.process_messages();

    let (mempool, miner_pubkey) = {
        let miner_node = network
            .get(&miner_id)
            .ok_or(MyError::NodeNotFound(miner_id.clone()))?;
        (
            miner_node.chain.mempool.clone(),
            miner_node.wallet.keypair.public,
        )
    };

    let block = network
        .get_mut(&miner_id)
        .ok_or(MyError::NodeNotFound(miner_id.clone()))?
        .mine_block(mempool, &miner_pubkey)?;

    network.broadcast_block(block);

    // Process the new block from the network
    network.process_messages();

    // Print UTXOs for each node
    for node in network.nodes.values() {
        println!(
            "UTXOS for node:
        {:#?}
        ",
            // node.id,
            node.chain.utxos_of(&node.wallet.address)
        );
    }
    Ok(())
}

/// Creates and signs a transaction from a wallet to a receiver
///
/// This function creates a new transaction, validates it with the sender's
/// public key, and returns a broadcast-ready transaction.
fn transfer(
    sender: &mut Node,
    receiver_addr: &str,
    amount: usize,
) -> Result<Transaction, SignatureError> {
    let tx = Transaction::new(
        amount,
        &sender.wallet.address,
        receiver_addr,
        &mut sender.chain.utxos,
        &mut sender.wallet.keypair,
    )
    .unwrap();
    tx.is_valid(&sender.wallet.keypair.public)?;
    Ok(tx.broadcast())
}

#[derive(Error, Debug)]
pub enum MyError {
    #[error("Custom error: {0}")]
    Custom(String),
    #[error("Signature error: {0}")]
    Signature(#[from] SignatureError),
    #[error("Node not found: {0}")]
    NodeNotFound(String),
}
