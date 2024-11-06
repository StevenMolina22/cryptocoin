use super::{Transaction, TransactionStatus};

impl Transaction {
    pub fn get_status(&self) -> TransactionStatus {
        TransactionStatus::Pending
    }
}
