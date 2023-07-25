use crate::core_module::runner::Runner;
use crate::core_module::utils;
use crate::core_module::utils::errors::ExecutionError;

// Primitive types
use ethers::types::U256;

// Colored output
use colored::*;

/// Loads a 32-byte word from memory and pushes it onto the stack.
///
/// # Arguments
///
/// * `runner` - A mutable reference to the `Runner` struct.
///
/// # Errors
///
/// Returns an `ExecutionError` if the stack is empty or if there is an error pushing the result onto the stack.
///
/// # Safety
///
/// This function is marked as `unsafe` because it accesses memory directly.
pub fn mload(runner: &mut Runner) -> Result<(), ExecutionError> {
    let address = U256::from_big_endian(&runner.stack.pop()?);
    let word = unsafe { runner.memory.mload(address.as_usize())? };
    let result = runner.stack.push(word);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let hex: String = utils::debug::to_hex_string(word);
        runner.print_debug(&format!("{:<14} 👉 [ {} ]", "MLOAD".bright_blue(), hex));
    }

    // Increment PC
    runner.increment_pc(1)
}

/// Stores a 32-byte word at the specified memory address.
/// It takes two items from the stack: the first item is the memory address and the second item is the data to store.
///
/// # Arguments
///
/// * `runner` - A mutable reference to the `Runner` struct.
///
/// # Errors
///
/// Returns an `ExecutionError` if the memory address is out of bounds.
pub fn mstore(runner: &mut Runner) -> Result<(), ExecutionError> {
    let address = U256::from_big_endian(&runner.stack.pop()?);
    let data = runner.stack.pop()?;

    let result = unsafe { runner.memory.mstore(address.as_usize(), data) };

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let hex: String = utils::debug::to_hex_string(data);
        runner.print_debug(&format!("{:<14} ⛔️ [ {} ]", "MSTORE".bright_blue(), hex));
    }

    // Increment PC
    runner.increment_pc(1)
}

/// Pushes the size of the memory onto the stack.
///
/// # Arguments
///
/// * `runner` - A mutable reference to the `Runner` struct.
///
/// # Errors
///
/// Returns an `ExecutionError` if there is an error pushing the result onto the stack.
pub fn msize(runner: &mut Runner) -> Result<(), ExecutionError> {
    let mut bytes_msize = [0u8; 32];
    U256::from(runner.memory.msize() as u64).to_big_endian(&mut bytes_msize);

    let result = runner.stack.push(bytes_msize);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let hex: String = utils::debug::to_hex_string(bytes_msize);
        runner.print_debug(&format!("{:<14} 👉 [ {} ]", "MSIZE".bright_blue(), hex));
    }

    // Increment PC
    runner.increment_pc(1)
}

#[cfg(test)]
mod tests {
    use crate::core_module::runner::Runner;
    use crate::core_module::utils::bytes::{_hex_string_to_bytes, pad_left};
    use crate::core_module::utils::errors::ExecutionError;

    #[test]
    fn test_mload() {
        let mut runner = Runner::_default(3);
        let interpret_result: Result<(), ExecutionError> = runner.interpret(_hex_string_to_bytes("7f00000000000000000000000000000000000000000000000000000000000000ff600052600051600151"), Some(2), true);
        assert!(interpret_result.is_ok());

        let result: [u8; 32] = runner.stack.pop().unwrap();
        assert_eq!(result, pad_left(&[0xff, 0x00]));
    }

    #[test]
    fn test_mstore() {
        let mut runner = Runner::_default(3);
        let interpret_result: Result<(), ExecutionError> = runner.interpret(
            _hex_string_to_bytes(
                "7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff600052",
            ),
            Some(2),
            true,
        );
        assert!(interpret_result.is_ok());

        let memory_content = unsafe { runner.memory.read(0x00, 0x20) };
        assert!(memory_content.is_ok());

        assert_eq!(memory_content.unwrap(), [0xff; 32]);
    }

    #[test]
    fn test_msize() {
        let mut runner = Runner::_default(3);
        let interpret_result: Result<(), ExecutionError> = runner.interpret(
            _hex_string_to_bytes("5960005150596039515059"),
            Some(2),
            true,
        );
        assert!(interpret_result.is_ok());

        let result1: [u8; 32] = runner.stack.pop().unwrap();
        let result2: [u8; 32] = runner.stack.pop().unwrap();
        let result3: [u8; 32] = runner.stack.pop().unwrap();

        assert_eq!(result1, pad_left(&[0x60]));
        assert_eq!(result2, pad_left(&[0x20]));
        assert_eq!(result3, pad_left(&[0x00]));
    }
}
