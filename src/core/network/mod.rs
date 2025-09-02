use std::collections::HashMap;

use super::{block::Block, transaction::Transaction};
use node::{Message, Node};
pub mod block;
pub mod blockchain;
pub mod node;
pub mod transaction;

#[derive(Debug)]
pub struct Network {
    pub nodes: HashMap<String, Node>,
}

impl Network {
    pub fn new() -> Self {
        Network {
            nodes: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.id.clone(), node);
    }

    pub fn broadcast(&mut self, node_id: &str, message: Message) {
        todo!()
    }

    pub fn broadcast_transaction(&mut self, tx: Transaction) {
        for node in self.nodes.values_mut() {
            node.messages.push_back(Message::Transaction(tx.clone()));
        }
    }

    pub fn broadcast_block(&mut self, block: Block) {
        for node in &mut self.nodes.values_mut() {
            node.messages.push_back(Message::Block(block.clone()));
        }
    }

    pub fn get(&self, node_id: &str) -> Option<&Node> {
        self.nodes.get(node_id)
    }

    pub fn get_mut(&mut self, node_id: &str) -> Option<&mut Node> {
        self.nodes.get_mut(node_id)
    }

    pub fn process_messages(&mut self) {
        for node in self.nodes.values_mut() {
            while let Some(message) = node.messages.pop_front() {
                let pk = node.wallet.keypair.public;
                match message {
                    Message::Transaction(tx) => {
                        node.add_transaction(tx, &pk).unwrap();
                        ()
                    }
                    Message::Block(block) => {
                        node.add_block(block).unwrap();
                        ()
                    }
                }
            }
        }
    }
}
