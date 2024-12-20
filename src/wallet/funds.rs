use crate::chain::Chain;

use super::Wallet;

impl Wallet {
    pub fn deposit_funds(&mut self, amount: usize) -> Result<(), ()> {
        Ok(())
    }

    /// ignore for now
    pub fn withdraw_funds(&mut self, amount: usize, bc: &Chain) -> Result<(), ()> {
        if self.balance(bc) < amount {
            return Err(());
        }
        Ok(())
    }
}
