use crate::core_module::runner::Runner;
use crate::core_module::utils;
use crate::core_module::utils::errors::ExecutionError;

// Colored output
use colored::*;

/// Removes the top item from the stack and returns it.
///
/// # Arguments
///
/// * `runner` - A mutable reference to the `Runner` struct.
///
/// # Errors
///
/// Returns an `ExecutionError` if the stack is empty.
pub fn pop(runner: &mut Runner) -> Result<(), ExecutionError> {
    let result = runner.stack.pop();

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let hex: String = utils::debug::to_hex_string(result.unwrap());
        runner.print_debug(&format!("{:<14} ⛔️ [ {} ]", "POP".bright_blue(), hex));
    }

    // Increment PC
    runner.increment_pc(1)
}
