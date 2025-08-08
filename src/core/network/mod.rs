use std::collections::HashMap;

use super::{block::Block, transaction::Transaction};
use node::{Message, Node};
pub mod block;
pub mod blockchain;
pub mod node;
pub mod transaction;

// Simulated module till it is implemented

#[derive(Debug)]
pub struct Network {
    // pub nodes: Vec<Node>,
    pub nodes: HashMap<String, Node>,
}

impl Network {
    pub fn new() -> Self {
        Network {
            // nodes: Vec::new(),
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
}
