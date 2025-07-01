use core::{network::node::{Message, Node}, transaction::Transaction};
use ed25519_dalek::SignatureError;

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
/// - Evaluate if choosing an Arc<Blockchain> or cloning
/// - Implement Canonical and Dynamic UTXO set
/// - Implement a fork handling system
/// - Improve error handling
/// - Define clearer interfaces
/// - Implement strategic testing
fn main() {
    let mut network = core::network::Network::new();

    // Create nodes
    let mut sender_node = Node::new();
    let mut receiver_node = Node::new();
    let mut miner_node = Node::new();

    // Add nodes to the network
    network.add_node(sender_node.clone());
    network.add_node(receiver_node.clone());
    network.add_node(miner_node.clone());

    // Mine initial funds for the sender
    sender_node.chain.mine_funds_for(&sender_node.wallet.address);
    sender_node.chain.mine_funds_for(&sender_node.wallet.address);
    sender_node.chain.mine_funds_for(&sender_node.wallet.address);

    // Create and broadcast transactions
    let transaction_amounts = [3 * 1_000_000, 5 * 1_000_000, 10 * 1_000_000];
    for amount in transaction_amounts.iter() {
        let tx = transfer(&mut sender_node, &receiver_node.wallet.address, *amount).unwrap();
        network.broadcast_transaction(tx);
    }

    // Process transactions from the network
    for node in network.nodes.iter_mut() {
        while let Some(message) = node.messages.pop_front() {
            if let core::network::node::Message::Transaction(tx) = message {
                node.validate_tx(tx, &sender_node.wallet.keypair.public).unwrap();
            }
        }
    }

    // Miner mines a new block
    let block = miner_node.mine(miner_node.chain.mempool.clone(), &sender_node.wallet.keypair.public).unwrap();
    network.broadcast_block(block);

    // Process the new block from the network
    for node in network.nodes.iter_mut() {
        while let Some(message) = node.messages.pop_front() {
            if let Message::Block(block) = message {
                node.add_block(block).unwrap();
            }
        }
    }

    // Print UTXOs for each node
    for node in network.nodes.iter() {
        println!(
            "UTXOS for node:
        {:#?}
        ",
            // node.id,
            node.chain.utxos_of(&node.wallet.address)
        );
    }
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
