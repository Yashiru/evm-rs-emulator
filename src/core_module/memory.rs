use std::ptr;
use super::errors::InterpreterError;

#[derive(Debug)]
pub struct Memory {
    pub heap: Vec<u8>,
}

impl Memory {
    pub fn new(size: usize) -> Self {
        Self {
            heap: vec![0; size],
        }
    }

    // Load 32 bytes from memory
    pub unsafe fn mload(&self, address: usize) -> Result<[u8; 32], InterpreterError> {
        if(address + 32 > self.heap.len()) {
            return Err(InterpreterError::OutOfBounds);
        }
    
        let ptr = self.heap.as_ptr().add(address);
        Ok(ptr::read(ptr as *const [u8; 32]))
    }

    // Store 32 bytes in memory
    pub unsafe fn mstore(&mut self, address: usize, data: [u8; 32]) -> Result<(), InterpreterError> {
        // Check if the address is out of bounds
        if(address + 32 > self.heap.len()) {
            return Err(InterpreterError::OutOfBounds);
        }

        let ptr = self.heap.as_mut_ptr().add(address);
        ptr::write(ptr as *mut [u8; 32], data);

        Ok(())
    }

    // Get the memory size
    pub fn msize(&self) -> usize {
        self.heap.len()
    }
}