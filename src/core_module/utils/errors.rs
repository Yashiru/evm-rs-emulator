use std::fmt;

#[derive(Debug)]
pub enum ExecutionError {
    OutOfBoundsMemory,
    OutOfBoundsByteCode,
    EmptyByteCode,
    StackTooSmall,
    StackTooDeep,
    InvalidOpcode(u8),
}

impl fmt::Display for ExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ExecutionError::OutOfBoundsMemory => 
                write!(f, "Attempted to access out of bounds memory bytes"),
            ExecutionError::OutOfBoundsByteCode => 
                write!(f, "Attempted to access out of bounds bytecode bytes"),
            ExecutionError::EmptyByteCode => 
                write!(f, "Attempted to interpret empty bytecode"),
            ExecutionError::StackTooSmall => 
                write!(f, "Attempted to read out of stacks bounds"),
            ExecutionError::StackTooDeep => 
                write!(f, "Stack too deep. Maximum stack size is 1024 words"),
            ExecutionError::InvalidOpcode(value) => 
                write!(f, "Invalid opcode: {}", value),
        }
    }
}

impl std::error::Error for ExecutionError {}