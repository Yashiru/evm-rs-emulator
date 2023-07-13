use crate::core_module::runner::Runner;
use crate::core_module::utils;
use crate::core_module::utils::errors::ExecutionError;

// Primitive types
use ethers::types::U256;
use ethers::utils::keccak256;

// Colored output
use colored::*;

pub fn not(runner: &mut Runner) -> Result<(), ExecutionError> {
    unsafe {
        let pop1 = runner.stack.pop()?;

        let a = U256::from_big_endian(&pop1);

        let result = !a;

        let mut result_bytes = [0u8; 32];
        result.to_big_endian(&mut result_bytes);

        let result = runner.stack.push(result_bytes);
        
        if result.is_err() {
            return Err(result.unwrap_err());
        }

        if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
            let hex: String = utils::debug::to_hex_string(result_bytes);
            println!("{:<14} 👉 [ {} ]", "NOT".bright_blue(), hex);
        }
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn xor(runner: &mut Runner) -> Result<(), ExecutionError> {
    unsafe {
        let pop1 = runner.stack.pop()?;
        let pop2 = runner.stack.pop()?;

        let a = U256::from_big_endian(&pop1);
        let b = U256::from_big_endian(&pop2);

        let result = a ^ b;

        let mut result_bytes = [0u8; 32];
        result.to_big_endian(&mut result_bytes);

        let result = runner.stack.push(result_bytes);
        
        if result.is_err() {
            return Err(result.unwrap_err());
        }

        if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
            let hex: String = utils::debug::to_hex_string(result_bytes);
            println!("{:<14} 👉 [ {} ]", "XOR".bright_blue(), hex);
        }
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn or(runner: &mut Runner) -> Result<(), ExecutionError> {
    unsafe {
        let pop1 = runner.stack.pop()?;
        let pop2 = runner.stack.pop()?;

        let a = U256::from_big_endian(&pop1);
        let b = U256::from_big_endian(&pop2);

        let result = a | b;

        let mut result_bytes = [0u8; 32];
        result.to_big_endian(&mut result_bytes);

        let result = runner.stack.push(result_bytes);

        if result.is_err() {
            return Err(result.unwrap_err());
        }

        if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
            let hex: String = utils::debug::to_hex_string(result_bytes);
            println!("{:<14} 👉 [ {} ]", "OR".bright_blue(), hex);
        }
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn and(runner: &mut Runner) -> Result<(), ExecutionError> {
    unsafe {
        let pop1 = runner.stack.pop()?;
        let pop2 = runner.stack.pop()?;

        let a = U256::from_big_endian(&pop1);
        let b = U256::from_big_endian(&pop2);

        let result = a & b;

        let mut result_bytes = [0u8; 32];
        result.to_big_endian(&mut result_bytes);

        let result = runner.stack.push(result_bytes);

        if result.is_err() {
            return Err(result.unwrap_err());
        }

        if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
            let hex: String = utils::debug::to_hex_string(result_bytes);
            println!("{:<14} 👉 [ {} ]", "AND".bright_blue(), hex);
        }
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn shl(runner: &mut Runner) -> Result<(), ExecutionError> {
    unsafe {
        let pop1 = runner.stack.pop()?;
        let pop2 = runner.stack.pop()?;

        let a = U256::from_big_endian(&pop1);
        let b = U256::from_big_endian(&pop2);

        let result = b << a;

        let mut result_bytes = [0u8; 32];
        result.to_big_endian(&mut result_bytes);

        let result = runner.stack.push(result_bytes);

        if result.is_err() {
            return Err(result.unwrap_err());
        }

        if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
            let hex: String = utils::debug::to_hex_string(result_bytes);
            println!("{:<14} 👉 [ {} ]", "SHL".bright_blue(), hex);
        }
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn shr(runner: &mut Runner) -> Result<(), ExecutionError> {
    unsafe {
        let pop1 = runner.stack.pop()?;
        let pop2 = runner.stack.pop()?;

        let a = U256::from_big_endian(&pop1);
        let b = U256::from_big_endian(&pop2);

        let result = b >> a;

        let mut result_bytes = [0u8; 32];
        result.to_big_endian(&mut result_bytes);

        let result = runner.stack.push(result_bytes);

        if result.is_err() {
            return Err(result.unwrap_err());
        }

        if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
            let hex: String = utils::debug::to_hex_string(result_bytes);
            println!("{:<14} 👉 [ {} ]", "SHR".bright_blue(), hex);
        }
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn sha(runner: &mut Runner) -> Result<(), ExecutionError> {
    unsafe {
        let pop1 = runner.stack.pop()?;
        let pop2 = runner.stack.pop()?;

        let offset = U256::from_big_endian(&pop1).as_usize();
        let size = U256::from_big_endian(&pop2).as_usize();

        let data_to_hash = runner.memory.read(offset, size);

        if data_to_hash.is_err() {
            return Err(data_to_hash.unwrap_err());
        }

        let bytes = keccak256(&data_to_hash?);

        runner.stack.push(bytes)?;

        if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
            let hex: String = utils::debug::to_hex_string(bytes);
            println!("{:<14} 👉 [ {} ]", "SHA".bright_blue(), hex);
        }
    }

    // Increment PC
    runner.increment_pc(1)
}