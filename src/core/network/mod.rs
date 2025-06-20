use std::collections::VecDeque;

use crate::wallet::Wallet;

use super::{block::Block, chain::BlockChain, transaction::Transaction};

pub mod block;
pub mod blockchain;
pub mod transaction;

#[derive(Debug)]
enum Message {
    Transaction(Transaction),
    Block(Block),
}

// Simulated module till it is implemented
#[derive(Debug)]
pub struct Node {
    pub wallet: Wallet,
    pub chain: BlockChain,
    pub messages: VecDeque<Message>,
}

#[derive(Debug)]
pub struct Network {
    pub nodes: Vec<Node>,
}

impl Node {
    pub fn new() -> Self {
        Node {
            wallet: Wallet::new(),
            chain: BlockChain::new(),
            messages: VecDeque::new(),
        }
    }

    pub fn sync_chain(&mut self, new_chain: BlockChain) {
        if new_chain.blocks.len() > self.chain.blocks.len() {
            self.chain = new_chain;
        }
    }
}

impl Network {
    pub fn new() -> Self {
        Network { nodes: Vec::new() }
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.push(node);
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
}
