use core::block::Block;
use core::chain::BlockChain;

use wallet::Wallet;
mod core;
mod crypto;
mod wallet;

/// Bitcoin like cryptocurrency implementation
/// uses a PoW system along with UXTOs for balance handling
fn main() {
    let mut wallet = Wallet::new(BlockChain::new());

    wallet.transfer("receiver", 10).unwrap();

    for transaction in wallet.get_transactions() {
        println!("{:#?}", transaction)
    }
}

fn setup_miner(miner: &str, bc: &mut BlockChain) {
    let mut coinbase_block = Block::new(&bc.last_hash(), vec![]);
    coinbase_block.mine(bc.difficulty as usize);
}

// Note: Some useful invariants
// - Every block in the chain has a valid hash (is_some)
// - Every transaction in a block has a valid signature
// -
