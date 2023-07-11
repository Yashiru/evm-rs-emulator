#[derive(Debug)]
pub struct Stack {
    pub stack: Vec<[u8; 32]>,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            stack: vec![],
        }
    }

    // Push a word onto the stack
    pub unsafe fn push(&mut self, word: [u8; 32]) {
        self.stack.push(word);
    }

    // Pop a word off the stack
    pub unsafe fn pop(&mut self) -> [u8; 32] {
        self.stack.pop().unwrap()
    }

    // Duplicate a word on the stack
    pub unsafe fn dup(&mut self, index: usize) {
        let word = self.stack[self.stack.len() - index];
        self.stack.push(word);
    }

    // Swap two words on the stack
    pub unsafe fn swap(&mut self, index: usize) {
        let len = self.stack.len();

        let word1 = self.stack[len - 1];
        let word2 = self.stack[len - index];

        self.stack[len - 1] = word2;
        self.stack[len - index] = word1;
    }
}