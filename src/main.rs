use core::{block::Block, chain::BlockChain, network::Node, transaction::Transaction};
use ed25519_dalek::{PublicKey, SignatureError};
use wallet::Wallet;

mod core;
mod crypto;
mod wallet;

/// **Bitcoin** like cryptocurrency implementation, it uses:
/// - PoW consensus mechanism
/// - UXTOs balance & funds handling
///
/// The main function is a simulation of different nodes
/// working in the network
/// - The listening of a node for information
/// will be represented as taking that information as input,
/// - The broadcasting of a node's information will be the output
/// of the broadcast method
///
/// (This is done for testing purposes,
/// while proper network management is introduced into the app)
///
/// Extra notes:
/// - Validation is redundant to simulate multiple validations
/// in a real network
///
/// **TODO**:
/// - Simulate fraudulent transactions and nodes
/// - Decide if an Arc Blockchain is of more use than a cloned one
/// - Implement Canonical and Dynamic UTXO set
/// - Implement a fork handling system
/// - Improve error handling
fn main() {
    // --- BlockChain bootstrapping
    let chain = BlockChain::new();
    let mut wallet = Wallet::new(chain.clone());
    let receiver = Wallet::new(chain.clone()); // receiver chain state does not get updated
    wallet.blockchain.setup_miner(&wallet.address);

    // --- Wallet user
    let transaction_amounts = [2 * 1_000_000];
    let transactions: Vec<_> = transaction_amounts
        .into_iter()
        .map(|amount| wallet_transfer(&mut wallet, &receiver.address, amount).unwrap())
        .collect();

    // --- Validator node (transactions)
    let mut node_tx = Node::new(chain.clone());
    let validations: Vec<_> = transactions
        .into_iter()
        .map(|tx| node_tx.validate_tx(tx, &wallet.keypair.public).unwrap())
        .collect();

    // --- Miner node
    let mut miner = Node::new(chain.clone());
    let block = miner.mine(validations, &wallet.keypair.public).unwrap();

    // --- Validator node (blocks)
    let mut node_block = Node::new(chain.clone());
    let block = node_block.validate_block(block);

    println!("Block: \n{:#?}", block);
    println!("UTXOS: \n{:#?}", node_block.blockchain.utxos);
}

pub fn wallet_transfer(
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
    pub fn validate_tx(
        &mut self,
        tx: Transaction,
        pk: &PublicKey,
    ) -> Result<Transaction, SignatureError> {
        tx.is_valid(pk)?;
        // update chain
        self.blockchain.mempool.push(tx.clone());
        Ok(tx.broadcast())
    }

    pub fn mine(
        &mut self,
        transactions: Vec<Transaction>,
        pk: &PublicKey,
    ) -> Result<Block, SignatureError> {
        // TODO! validate UTXOs in the mempool before adding to it (Dynamic vs Canonical UTXO set)
        transactions.iter().try_for_each(|tx| tx.is_valid(pk))?;
        transactions
            .into_iter()
            .for_each(|tx| self.blockchain.mempool.push(tx));

        let mut block = Block::new_template(
            &self.blockchain.last_hash(),
            "miner",
            self.blockchain.reward,
            self.blockchain.mempool.clone(),
        );
        block.mine(self.blockchain.difficulty);
        self.blockchain.update_from(block.clone());
        Ok(block.broadcast())
    }

    pub fn validate_block(&mut self, block: Block) -> Result<Block, SignatureError> {
        block.is_valid(self.blockchain.difficulty)?;
        self.blockchain.update_from(block.clone());
        Ok(block.broadcast())
    }
}
