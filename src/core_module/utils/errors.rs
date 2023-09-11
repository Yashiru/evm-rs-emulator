use std::fmt;

#[derive(Debug)]
pub enum ExecutionError {
    // Memory errors
    OutOfBoundsByteCode,

    // Account errors
    AccountNotFound,
    CodeNotFound,
    EmptyByteCode,
    InsufficientBalance,

    // Flow errors
    StaticCallStateChanged,
    InvalidOpcode(u8),
    InvalidJumpDestination,

    // Stack errors
    StackTooSmall,
    StackTooDeep,

    // General execution errors
    Revert(Vec<u8>),
    RevertWithoutData,
    NotImplemented(u8),
}

impl fmt::Display for ExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ExecutionError::OutOfBoundsByteCode => {
                write!(f, "Attempted to access out of bounds bytecode bytes")
            }
            ExecutionError::EmptyByteCode => write!(f, "Attempted to interpret empty bytecode"),
            ExecutionError::StackTooSmall => write!(f, "Attempted to read out of stacks bounds"),
            ExecutionError::StackTooDeep => {
                write!(f, "Stack too deep. Maximum stack size is 1024 words")
            }
            ExecutionError::AccountNotFound => {
                write!(f, "Trying to access non-existent account state")
            }
            ExecutionError::CodeNotFound => write!(f, "Trying to access non-existent account code"),
            ExecutionError::RevertWithoutData => write!(f, "Execution revert without data"),
            ExecutionError::InsufficientBalance => write!(f, "Insufficient balance to transfer"),
            ExecutionError::InvalidOpcode(op_code) => {
                write!(f, "Invalid op code 0x{:X}", op_code)
            }
            ExecutionError::StaticCallStateChanged => {
                write!(f, "State changed during a static call")
            }
            ExecutionError::NotImplemented(op_code) => {
                write!(f, "Op code 0x{:X} not implemented", op_code)
            }
            ExecutionError::InvalidJumpDestination => write!(f, "Invalid jump destination"),
            ExecutionError::Revert(data) => {
                let hex = super::debug::vec_to_hex_string(data.to_owned());
                write!(f, "Execution revert with data: {}", hex)
            }
        }
    }
}

impl std::error::Error for ExecutionError {}

impl PartialEq for ExecutionError {
    fn eq(&self, other: &Self) -> bool {
        use ExecutionError::*;
        match (self, other) {
            (OutOfBoundsByteCode, OutOfBoundsByteCode)
            | (AccountNotFound, AccountNotFound)
            | (CodeNotFound, CodeNotFound)
            | (EmptyByteCode, EmptyByteCode)
            | (InsufficientBalance, InsufficientBalance)
            | (StaticCallStateChanged, StaticCallStateChanged)
            | (StackTooSmall, StackTooSmall)
            | (InvalidJumpDestination, InvalidJumpDestination)
            | (StackTooDeep, StackTooDeep)
            | (RevertWithoutData, RevertWithoutData) => true,
            (InvalidOpcode(a), InvalidOpcode(b)) => a == b,
            (NotImplemented(a), NotImplemented(b)) => a == b,
            (Revert(a), Revert(b)) => a == b,
            _ => false,
        }
    }
}
