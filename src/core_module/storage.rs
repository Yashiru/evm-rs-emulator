use std::collections::HashMap;

use super::utils::errors::ExecutionError;

#[derive(Debug)]
pub struct Storage {
    pub state: HashMap<[u8; 32], [u8; 32]>
}

impl Storage {
    pub fn new() -> Self {
        Self {
            state: HashMap::new(),
        }
    }

    // Load a 32 bytes word from storage
    pub unsafe fn sload(&self, slot: [u8; 32]) -> Result<[u8; 32], ExecutionError> {
        match self.state.get(&slot) {
            Some(value) => Ok(value.clone()),
            None => Ok([0u8; 32]),
        }

    }

    // Store a 32 bytes word in storage
    pub unsafe fn sstore(&mut self, slot: [u8; 32], value: [u8; 32]) -> Result<(), ExecutionError> {
        self.state.insert(slot, value);

        Ok(())
    }
}