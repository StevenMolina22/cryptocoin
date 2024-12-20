use super::{Transaction, TransactionStatus};

impl Transaction {
    pub fn get_status(&self) -> TransactionStatus {
        TransactionStatus::Pending
    }
    pub fn amount(&self) -> usize {
        self.amount
    }
    pub fn get_sender(&self) -> String {
        self.from_addr.clone()
    }
    pub fn get_receiver(&self) -> String {
        self.to_addr.clone()
    }
}
