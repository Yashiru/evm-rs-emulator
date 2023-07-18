use crate::core_module::runner::Runner;
use crate::core_module::utils;
use crate::core_module::utils::errors::ExecutionError;

// Primitive types
use ethers::types::U256;

// Colored output
use colored::*;

/// Adds the top two elements of the stack and pushes the result onto the stack.
///
/// # Arguments
///
/// * `runner` - A mutable reference to a `Runner` instance.
///
/// # Errors
///
/// Returns an `ExecutionError` if there are not enough elements on the stack or if there is an error pushing the result onto the stack.
pub fn add(runner: &mut Runner) -> Result<(), ExecutionError> {
    let pop1 = runner.stack.pop()?;
    let pop2 = runner.stack.pop()?;

    let a = U256::from_big_endian(&pop1);
    let b = U256::from_big_endian(&pop2);

    let (result, _) = a.overflowing_add(b);

    let mut result_bytes = [0u8; 32];
    result.to_big_endian(&mut result_bytes);

    let result = runner.stack.push(result_bytes);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let hex: String = utils::debug::to_hex_string(result_bytes);
        runner.print_debug(&format!("{:<14} 👉 [ {} ]", "ADD".bright_blue(), hex));
    }

    // Increment PC
    runner.increment_pc(1)
}

/// Multiplies the top two elements of the stack and pushes the result onto the stack.
///
/// # Arguments
///
/// * `runner` - A mutable reference to a `Runner` instance.
///
/// # Errors
///
/// Returns an `ExecutionError` if there are not enough elements on the stack or if there is an error pushing the result onto the stack.
pub fn mul(runner: &mut Runner) -> Result<(), ExecutionError> {
    let pop1 = runner.stack.pop()?;
    let pop2 = runner.stack.pop()?;

    let a = U256::from_big_endian(&pop1);
    let b = U256::from_big_endian(&pop2);

    let (result, _) = a.overflowing_mul(b);

    let mut result_bytes = [0u8; 32];
    result.to_big_endian(&mut result_bytes);

    let result = runner.stack.push(result_bytes);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let hex: String = utils::debug::to_hex_string(result_bytes);
        runner.print_debug(&format!("{:<14} 👉 [ {} ]", "MUL".bright_blue(), hex));
    }

    // Increment PC
    runner.increment_pc(1)
}

/// Subtracts the top two elements of the stack and pushes the result onto the stack.
///
/// # Arguments
///
/// * `runner` - A mutable reference to a `Runner` instance.
///
/// # Errors
///
/// Returns an `ExecutionError` if there are not enough elements on the stack or if there is an error pushing the result onto the stack.
pub fn sub(runner: &mut Runner) -> Result<(), ExecutionError> {
    let pop1 = runner.stack.pop()?;
    let pop2 = runner.stack.pop()?;

    let a = U256::from_big_endian(&pop1);
    let b = U256::from_big_endian(&pop2);

    let (result, _) = a.overflowing_sub(b);

    let mut result_bytes = [0u8; 32];
    result.to_big_endian(&mut result_bytes);

    let result = runner.stack.push(result_bytes);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let hex: String = utils::debug::to_hex_string(result_bytes);
        runner.print_debug(&format!("{:<14} 👉 [ {} ]", "SUB".bright_blue(), hex));
    }

    // Increment PC
    runner.increment_pc(1)
}

/// Computes the remainder of the top two elements of the stack and pushes the result onto the stack.
///
/// # Arguments
///
/// * `runner` - A mutable reference to a `Runner` instance.
///
/// # Errors
///
/// Returns an `ExecutionError` if there are not enough elements on the stack or if there is an error pushing the result onto the stack.
pub fn modulo(runner: &mut Runner) -> Result<(), ExecutionError> {
    let pop1 = runner.stack.pop()?;
    let pop2 = runner.stack.pop()?;

    let a = U256::from_big_endian(&pop1);
    let b = U256::from_big_endian(&pop2);

    let result = a.checked_rem(b);

    let mut result_bytes = [0u8; 32];
    result
        .unwrap_or(U256::from(0))
        .to_big_endian(&mut result_bytes);

    let result = runner.stack.push(result_bytes);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let hex: String = utils::debug::to_hex_string(result_bytes);
        runner.print_debug(&format!("{:<14} 👉 [ {} ]", "MOD".bright_blue(), hex));
    }

    // Increment PC
    runner.increment_pc(1)
}

