use ed25519_dalek::{PublicKey, SignatureError};

use super::Transaction;

impl Transaction {
    pub fn is_valid(&self, pk: &PublicKey) -> Result<(), SignatureError> {
        self.inputs
            .iter()
            .try_for_each(|txinput| txinput.is_valid(&pk))
    }
}
