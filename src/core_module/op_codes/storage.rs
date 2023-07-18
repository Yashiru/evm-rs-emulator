use crate::core_module::runner::Runner;
use crate::core_module::utils;
use crate::core_module::utils::errors::ExecutionError;

// Colored output
use colored::*;

// Load 32 bytes from memory
pub fn sload(runner: &mut Runner) -> Result<(), ExecutionError> {
    let address = runner.stack.pop()?;
    let word = runner.state.sload(runner.address, address)?;

    let result = runner.stack.push(word);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let hex: String = utils::debug::to_hex_string(word);
        runner.print_debug(&format!("{:<14} 👉 [ {} ]", "SLOAD".bright_blue(), hex));
    }

    // Increment PC
    runner.increment_pc(1)
}

// Store 32 bytes in memory
pub fn sstore(runner: &mut Runner) -> Result<(), ExecutionError> {
    let address = runner.stack.pop()?;
    let word = runner.stack.pop()?;

    let result = runner.state.sstore(runner.address, address, word);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let hex: String = utils::debug::to_hex_string(word);
        runner.print_debug(&format!("{:<14} ⛔️ [ {} ]", "SSTORE".bright_blue(), hex));
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
    fn test_sload() {
        let mut runner = Runner::_default(3);
        let interpret_result: Result<(), ExecutionError> = runner.interpret(
            _hex_string_to_bytes("602e600055600054600154"),
            Some(2),
            true,
        );
        assert!(interpret_result.is_ok());

        let result: [u8; 32] = runner.stack.pop().unwrap();
        assert_eq!(result, pad_left(&[0x00]));
        let result: [u8; 32] = runner.stack.pop().unwrap();
        assert_eq!(result, pad_left(&[0x2e]));
    }

    #[test]
    fn test_sstore() {
        let mut runner = Runner::_default(3);
        let interpret_result: Result<(), ExecutionError> =
            runner.interpret(_hex_string_to_bytes("602e600055"), Some(2), true);
        assert!(interpret_result.is_ok());

        let result = runner
            .state
            .sload(runner.address, pad_left(&[0x00]))
            .unwrap();
        assert_eq!(result, pad_left(&[0x2e]));
    }
}
