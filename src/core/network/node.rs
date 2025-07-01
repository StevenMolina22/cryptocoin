use std::collections::VecDeque;

use crate::{
    core::{block::Block, chain::BlockChain, transaction::Transaction},
    wallet::Wallet,
};
use ed25519_dalek::{PublicKey, SignatureError};

#[derive(Debug, Clone)]
pub enum Message {
    Transaction(Transaction),
    Block(Block),
}

#[derive(Debug, Clone)]
pub struct Node {
    pub id: String, // Unique identifier for the node
    pub wallet: Wallet,
    pub chain: BlockChain,
    pub messages: VecDeque<Message>,
    peers: Vec<String>, // List of peer node IDs
}

impl Node {
    pub fn new() -> Self {
        let wallet = Wallet::new();
        Self {
            id: wallet.address.clone(),
            wallet,
            chain: BlockChain::new(),
            messages: VecDeque::new(),
            peers: Vec::new(),
        }
    }

    /// Validates a transaction and adds it to the mempool
    pub fn validate_tx(
        &mut self,
        tx: Transaction,
        pk: &PublicKey,
    ) -> Result<Transaction, SignatureError> {
        tx.is_valid(pk)?;
        // Update chain by adding the transaction to the mempool
        self.chain.mempool.push(tx.clone());
        Ok(tx.broadcast())
    }

    /// Mines a new block with the provided transactions
    ///
    /// This function:
    /// 1. Validates all transactions
    /// 2. Adds them to the mempool
    /// 3. Creates a block template with a coinbase transaction
    /// 4. Performs proof-of-work mining
    /// 5. Updates the blockchain with the new block
    ///
    /// # TODO
    /// - Validate UTXOs in the mempool before adding to it (Dynamic vs Canonical UTXO set)
    pub fn mine(
        &mut self,
        transactions: Vec<Transaction>,
        pk: &PublicKey,
    ) -> Result<Block, SignatureError> {
        transactions.iter().try_for_each(|tx| tx.is_valid(pk))?;
        transactions
            .into_iter()
            .for_each(|tx| self.chain.mempool.push(tx));

        let mut block = Block::new_template(
            &self.chain.last_hash(),
            &self.wallet.address,
            self.chain.reward,
            self.chain.mempool.clone(),
        );
        block.mine(self.chain.difficulty);
        self.chain.update_from(block.clone());
        Ok(block.broadcast())
    }

    /// Validates a block and adds it to the chain
    pub fn add_block(&mut self, block: Block) -> Result<Block, SignatureError> {
        block.is_valid(self.chain.difficulty)?;
        self.chain.update_from(block.clone());
        Ok(block.broadcast())
    }
}
