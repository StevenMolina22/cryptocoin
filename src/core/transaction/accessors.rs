use super::{Transaction, TransactionStatus};

impl Transaction {
    #[allow(dead_code)]
    pub fn status(&self) -> TransactionStatus {
        TransactionStatus::Pending
    }
    pub fn amount(&self) -> usize {
        self.amount
    }
    #[allow(dead_code)]
    pub fn sender(&self) -> String {
        self.from_addr.clone()
    }
    #[allow(dead_code)]
    pub fn receiver(&self) -> String {
        self.to_addr.clone()
    }
}
