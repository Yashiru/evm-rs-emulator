use std::fmt;

#[derive(Debug)]
pub enum InterpreterError {
    OutOfBounds,
    EmptyByteCode,
    InvalidOpcode(u8),
}

impl fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InterpreterError::OutOfBounds => 
                write!(f, "Attempted to access out of bounds memory"),
            InterpreterError::EmptyByteCode => 
                write!(f, "Attempted to interpret empty bytecode"),
            InterpreterError::InvalidOpcode(value) => 
                write!(f, "Invalid opcode: {}", value),
        }
    }
}

impl std::error::Error for InterpreterError {}