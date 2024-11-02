use crate::common::Signature;

trait Signable {
    fn sign(&self, sk: &[u8; 32]) -> Signature;
    fn verify_signature(&self, signature: &Signature, pk: &[u8; 32]) -> bool;
}
