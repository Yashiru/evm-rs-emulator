use crate::core_module::utils;
use crate::core_module::utils::errors::ExecutionError;
use crate::core_module::runner::Runner;

// Colored output
use colored::*;

// Load 32 bytes from memory
pub fn sload(runner: &mut Runner) -> Result<(), ExecutionError> {
    let address = unsafe { runner.stack.pop()? };
    let word = runner.state.sload(runner.address, address)?;

    unsafe {
        let result = runner.stack
            .push(word);

        if result.is_err() {
            return Err(result.unwrap_err());
        }
    }

    if runner.debug.is_some() && runner.debug.unwrap() {
        let hex: String = utils::debug::to_hex_string(word);
        println!(
            "{:<14} 👉 [ {} ]",
            "SLOAD".bright_blue(),
            hex
        );
    }

    // Increment PC
    runner.increment_pc(1)
}

// Store 32 bytes in memory
pub fn sstore(runner: &mut Runner) -> Result<(), ExecutionError> {
    let address = unsafe { runner.stack.pop()? };
    let word = unsafe { runner.stack.pop()? };

    let result = runner.state.sstore(runner.address, address, word);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug.is_some() && runner.debug.unwrap() {
        let hex: String = utils::debug::to_hex_string(word);
        println!(
            "{:<14} ⛔️ [ {} ]",
            "SSTORE".bright_blue(),
            hex
        );
    }

    // Increment PC
    runner.increment_pc(1)
}