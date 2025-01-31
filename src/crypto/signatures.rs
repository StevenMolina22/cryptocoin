use crate::core::transaction::Transaction;
use ed25519_dalek::{Keypair, PublicKey, Signature, Signer};

pub fn is_valid_signature(
    transaction: &Transaction,
    signature: &Signature,
    pk: &PublicKey,
) -> bool {
    let transaction_bytes = match bincode::serialize(&transaction) {
        Ok(bytes) => bytes,
        Err(_) => return false,
    };
    pk.verify_strict(&transaction_bytes, signature).is_ok()
}

pub fn signature_from(transaction: &Transaction, keypair: &Keypair) -> Signature {
    let transaction_bytes = bincode::serialize(&transaction).unwrap();
    keypair.sign(&transaction_bytes)
}
