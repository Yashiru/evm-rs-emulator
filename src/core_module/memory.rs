use std::ptr;
use super::utils::errors::ExecutionError;

use std::mem::size_of_val;

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

    // Extend memory by size
    pub fn extend(&mut self, size: usize) {
        self.heap.extend(vec![0; size]);
    }

    // Load bytes from memory
    pub unsafe fn read(&self, address: usize, size: usize) -> Result<Vec<u8>, ExecutionError> {
        if address + size > self.heap.len() {
            return Err(ExecutionError::OutOfBoundsMemory);
        }

        let ptr = self.heap.as_ptr().add(address);
        let mut data = vec![0; size];
        ptr::copy(ptr, data.as_mut_ptr(), size);

        Ok(data)
    }

    // Load 32 bytes from memory
    pub unsafe fn mload(&self, address: usize) -> Result<[u8; 32], ExecutionError> {
        if address + 32 > self.heap.len() {
            return Err(ExecutionError::OutOfBoundsMemory);
        }
    
        let ptr = self.heap.as_ptr().add(address);
        Ok(ptr::read(ptr as *const [u8; 32]))
    }

    // Store 32 bytes in memory
    pub unsafe fn mstore(&mut self, address: usize, data: [u8; 32]) -> Result<(), ExecutionError> {
        // Check if memory should be extended
        if address + 32 > self.heap.len() {
            self.extend(address + 32 - self.heap.len());
        }

        let ptr = self.heap.as_mut_ptr().add(address);
        ptr::write(ptr as *mut [u8; 32], data);

        Ok(())
    }

    // Get the memory size
    pub fn msize(&self) -> usize {
        size_of_val(&self.heap)
    }
}