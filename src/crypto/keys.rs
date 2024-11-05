use ed25519_dalek::Keypair;
use rand::rngs::OsRng;

pub fn generate_key_pair() -> Keypair {
    let mut csprng = OsRng {};
    Keypair::generate(&mut csprng)
}
