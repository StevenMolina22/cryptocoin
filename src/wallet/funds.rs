use super::Wallet;

impl Wallet {
    pub fn deposit_funds(&mut self, amount: usize) -> Result<(), ()> {
        self.balance += amount;
        Ok(())
    }

    /// ignore for now
    pub fn withdraw_funds(&mut self, amount: usize) -> Result<(), ()> {
        if self.balance < amount {
            return Err(());
        }
        self.balance -= amount;
        Ok(())
    }
}
