use super::Chain;
use crate::{
    block::Block,
    crypto::is_valid_signature,
    transaction::{Transaction, TransactionType},
    wallet::Wallet,
};
use ed25519_dalek::PublicKey;

const MAX_TRANSACTIONS: usize = 1;

impl Chain {
    ///
    pub fn add_deposit(&mut self, tx: Transaction, pk: &PublicKey) -> Result<(), ()> {
        // Verify signature
        match tx.signature {
            Some(ref signature) if is_valid_signature(&tx, signature, &pk) => {}
            _ => return Err(()),
        }

        // Step 2: Add to last block
        assert!(self.get_last_block().is_some());
        self.mempool.push(tx);

        if let Some(block) = self.get_last_block_mut() {
            let last_hash = block.get_hash().unwrap();
            if self.mempool.len() == MAX_TRANSACTIONS {
                let mut new_block = Block::new(&last_hash, vec![]);

                for tx_x in self.mempool.drain(..) {
                    let _ = new_block.add_transaction(tx_x.clone()); // invalid txs are skipped
                }
                new_block.mine(3);
                self.blocks.push(new_block);
            }
        }
        Ok(())
    }

    pub fn add_transaction(&mut self, tx: Transaction, pk: &PublicKey) -> Result<(), ()> {
        // Step 1: Validations
        if self.balance_of(&tx.from_addr) < tx.amount() {
            return Err(());
        }
        // Verify signature
        match tx.signature {
            Some(ref signature) if is_valid_signature(&tx, signature, &pk) => {}
            _ => return Err(()),
        }

        // Step 2: Add to last block
        assert!(self.get_last_block().is_some());
        self.mempool.push(tx);

        if let Some(block) = self.get_last_block_mut() {
            let last_hash = block.get_hash().unwrap();
            if self.mempool.len() == MAX_TRANSACTIONS {
                let mut new_block = Block::new(&last_hash, vec![]);

                for tx_x in self.mempool.drain(..) {
                    let _ = new_block.add_transaction(tx_x.clone()); // invalid txs are skipped
                }
                new_block.mine(3);
                self.blocks.push(new_block);
            }
        }
        Ok(())
    }

    // TODO!: Verify this function because it is not done
    pub fn deposit_to(&mut self, wallet: &Wallet, amount: usize) -> usize {
        let tx = Transaction::new(amount, "", &wallet.address, TransactionType::Cash);
        let amount = tx.amount();
        self.add_transaction(tx, wallet.get_pk()).unwrap();
        amount
    }

    /// Returns a list of all transactions in the blockchain
    pub fn get_transaction_list(&self) -> Vec<Transaction> {
        self.blocks
            .iter()
            .flat_map(|block| block.get_transactions().iter().cloned())
            .collect()
    }

    pub fn search_transaction(&self, id: &str) -> Option<&Transaction> {
        self.blocks
            .iter()
            .flat_map(|block| block.get_transactions().iter())
            .find(|&transaction| transaction.id == id)
    }
}
