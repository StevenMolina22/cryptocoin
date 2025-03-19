use ed25519_dalek::{PublicKey, SignatureError};

use super::Transaction;

impl Transaction {
    pub fn is_valid(&self, pk: &PublicKey) -> Result<(), SignatureError> {
        self.inputs
            .iter()
            .try_for_each(|txinput| txinput.is_valid(&pk))
    }
}

#[cfg(test)]
mod tests {
    use crate::{core::chain::BlockChain, wallet::Wallet};

    #[test]
    fn test_get_empty_transactions() {
        let wallet = Wallet::new(BlockChain::new());
        let transactions = wallet.get_transactions();
        assert_eq!(transactions.len(), 0);
    }
}
