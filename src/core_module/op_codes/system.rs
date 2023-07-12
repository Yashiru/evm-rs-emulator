use crate::core_module::utils;
use crate::core_module::utils::errors::ExecutionError;
use crate::core_module::runner::Runner;

// Primitive types
use ethers::types::U256;
use ethers::utils::keccak256;

// Colored output
use colored::*;

// Load 32 bytes from memory
pub fn create(runner: &mut Runner) -> Result<(), ExecutionError> {
    let value = U256::from_big_endian(&unsafe { runner.stack.pop()? });
    let offset = U256::from_big_endian(&unsafe { runner.stack.pop()? });
    let size = U256::from_big_endian(&unsafe { runner.stack.pop()? });

    // Check if the address is out of bounds
    if offset.as_usize() + size.as_usize() > runner.heap.heap.len() {
        return Err(ExecutionError::OutOfBoundsMemory);
    }

    let init_code = unsafe { runner.heap.read(offset.as_usize(), size.as_usize())? };

    let contract_address = keccak256([
        
    ]);

    // if runner.debug.is_some() && runner.debug.unwrap() {
    //     let hex: String = utils::debug::to_hex_string(word);
    //     println!(
    //         "{:<14} 👉 [ {} ]",
    //         "SLOAD".bright_blue(),
    //         hex
    //     );
    // }

    // Increment PC
    runner.increment_pc(1)
}
