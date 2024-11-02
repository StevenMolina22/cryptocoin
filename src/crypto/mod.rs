use ed25519_dalek::Keypair;
use rand::rngs::OsRng;
use std::hash::{DefaultHasher, Hash, Hasher};

use crate::{common::Signature, transaction::Transaction};

mod keys;
mod signatures;

pub fn verify_signature(transaction: &Transaction, signature: &Signature, pk: [u8; 32]) -> bool {
    false
}

pub fn generate_signature(transaction: &Transaction, sk: [u8; 32]) -> Signature {
    let mut hasher = DefaultHasher::new();
    transaction.hash(&mut hasher);
    let hash_value = hasher.finish();
    let hash_value = format!("{:016x}", hash_value);
    Signature::new(hash_value, sk)
}

pub fn generate_key_pair() -> Keypair {
    let mut csprng = OsRng {};
    Keypair::generate(&mut csprng)
}
