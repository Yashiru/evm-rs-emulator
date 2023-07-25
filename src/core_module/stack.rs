use super::utils::errors::ExecutionError;

/// A stack data structure used in the Ethereum Virtual Machine (EVM) to store and manipulate data.
#[derive(Debug)]
pub struct Stack {
    /// The stack itself
    pub stack: Vec<[u8; 32]>,
}

/// Implements a stack data structure for the EVM emulator.
impl Stack {
    /// Creates a new stack instance.
    pub fn new() -> Self {
        Self { stack: vec![] }
    }

    /// Pushes a 32-byte word onto the stack.
    ///
    /// # Arguments
    ///
    /// * `word` - A 32-byte array representing the word to be pushed onto the stack.
    ///
    /// # Errors
    ///
    /// Returns an `ExecutionError` if the stack is too deep (i.e., has more than 1024 elements).
    pub fn push(&mut self, word: [u8; 32]) -> Result<(), ExecutionError> {
        // Check if the stack is too deep
        if self.stack.len() >= 1024 {
            // Return an error
            return Err(ExecutionError::StackTooDeep);
        }

        Ok(self.stack.push(word))
    }

    /// Pop a word off the stack
    ///
    /// # Errors
    ///
    /// Returns an `ExecutionError` if the stack is empty.
    pub fn pop(&mut self) -> Result<[u8; 32], ExecutionError> {
        // Check if the stack is empty
        if self.stack.is_empty() {
            // Return an error
            return Err(ExecutionError::StackTooSmall);
        }

        Ok(self.stack.pop().unwrap())
    }

    /// Duplicate a word on the stack
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the word to duplicate, counting from the top of the stack
    ///
    /// # Errors
    ///
    /// Returns an `ExecutionError` if the stack is too small to perform the operation
    ///
    /// # Returns
    ///
    /// Returns the duplicated word if successful
    pub fn dup(&mut self, index: usize) -> Result<[u8; 32], ExecutionError> {
        // Check if the stack is long enough
        if self.stack.len() < index {
            return Err(ExecutionError::StackTooSmall);
        }

        let word = self.stack[self.stack.len() - index];
        self.stack.push(word);

        Ok(word)
    }

    /// Swaps the word at the top of the stack with the word at the specified index.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the word to swap with the top of the stack.
    ///
    /// # Errors
    ///
    /// Returns an `ExecutionError::StackTooSmall` error if the stack is not long enough to perform the swap.
    ///
    /// # Returns
    ///
    /// Returns an array containing the two swapped words.
    pub fn swap(&mut self, index: usize) -> Result<[[u8; 32]; 2], ExecutionError> {
        // Check if the stack is long enough
        if self.stack.len() < index {
            return Err(ExecutionError::StackTooSmall);
        }

        let len = self.stack.len();

        let word1 = self.stack[len - 1];
        let word2 = self.stack[len - 1 - index];

        self.stack[len - 1] = word2;
        self.stack[len - 1 - index] = word1;

        Ok([word1, word2])
    }
}

/// Implements the `Clone` trait for the `Stack` struct.
impl Clone for Stack {
    /// Returns a new instance of `Stack` with the same elements as `self`.
    fn clone(&self) -> Self {
        Self {
            stack: self.stack.clone(),
        }
    }
}
