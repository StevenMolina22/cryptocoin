use crate::core::transaction::Transaction;

// Methods are simulated while proper network management is implemented
impl Transaction {
    #[allow(dead_code)]
    pub fn listen_from_network() -> Vec<Self> {
        todo!()
    }

    /// function to simulate broadcasting
    #[allow(dead_code)]
    pub fn broadcast(&self) -> Self {
        self.clone()
    }
}
