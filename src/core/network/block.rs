use crate::core::block::Block;

/// Methods are simulated while proper network management is added
impl Block {
    #[allow(dead_code)]
    pub fn listen_from_network() {
        todo!()
    }

    #[allow(dead_code)]
    pub fn broadcast(&self) -> Self {
        self.clone()
    }
}
