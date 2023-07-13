use std::fmt;

#[derive(Debug)]
pub enum ExecutionError {
    // Memory errors
    OutOfBoundsMemory,
    OutOfBoundsByteCode,

    // Account errors
    AccountNotFound,
    CodeNotFound,
    EmptyByteCode,
    InsufficientBalance,

    // Flow errors
    NotEmptyStack,
    ContractCreationFailed,

    // Stack errors
    StackTooSmall,
    StackTooDeep,

    // General execution errors
    InvalidFile,
    Revert(Vec<u8>),
    RevertWithoutData,
    NotImplemented(u8)
}

impl fmt::Display for ExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ExecutionError::OutOfBoundsMemory => {
                write!(f, "Attempted to access out of bounds memory bytes")
            }
            ExecutionError::OutOfBoundsByteCode => {
                write!(f, "Attempted to access out of bounds bytecode bytes")
            }
            ExecutionError::EmptyByteCode => write!(f, "Attempted to interpret empty bytecode"),
            ExecutionError::StackTooSmall => write!(f, "Attempted to read out of stacks bounds"),
            ExecutionError::StackTooDeep => {
                write!(f, "Stack too deep. Maximum stack size is 1024 words")
            }
            ExecutionError::InvalidFile => write!(f, "Invalid file"),
            ExecutionError::AccountNotFound => write!(f, "Trying to access non-existent account state"),
            ExecutionError::CodeNotFound => write!(f, "Trying to access non-existent account code"),
            ExecutionError::RevertWithoutData => write!(f, "Execution revert without data"),
            ExecutionError::InsufficientBalance => write!(f, "Insufficient balance to transfer"),
            ExecutionError::NotEmptyStack => write!(f, "Stack is not empty after the call"),
            ExecutionError::ContractCreationFailed => write!(f, "Contract creation failed, the constructor reverted"),
            ExecutionError::NotImplemented(op_code) => write!(f, "Op code 0x{:X} not implemented", op_code),
            ExecutionError::Revert(data) => {
                let hex = super::debug::vec_to_hex_string(data.to_owned());
                write!(f, "Execution revert with data: {}", hex)
            }
        }
    }
}

impl std::error::Error for ExecutionError {}
