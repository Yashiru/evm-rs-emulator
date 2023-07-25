use crate::core_module::runner::Runner;
use crate::core_module::utils;
use crate::core_module::utils::errors::ExecutionError;

// Primitive types
use ethers::types::I256;
use ethers::types::U256;

// Colored output
use colored::*;

/// Divide the second item in the stack by the first item in the stack.
/// Support signed integers.
///
/// # Arguments
///
/// * `runner` - A mutable reference to the `Runner`
///
/// # Errors
///
/// returns an `ExecutionError` if:
///
/// * Their is not enough items on the stack
pub fn sdiv(runner: &mut Runner) -> Result<(), ExecutionError> {
    let pop1 = runner.stack.pop()?;
    let pop2 = runner.stack.pop()?;

    let a = I256::from_raw(U256::from_big_endian(&pop1));
    let b = I256::from_raw(U256::from_big_endian(&pop2));

    let result = a.checked_div(b);

    let mut result_bytes = [0u8; 32];
    result
        .unwrap_or(I256::from(0))
        .to_big_endian(&mut result_bytes);

    let result = runner.stack.push(result_bytes);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let hex: String = utils::debug::to_hex_string(result_bytes);
        runner.print_debug(&format!("{:<14} 👉 [ {} ]", "SDIV".bright_blue(), hex));
    }

    // Increment PC
    runner.increment_pc(1)
}

/// Push the remainder of the second item in the stack divided by the first item in the stack.
/// Support signed integers.
///
/// # Arguments
///
/// * `runner` - A mutable reference to the `Runner`
///
/// # Errors
///
/// returns an `ExecutionError` if:
///
/// * Their is not enough items on the stack
pub fn smodulo(runner: &mut Runner) -> Result<(), ExecutionError> {
    let pop1 = runner.stack.pop()?;
    let pop2 = runner.stack.pop()?;

    let a = I256::from_raw(U256::from_big_endian(&pop1));
    let b = I256::from_raw(U256::from_big_endian(&pop2));

    let result = a.checked_rem(b);

    let mut result_bytes = [0u8; 32];
    result
        .unwrap_or(I256::from(0))
        .to_big_endian(&mut result_bytes);

    let result = runner.stack.push(result_bytes);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let hex: String = utils::debug::to_hex_string(result_bytes);
        runner.print_debug(&format!("{:<14} 👉 [ {} ]", "SMOD".bright_blue(), hex));
    }

    // Increment PC
    runner.increment_pc(1)
}
