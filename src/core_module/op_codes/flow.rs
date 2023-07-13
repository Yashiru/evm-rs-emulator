use crate::core_module::runner::Runner;
use crate::core_module::utils;
use crate::core_module::utils::errors::ExecutionError;

// Primitive types
use ethers::types::U256;

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
    if offset.as_usize() + size.as_usize() > runner.memory.heap.len() {
        return Err(ExecutionError::OutOfBoundsMemory);
    }

    let revert_data = unsafe { runner.memory.read(offset.as_usize(), size.as_usize()) };

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
            "\n{:<14} 💥 [ {} ]",
            "REVERT".red(),
            hex
        );
    }

    Err(err)
}

// jump
pub fn jump(runner: &mut Runner) -> Result<(), ExecutionError> {
    let mut bytes = [0u8; 32];
    let jump_address = U256::from_big_endian(&unsafe { runner.stack.pop()? });
    jump_address.to_big_endian(&mut bytes);

    // Check if the address is out of bounds
    if jump_address.as_usize() > runner.bytecode.len() {
        return Err(ExecutionError::OutOfBoundsByteCode);
    }

    if runner.debug.is_some() && runner.debug.unwrap() {
        let hex = utils::debug::to_hex_string(bytes);

        println!(
            "{:<14} 〰️ [ {} ]",
            "JUMP".bright_green(),
            hex
        );
    }

    // Set the program counter to the jump address
    runner.set_pc(jump_address.as_usize());
    Ok(())
}

// jumpi
pub fn jumpi(runner: &mut Runner) -> Result<(), ExecutionError> {
    let mut bytes = [0u8; 32];
    let jump_address = U256::from_big_endian(&unsafe { runner.stack.pop()? });
    jump_address.to_big_endian(&mut bytes);

    let condition = U256::from_big_endian(&unsafe { runner.stack.pop()? });

    // Check if the address is out of bounds
    if jump_address.as_usize() > runner.bytecode.len() {
        return Err(ExecutionError::OutOfBoundsByteCode);
    }

    // Check if the condition is true
    if !condition.is_zero() {
        // Set the program counter to the jump address
        runner.set_pc(jump_address.as_usize());
    }
    else{
        // Increment the program counter
        let _ = runner.increment_pc(1);
    }

    if runner.debug.is_some() && runner.debug.unwrap() {
        let hex = utils::debug::to_hex_string(bytes);

        println!(
            "{:<14} 〰️ [ {} ]",
            if condition.is_zero() { "JUMPI".green() } else { "JUMPI".bright_red() },
            hex
        );
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
    runner.increment_pc(1)
}

// gas
pub fn gas(runner: &mut Runner) -> Result<(), ExecutionError> {
    let mut gas = runner.gas.to_le_bytes();
    gas.reverse();
    let bytes = [[0u8; 24].to_vec(), gas.to_vec()].concat().as_slice().try_into().unwrap();

    // Push the gas to the stack
    unsafe { runner.stack.push(bytes)? };

    // Increment the program counter
    runner.increment_pc(1)
}

// jumpdest
pub fn jumpdest(runner: &mut Runner) -> Result<(), ExecutionError> {
    // Increment the program counter
    runner.increment_pc(1)
}