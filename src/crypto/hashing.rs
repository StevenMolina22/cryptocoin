use sha3::{Digest, Sha3_256};

use crate::block::Block;

pub fn hash_block(block: &Block) -> String {
    let bytes = bincode::serialize(block).unwrap();
    let hash_value = Sha3_256::digest(&bytes);
    format!("{:016x}", hash_value)
}
