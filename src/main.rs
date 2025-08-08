use core::{
    network::{
        node::{Message, Node},
        Network,
    },
    transaction::Transaction,
};
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
/// - Implement Canonical and Dynamic UTXO set
/// - Implement a fork handling system
/// - Improve error handling
/// - Improve testing
/// - Define clearer interfaces
fn main() -> Result<(), String> {
    let mut network = Network::new();

    // add nodes
    for _ in 0..3 {
        network.add_node(Node::new());
    }
    let [sender_id, receiver_id, miner_id] =
        network.nodes.keys().collect::<Vec<_>>().try_into().unwrap();

    network
        .get_mut(sender_id)
        .ok_or("Sender not found")?
        .chain
        .mine_funds_for(sender_id);

    // Create and broadcast transactions
    let transaction_amounts = [3 * 1_000_000, 5 * 1_000_000, 10 * 1_000_000];
    for amount in transaction_amounts.iter() {
        let tx = transfer(
            network.get_mut(sender_id).ok_or("Sender not found")?,
            &network
                .get(receiver_id)
                .ok_or("Receiver not found")?
                .wallet
                .address,
            *amount,
        )
        .unwrap();
        network.broadcast_transaction(tx);
    }

    // Process transactions from the network
    for node in network.nodes.iter_mut() {
        while let Some(message) = node.messages.pop_front() {
            if let Message::Transaction(tx) = message {
                node.add_to_mempool(tx, &sender.wallet.keypair.public)
                    .unwrap();
            }
        }
    }

    // Miner mines a new block
    let block = network
        .get_mut(miner_id)
        .ok_or("Miner not found")?
        .mine_block(
            network
                .get(miner_id)
                .ok_or("Miner not found")?
                .chain
                .mempool
                .clone(),
            &network
                .get(sender_id)
                .ok_or("Sender not found")?
                .wallet
                .keypair
                .public,
        );
    broadcast_block(&mut network, miner_id, block)?;

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
