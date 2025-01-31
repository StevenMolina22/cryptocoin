use super::Wallet;
use crate::core::chain::Chain;

impl Wallet {
    #[allow(dead_code, unused_variables)]
    pub fn deposit_funds(&mut self, amount: usize) -> Result<(), ()> {
        todo!()
    }

    /// ignore for now
    #[allow(dead_code)]
    pub fn withdraw_funds(&mut self, amount: usize, bc: &Chain) -> Result<(), ()> {
        if self.balance(bc) < amount {
            return Err(());
        }
        Ok(())
    }
}
