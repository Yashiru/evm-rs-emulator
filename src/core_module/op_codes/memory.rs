use crate::core_module::utils::errors::ExecutionError;
use crate::core_module::runner::Runner;

// Primitive types
use ethers::types::I256;
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

    // Increment PC
    runner.increment_pc(1)
}

// Store 32 bytes in memory
pub fn mstore(runner: &mut Runner) -> Result<(), ExecutionError> {
    // Increment PC
    runner.increment_pc(1);

    let address = U256::from_big_endian(&unsafe { runner.stack.pop()? });
    let data = unsafe { runner.stack.pop()? };

    unsafe { runner.heap.mstore(address.low_u32() as usize, data) }
}

pub fn msize(runner: &mut Runner) -> Result<(), ExecutionError> {
    let mut bytes_msize = [0u8; 32];
    U256::from(runner.heap.msize() as u64).to_big_endian(&mut bytes_msize);

    unsafe { runner.stack.push(bytes_msize) }
}
