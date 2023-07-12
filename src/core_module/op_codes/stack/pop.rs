use crate::core_module::utils::errors::ExecutionError;
use crate::core_module::runner::Runner;
use crate::core_module::utils;

// Colored output
use colored::*;

pub fn pop(runner: &mut Runner) -> Result<(), ExecutionError> {
    let result = unsafe { runner.stack.pop() };

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug.is_some() && runner.debug.unwrap() {
        let hex: String = utils::debug::to_hex_string(result.unwrap());
        println!("{:<14} ⛔️ [ {} ]", "POP".bright_blue(), hex);
    }

    // Increment PC
    runner.increment_pc(1)
}