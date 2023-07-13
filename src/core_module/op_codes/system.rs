use crate::core_module::runner::Runner;
use crate::core_module::utils;
use crate::core_module::utils::bytes32::pad_to_32_bytes;
use crate::core_module::utils::environment::{get_nonce, init_account};
use crate::core_module::utils::bytes32;
use crate::core_module::utils::errors::ExecutionError;

// Primitive types
use ethers::types::U256;
use ethers::utils::keccak256;

// Colored output
use colored::*;

pub fn invalid(runner: &mut Runner) -> Result<(), ExecutionError> {
    if runner.debug.is_some() && runner.debug.unwrap() {
        println!("{:} 0x{:X}", "INVALID".red(), runner.bytecode[runner.pc]);
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn create(runner: &mut Runner) -> Result<(), ExecutionError> {
    // Get the values on the stack
    let value = unsafe { runner.stack.pop()? };
    let offset = U256::from_big_endian(&unsafe { runner.stack.pop()? });
    let size = U256::from_big_endian(&unsafe { runner.stack.pop()? });

    // Check if the address is out of bounds
    if offset.as_usize() + size.as_usize() > runner.memory.heap.len() {
        return Err(ExecutionError::OutOfBoundsMemory);
    }

    // Load the init code from memory
    let init_code = unsafe { runner.memory.read(offset.as_usize(), size.as_usize())? };

    // Compute the contract address
    let mut input = vec![0xd6, 0x94];
    input.extend_from_slice(&runner.caller);
    input.extend_from_slice(&bytes32::strip_zero_padding(&get_nonce(runner.caller, runner)?));

    let hash = keccak256(input);
    let contract_address: [u8; 20] = hash[12..].try_into().unwrap();

    // Create the contract
    init_account(contract_address, runner)?;
    runner.state.put_code_at(contract_address, init_code)?;

    // Transfer the value
    runner.state.transfer(runner.caller, contract_address, value)?;

    unsafe { runner.stack.push(pad_to_32_bytes(&contract_address))}?;

    if runner.debug.is_some() && runner.debug.unwrap() {
        let hex: String = utils::debug::to_hex_string(pad_to_32_bytes(&contract_address));
        println!("{:<14} 👉 [ {} ]", "CREATE".bright_blue(), hex);
    }

    // Increment PC
    runner.increment_pc(1)
}
