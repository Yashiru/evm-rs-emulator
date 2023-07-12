use crate::core_module::runner::Runner;
use crate::core_module::utils;
use crate::core_module::utils::errors::ExecutionError;

// Primitive types
use ethers::types::{U256, I256};

// Colored output
use colored::*;

pub fn iszero(runner: &mut Runner) -> Result<(), ExecutionError> {
    unsafe {
        let pop1 = runner.stack.pop()?;

        let a = U256::from_big_endian(&pop1);

        let bool = a.is_zero();

        let result_bytes = [
            [0u8; 31].to_vec(),
            [if bool { 1u8 } else { 0u8 }; 1].to_vec(),
        ]
        .concat()
        .as_slice()
        .try_into()
        .unwrap();

        let result = runner.stack.push(result_bytes);

        if result.is_err() {
            return Err(result.unwrap_err());
        }

        if runner.debug.is_some() && runner.debug.unwrap() {
            let hex: String = utils::debug::to_hex_string(result_bytes);
            println!("{:<14} 👉 [ {} ]", "ISZERO".bright_blue(), hex);
        }
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn eq(runner: &mut Runner) -> Result<(), ExecutionError> {
    unsafe {
        let pop1 = runner.stack.pop()?;
        let pop2 = runner.stack.pop()?;

        let a = U256::from_big_endian(&pop1);
        let b = U256::from_big_endian(&pop2);

        let bool = a.eq(&b);

        let result_bytes = [
            [0u8; 31].to_vec(),
            [if bool { 1u8 } else { 0u8 }; 1].to_vec(),
        ]
        .concat()
        .as_slice()
        .try_into()
        .unwrap();

        let result = runner.stack.push(result_bytes);

        if result.is_err() {
            return Err(result.unwrap_err());
        }

        if runner.debug.is_some() && runner.debug.unwrap() {
            let hex: String = utils::debug::to_hex_string(result_bytes);
            println!("{:<14} 👉 [ {} ]", "EQ".bright_blue(), hex);
        }
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn lt(runner: &mut Runner) -> Result<(), ExecutionError> {
    unsafe {
        let pop1 = runner.stack.pop()?;
        let pop2 = runner.stack.pop()?;

        let a = U256::from_big_endian(&pop1);
        let b = U256::from_big_endian(&pop2);

        let bool = a.lt(&b);

        let result_bytes = [
            [0u8; 31].to_vec(),
            [if bool { 1u8 } else { 0u8 }; 1].to_vec(),
        ]
        .concat()
        .as_slice()
        .try_into()
        .expect("Wrong length");

        let result = runner.stack.push(result_bytes);

        if result.is_err() {
            return Err(result.unwrap_err());
        }

        if runner.debug.is_some() && runner.debug.unwrap() {
            let hex: String = utils::debug::to_hex_string(result_bytes);
            println!("{:<14} 👉 [ {} ]", "LT".bright_blue(), hex);
        }
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn gt(runner: &mut Runner) -> Result<(), ExecutionError> {
    unsafe {
        let pop1 = runner.stack.pop()?;
        let pop2 = runner.stack.pop()?;

        let a = U256::from_big_endian(&pop1);
        let b = U256::from_big_endian(&pop2);

        let bool = a.gt(&b);

        let result_bytes = [
            [0u8; 31].to_vec(),
            [if bool { 1u8 } else { 0u8 }; 1].to_vec(),
        ]
        .concat()
        .as_slice()
        .try_into()
        .unwrap();

        let result = runner.stack.push(result_bytes);

        if result.is_err() {
            return Err(result.unwrap_err());
        }

        if runner.debug.is_some() && runner.debug.unwrap() {
            let hex: String = utils::debug::to_hex_string(result_bytes);
            println!("{:<14} 👉 [ {} ]", "GT".bright_blue(), hex);
        }
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn slt(runner: &mut Runner) -> Result<(), ExecutionError> {
    unsafe {
        let pop1 = runner.stack.pop()?;
        let pop2 = runner.stack.pop()?;

        let a = I256::from_raw(U256::from_big_endian(&pop1));
        let b = I256::from_raw(U256::from_big_endian(&pop2));

        let bool = a.lt(&b);

        let result_bytes = [
            [0u8; 31].to_vec(),
            [if bool { 1u8 } else { 0u8 }; 1].to_vec(),
        ]
        .concat()
        .as_slice()
        .try_into()
        .unwrap();

        let result = runner.stack.push(result_bytes);

        if result.is_err() {
            return Err(result.unwrap_err());
        }

        if runner.debug.is_some() && runner.debug.unwrap() {
            let hex: String = utils::debug::to_hex_string(result_bytes);
            println!("{:<14} 👉 [ {} ]", "SLT".bright_blue(), hex);
        }
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn sgt(runner: &mut Runner) -> Result<(), ExecutionError> {
    unsafe {
        let pop1 = runner.stack.pop()?;
        let pop2 = runner.stack.pop()?;

        let a = I256::from_raw(U256::from_big_endian(&pop1));
        let b = I256::from_raw(U256::from_big_endian(&pop2));

        let bool = a.gt(&b);

        let result_bytes = [
            [0u8; 31].to_vec(),
            [if bool { 1u8 } else { 0u8 }; 1].to_vec(),
        ]
        .concat()
        .as_slice()
        .try_into()
        .unwrap();

        let result = runner.stack.push(result_bytes);

        if result.is_err() {
            return Err(result.unwrap_err());
        }

        if runner.debug.is_some() && runner.debug.unwrap() {
            let hex: String = utils::debug::to_hex_string(result_bytes);
            println!("{:<14} 👉 [ {} ]", "SGT".bright_blue(), hex);
        }
    }

    // Increment PC
    runner.increment_pc(1)
}