/// Divides the top two elements of the stack and pushes the result onto the stack.
///
/// # Arguments
///
/// * `runner` - A mutable reference to a `Runner` instance.
///
/// # Errors
///
/// Returns an `ExecutionError` if there are not enough elements on the stack or if there is an error pushing the result onto the stack.
pub fn div(runner: &mut Runner) -> Result<(), ExecutionError> {
    let pop1 = runner.stack.pop()?;
    let pop2 = runner.stack.pop()?;

    let a = U256::from_big_endian(&pop1);
    let b = U256::from_big_endian(&pop2);

    let result = a.checked_div(b);

    let mut result_bytes = [0u8; 32];
    result
        .unwrap_or(U256::from(0))
        .to_big_endian(&mut result_bytes);

    let result = runner.stack.push(result_bytes);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let hex: String = utils::debug::to_hex_string(result_bytes);
        runner.print_debug(&format!("{:<14} 👉 [ {} ]", "DIV".bright_blue(), hex));
    }

    // Increment PC
    runner.increment_pc(1)
}

/// Computes the addition modulo of the top three elements of the stack and pushes the result onto the stack.
///
/// # Arguments
///
/// * `runner` - A mutable reference to a `Runner` instance.
///
/// # Errors
///
/// Returns an `ExecutionError` if there are not enough elements on the stack or if there is an error pushing the result onto the stack.
pub fn addmod(runner: &mut Runner) -> Result<(), ExecutionError> {
    let pop1 = runner.stack.pop()?;
    let pop2 = runner.stack.pop()?;
    let pop3 = runner.stack.pop()?;

    let a = U256::from_big_endian(&pop1);
    let b = U256::from_big_endian(&pop2);
    let c = U256::from_big_endian(&pop3);

    let (result, _) = a.overflowing_add(b);
    let result = result.checked_rem(c);

    let mut result_bytes = [0u8; 32];
    result
        .unwrap_or(U256::from(0))
        .to_big_endian(&mut result_bytes);

    let result = runner.stack.push(result_bytes);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let hex: String = utils::debug::to_hex_string(result_bytes);
        runner.print_debug(&format!("{:<14} 👉 [ {} ]", "ADDMOD".bright_blue(), hex));
    }

    // Increment PC
    runner.increment_pc(1)
}

/// Computes the multiplication modulo of the top three elements of the stack and pushes the result onto the stack.
///
/// # Arguments
///
/// * `runner` - A mutable reference to a `Runner` instance.
///
/// # Errors
///
/// Returns an `ExecutionError` if there are not enough elements on the stack or if there is an error pushing the result onto the stack.
pub fn mulmod(runner: &mut Runner) -> Result<(), ExecutionError> {
    let pop1 = runner.stack.pop()?;
    let pop2 = runner.stack.pop()?;
    let pop3 = runner.stack.pop()?;

    let a = U256::from_big_endian(&pop1);
    let b = U256::from_big_endian(&pop2);
    let c = U256::from_big_endian(&pop3);

    let (result, _) = a.overflowing_mul(b);
    let result = result.checked_rem(c);

    let mut result_bytes = [0u8; 32];
    result
        .unwrap_or(U256::from(0))
        .to_big_endian(&mut result_bytes);

    let result = runner.stack.push(result_bytes);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let hex: String = utils::debug::to_hex_string(result_bytes);
        runner.print_debug(&format!("{:<14} 👉 [ {} ]", "MULMOD".bright_blue(), hex));
    }

    // Increment PC
    runner.increment_pc(1)
}

/// Computes the exponentiation of the top two elements of the stack and pushes the result onto the stack.
///
/// # Arguments
///
/// * `runner` - A mutable reference to a `Runner` instance.
///
/// # Errors
///
/// Returns an `ExecutionError` if there are not enough elements on the stack or if there is an error pushing the result onto the stack.
pub fn exp(runner: &mut Runner) -> Result<(), ExecutionError> {
    let pop1 = runner.stack.pop()?;
    let pop2 = runner.stack.pop()?;

    let a = U256::from_big_endian(&pop1);
    let b = U256::from_big_endian(&pop2);

    let (result, _) = a.overflowing_pow(b);

    let mut result_bytes = [0u8; 32];
    result.to_big_endian(&mut result_bytes);

    let result = runner.stack.push(result_bytes);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let hex: String = utils::debug::to_hex_string(result_bytes);
        runner.print_debug(&format!("{:<14} 👉 [ {} ]", "EXP".bright_blue(), hex));
    }

    // Increment PC
    runner.increment_pc(1)
}
