use crate::core_module::runner::Runner;
use crate::core_module::utils;
use crate::core_module::utils::errors::ExecutionError;

// Colored output
use colored::*;

/// Pushes a value onto the stack.
///
/// # Arguments
///
/// * `runner` - A mutable reference to the `Runner` struct.
/// * `data_len` - The length of the data to be pushed onto the stack.
///
/// # Errors
///
/// Returns an `ExecutionError` if the data length is out of bounds.
pub fn push(runner: &mut Runner, data_len: usize) -> Result<(), ExecutionError> {
    // Check if the data length is out of bounds
    if runner.pc + 1 + data_len > runner.bytecode.len() {
        return Err(ExecutionError::OutOfBoundsByteCode);
    }

    let data = &runner.bytecode[runner.pc + 1..runner.pc + 1 + data_len];

    let mut padded = [0u8; 32]; // Start with an array of zeroes
    let start = 32 - data.len(); // Calculate where to start copying
    padded[start..].copy_from_slice(data); // Copy the slice into the end of the array

    let result = runner.stack.push(padded);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let hex: String = utils::debug::to_hex_string(padded);
        runner.print_debug(&format!(
            "{}{:<10} 👉 [ {} ]",
            "PUSH".bright_blue(),
            data_len.to_string().magenta(),
            hex
        ));
    }

    // Decrement gas
    runner.decrement_gas(if data_len == 0 { 2 } else { 3 });

    // Increment PC
    runner.increment_pc(1 + data_len)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push() {
        let mut runner = Runner::new([0xaa; 20], None, None, None, None, None);
        let _ = runner.interpret(vec![0x60, 0xff], Some(2), true);

        assert_eq!(runner.stack.stack.len(), 1);
        assert_eq!(
            runner.stack.pop().unwrap(),
            [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 255
            ]
        );
    }
}
