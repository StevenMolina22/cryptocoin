use core::chain::BlockChain;
use wallet::Wallet;

mod core;
mod crypto;
mod wallet;

/// Bitcoin like cryptocurrency implementation
/// uses a PoW system along with UXTOs for balance handling
fn main() {
    let mut wallet = Wallet::new(BlockChain::new());

    wallet.blockchain.setup_miner(&wallet.address);
    wallet.transfer("receiver", 10).unwrap();

    for transaction in wallet.get_transactions() {
        println!("{:#?}", transaction)
    }

    println!("utxos: \n{:#?}", wallet.blockchain.utxos);
}

// Note: Some useful invariants
// - Every block in the chain has a valid hash (is_some)
// - Every transaction in a block has a valid signature
