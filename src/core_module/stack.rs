use super::utils::errors::ExecutionError;

#[derive(Debug)]
pub struct Stack {
    pub stack: Vec<[u8; 32]>,
}

impl Stack {
    pub fn new() -> Self {
        Self { stack: vec![] }
    }

    // Push a word onto the stack
    pub fn push(&mut self, word: [u8; 32]) -> Result<(), ExecutionError> {
        // Check if the stack is too deep
        if self.stack.len() >= 1024 {
            // Return an error
            return Err(ExecutionError::StackTooDeep);
        }

        Ok(self.stack.push(word))
    }

    // Pop a word off the stack
    pub fn pop(&mut self) -> Result<[u8; 32], ExecutionError> {
        // Check if the stack is empty
        if self.stack.is_empty() {
            // Return an error
            return Err(ExecutionError::StackTooSmall);
        }

        Ok(self.stack.pop().unwrap())
    }

    // Duplicate a word on the stack
    pub fn dup(&mut self, index: usize) -> Result<[u8; 32], ExecutionError> {
        // Check if the stack is long enough
        if self.stack.len() < index {
            return Err(ExecutionError::StackTooSmall);
        }

        let word = self.stack[self.stack.len() - index];
        self.stack.push(word);

        Ok(word)
    }

    // Swap two words on the stack
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

impl Clone for Stack {
    fn clone(&self) -> Self {
        Self {
            stack: self.stack.clone(),
        }
    }
}
