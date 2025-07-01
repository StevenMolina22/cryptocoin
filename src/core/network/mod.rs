use super::{block::Block, transaction::Transaction};
use node::{Message, Node};
pub mod block;
pub mod blockchain;
pub mod node;
pub mod transaction;

// Simulated module till it is implemented

#[derive(Debug)]
pub struct Network {
    pub nodes: Vec<Node>,
}

impl Network {
    pub fn new() -> Self {
        Network { nodes: Vec::new() }
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.push(node);
    }

    // use BFS and visited nodes to broadcast a message to all nodes
    pub fn broadcast(&mut self, node_id: &str, message: Message) {
        todo!()
    }

    pub fn broadcast_transaction(&mut self, tx: Transaction) {
        for node in &mut self.nodes {
            node.messages.push_back(Message::Transaction(tx.clone()));
        }
    }

    pub fn broadcast_block(&mut self, block: Block) {
        for node in &mut self.nodes {
            node.messages.push_back(Message::Block(block.clone()));
        }
    }

    pub fn get(&self, node_id: &str) -> Option<&Node> {
        self.nodes.iter().find(|node| node.id == node_id)
    }

    pub fn get_mut(&mut self, node_id: &str) -> Option<&mut Node> {
        self.nodes.iter_mut().find(|node| node.id == node_id)
    }
}
