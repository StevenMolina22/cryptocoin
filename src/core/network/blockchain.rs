use crate::core::chain::BlockChain;

/// Methods are simulated while proper network management is added
impl BlockChain {
    #[allow(dead_code)]
    pub fn retrieve_from_network() -> Self {
        todo!()
    }

    /// function to simulate broadcasting
    #[allow(dead_code)]
    pub fn broadcast(&self) -> Self {
        self.clone()
    }
}
