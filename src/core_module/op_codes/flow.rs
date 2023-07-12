use crate::core_module::runner::Runner;
use crate::core_module::utils;
use crate::core_module::utils::errors::ExecutionError;
use std::time::{SystemTime, UNIX_EPOCH};

// Primitive types
use ethers::types::U256;
use ethers::utils::keccak256;

// Colored output
use colored::*;

// Stop execution
pub fn stop(runner: &mut Runner) -> Result<(), ExecutionError> {
    // Set the program counter to the end of the bytecode
    runner.set_pc(runner.bytecode.len());


    if runner.debug.is_some() && runner.debug.unwrap() {
        println!(
            "{:<14} {}",
            "STOP".bright_blue(),
            "🛑 STOP 🛑".red()
        );
    }

    Ok(())
}

// Revert data from memory heap
pub fn revert(runner: &mut Runner) -> Result<(), ExecutionError> {
    let offset = U256::from_big_endian(&unsafe { runner.stack.pop()? });
    let size = U256::from_big_endian(&unsafe { runner.stack.pop()? });

    // Check if the address is out of bounds
    if offset.as_usize() + size.as_usize() > runner.heap.heap.len() {
        return Err(ExecutionError::OutOfBoundsMemory);
    }

    let revert_data = unsafe { runner.heap.read(offset.as_usize(), size.as_usize()) };

    let err;
    let hex;

    if revert_data.is_ok() && revert_data.as_ref().unwrap().len() > 0 {
        hex = utils::debug::to_hex_string(revert_data.as_ref().unwrap().as_slice().try_into().unwrap());
        err = ExecutionError::Revert(revert_data.unwrap());
    }
    else {
        hex = utils::debug::to_hex_string([0u8; 32]);
        err = ExecutionError::RevertWithoutData;
    }

    if runner.debug.is_some() && runner.debug.unwrap() {
        println!(
            "{:<14} 💢 [ {} ]",
            "REVERT".red(),
            hex
        );
    }

    Err(err)
}

// jump
pub fn jump(runner: &mut Runner) -> Result<(), ExecutionError> {
    let jump_address = U256::from_big_endian(&unsafe { runner.stack.pop()? });

    // Check if the address is out of bounds
    if jump_address.as_usize() > runner.bytecode.len() {
        return Err(ExecutionError::OutOfBoundsByteCode);
    }

    // Set the program counter to the jump address
    runner.set_pc(jump_address.as_usize());
    Ok(())
}

// jumpi
pub fn jumpi(runner: &mut Runner) -> Result<(), ExecutionError> {
    let jump_address = U256::from_big_endian(&unsafe { runner.stack.pop()? });
    let condition = U256::from_big_endian(&unsafe { runner.stack.pop()? });

    // Check if the address is out of bounds
    if jump_address.as_usize() > runner.bytecode.len() {
        return Err(ExecutionError::OutOfBoundsByteCode);
    }

    // Check if the condition is true
    if condition != U256::zero() {
        // Set the program counter to the jump address
        runner.set_pc(jump_address.as_usize());
    }
    else{
        // Increment the program counter
        runner.increment_pc(1);
    }

    Ok(())
}

// pc
pub fn pc(runner: &mut Runner) -> Result<(), ExecutionError> {
    let mut pc = runner.get_pc().to_le_bytes();
    pc.reverse();
    let bytes = [[0u8; 24].to_vec(), pc.to_vec()].concat().as_slice().try_into().unwrap();

    // Push the program counter to the stack
    unsafe { runner.stack.push(bytes)? };

    // Increment the program counter
    runner.increment_pc(1);
    Ok(())
}

// gas
pub fn gas(runner: &mut Runner) -> Result<(), ExecutionError> {
    let mut gas = runner.gas.to_le_bytes();
    gas.reverse();
    let bytes = [[0u8; 24].to_vec(), gas.to_vec()].concat().as_slice().try_into().unwrap();

    // Push the gas to the stack
    unsafe { runner.stack.push(bytes)? };

    // Increment the program counter
    runner.increment_pc(1);
    Ok(())
}

// jumpdest
pub fn jumpdest(runner: &mut Runner) -> Result<(), ExecutionError> {
    // Increment the program counter
    runner.increment_pc(1);
    Ok(())
}