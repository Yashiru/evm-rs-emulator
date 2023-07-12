use crate::core_module::utils;
use crate::core_module::utils::errors::ExecutionError;
use crate::core_module::runner::Runner;

// Primitive types
use ethers::types::U256;

// Colored output
use colored::*;

// Load 32 bytes from memory
pub fn mload(runner: &mut Runner) -> Result<(), ExecutionError> {

    let address = U256::from_big_endian(&unsafe { runner.stack.pop()? });
    let word = unsafe { runner.heap.mload(address.low_u32() as usize)? };
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
            "MLOAD".bright_blue(),
            hex
        );
    }

    // Increment PC
    runner.increment_pc(1)
}

// Store 32 bytes in memory
pub fn mstore(runner: &mut Runner) -> Result<(), ExecutionError> {

    let address = U256::from_big_endian(&unsafe { runner.stack.pop()? });
    let data = unsafe { runner.stack.pop()? };

    let result = unsafe { runner.heap.mstore(address.low_u32() as usize, data) };

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug.is_some() && runner.debug.unwrap() {
        let hex: String = utils::debug::to_hex_string(data);
        println!(
            "{:<14} ⛔️ [ {} ]",
            "MSTORE".bright_blue(),
            hex
        );
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn msize(runner: &mut Runner) -> Result<(), ExecutionError> {
    let mut bytes_msize = [0u8; 32];
    U256::from(runner.heap.msize() as u64).to_big_endian(&mut bytes_msize);

    let result = unsafe { runner.stack.push(bytes_msize) };

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug.is_some() && runner.debug.unwrap() {
        let hex: String = utils::debug::to_hex_string(bytes_msize);
        println!(
            "{:<14} 👉 [ {} ]",
            "MSIZE".bright_blue(),
            hex
        );
    }

    // Increment PC
    runner.increment_pc(1)
}
