use core::{block::Block, chain::BlockChain, network::Node, transaction::Transaction};
use ed25519_dalek::{PublicKey, SignatureError};
use wallet::Wallet;

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
    // --- BlockChain bootstrapping
    let chain = BlockChain::new();
    let mut wallet = Wallet::new(chain.clone());
    // receiver chain state does not get updated
    let receiver = Wallet::new(chain.clone());

    // get more than one utxo for more than one transaction
    wallet.blockchain.mine_funds_for(&wallet.address);
    wallet.blockchain.mine_funds_for(&wallet.address);
    wallet.blockchain.mine_funds_for(&wallet.address);

    // --- Wallet user
    let transaction_amounts = [3 * 1_000_000, 5 * 1_000_000, 10 * 1_000_000];
    let transactions: Vec<_> = transaction_amounts
        .into_iter()
        .map(|amount| wallet_transfer(&mut wallet, &receiver.address, amount).unwrap())
        .collect();

    // --- Validator node (transactions)
    let mut node_tx = Node::new(Wallet::new(chain.clone()));
    let validations: Vec<_> = transactions
        .into_iter()
        .map(|tx| node_tx.validate_tx(tx, &wallet.keypair.public).unwrap())
        .collect();

    // --- Miner node
    let mut miner = Node::new(Wallet::new(chain.clone()));
    let block = miner.mine(validations, &wallet.keypair.public).unwrap();

    // --- Validator node (blocks)
    let mut node_block = Node::new(Wallet::new(chain.clone()));
    let _ = node_block.validate_block(block);

    println!(
        "UTXOS for wallet: \n{:#?}\n",
        node_block.wallet.blockchain.utxos_of(&wallet.address)
    );
    println!(
        "UTXOS for receiver: \n{:#?}\n",
        node_block.wallet.blockchain.utxos_of(&receiver.address)
    );
    println!(
        "UTXOS for miner: \n{:#?}\n",
        node_block.wallet.blockchain.utxos_of(&miner.wallet.address)
    )
}

/// Creates and signs a transaction from a wallet to a receiver
///
/// This function creates a new transaction, validates it with the sender's
/// public key, and returns a broadcast-ready transaction.
fn wallet_transfer(
    wallet: &mut Wallet,
    receiver: &str,
    amount: usize,
) -> Result<Transaction, SignatureError> {
    let tx = Transaction::new(
        amount,
        &wallet.address,
        receiver,
        &mut wallet.blockchain.utxos,
        &mut wallet.keypair,
    )
    .unwrap();
    tx.is_valid(&wallet.keypair.public)?;
    Ok(tx.broadcast())
}

impl Node {
    /// Validates a transaction and adds it to the mempool
    pub fn validate_tx(
        &mut self,
        tx: Transaction,
        pk: &PublicKey,
    ) -> Result<Transaction, SignatureError> {
        tx.is_valid(pk)?;
        // Update chain by adding the transaction to the mempool
        self.wallet.blockchain.mempool.push(tx.clone());
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
            .for_each(|tx| self.wallet.blockchain.mempool.push(tx));

        let mut block = Block::new_template(
            &self.wallet.blockchain.last_hash(),
            &self.wallet.address,
            self.wallet.blockchain.reward,
            self.wallet.blockchain.mempool.clone(),
        );
        block.mine(self.wallet.blockchain.difficulty);
        self.wallet.blockchain.update_from(block.clone());
        Ok(block.broadcast())
    }

    /// Validates a block and adds it to the blockchain
    pub fn validate_block(&mut self, block: Block) -> Result<Block, SignatureError> {
        block.is_valid(self.wallet.blockchain.difficulty)?;
        self.wallet.blockchain.update_from(block.clone());
        Ok(block.broadcast())
    }
}
