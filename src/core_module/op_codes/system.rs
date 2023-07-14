use crate::core_module::runner::Runner;
use crate::core_module::utils;
use crate::core_module::utils::bytes32;
use crate::core_module::utils::bytes32::{bytes32_to_address, pad_to_32_bytes};
use crate::core_module::utils::environment::{get_nonce, init_account};
use crate::core_module::utils::errors::ExecutionError;

// Primitive types
use ethers::types::U256;
use ethers::utils::keccak256;

// Colored output
use colored::*;

pub fn invalid(runner: &mut Runner) -> Result<(), ExecutionError> {
    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
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
    input.extend_from_slice(&bytes32::strip_zero_padding(&get_nonce(
        runner.caller,
        runner,
    )?));

    let hash = keccak256(input);
    let contract_address: [u8; 20] = hash[12..].try_into().unwrap();

    // Create the contract with init code as code
    init_account(contract_address, runner)?;
    runner.state.put_code_at(contract_address, init_code)?;

    // Call the contract to run its constructor
    let temp_debug_level = runner.debug_level;
    runner.debug_level = Some(0);
    let call_result = runner.call(contract_address, value, Vec::new(), runner.gas, false);
    runner.debug_level = temp_debug_level;

    // Check if the call failed
    if call_result.is_err() {
        unsafe { runner.stack.push(pad_to_32_bytes(&[0x00]))? };
    } else {
        unsafe { runner.stack.push(pad_to_32_bytes(&[0x01]))? };
    }

    // Get the return data to store the real contract code
    let returndata = runner.returndata.heap.clone();
    runner.state.put_code_at(contract_address, returndata)?;

    // Transfer the value
    runner
        .state
        .transfer(runner.caller, contract_address, value)?;

    unsafe { runner.stack.push(pad_to_32_bytes(&contract_address)) }?;

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let hex: String = utils::debug::to_hex_string(pad_to_32_bytes(&contract_address));
        println!("{:<14} 👉 [ {} ]", "CREATE".bright_blue(), hex);
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn create2(runner: &mut Runner) -> Result<(), ExecutionError> {
    // Get the values on the stack
    let value = unsafe { runner.stack.pop()? };
    let offset = U256::from_big_endian(&unsafe { runner.stack.pop()? });
    let size = U256::from_big_endian(&unsafe { runner.stack.pop()? });
    let salt = unsafe { runner.stack.pop()? };

    // Check if the offset is out of bounds
    if offset.as_usize() + size.as_usize() > runner.memory.heap.len() {
        return Err(ExecutionError::OutOfBoundsMemory);
    }
 
    // Load the init code from memory
    let init_code = unsafe { runner.memory.read(offset.as_usize(), size.as_usize())? };

    // Compute the contract address
    let init_code_hash = keccak256(init_code.clone());

    let mut input = vec![0xff];
    input.extend_from_slice(&runner.caller);
    input.extend_from_slice(&salt);
    input.extend_from_slice(&init_code_hash);

    let hash = keccak256(input);
    let contract_address: [u8; 20] = hash[12..].try_into().unwrap();
    
    // Create the contract with init code as code
    init_account(contract_address, runner)?;
    runner.state.put_code_at(contract_address, init_code)?;

    // Call the contract to run its constructor
    let temp_debug_level = runner.debug_level;
    runner.debug_level = Some(0);
    let call_result = runner.call(contract_address, value, Vec::new(), runner.gas, false);
    runner.debug_level = temp_debug_level;

    // Check if the call failed
    if call_result.is_err() {
        unsafe { runner.stack.push(pad_to_32_bytes(&[0x00]))? };
    } else {
        unsafe { runner.stack.push(pad_to_32_bytes(&[0x01]))? };
    }

    // Get the return data to store the real contract code
    let returndata = runner.returndata.heap.clone();
    runner.state.put_code_at(contract_address, returndata)?;

    // Transfer the value
    runner
        .state
        .transfer(runner.caller, contract_address, value)?;

    unsafe { runner.stack.push(pad_to_32_bytes(&contract_address)) }?;

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let hex: String = utils::debug::to_hex_string(pad_to_32_bytes(&contract_address));
        println!("{:<14} 👉 [ {} ]", "CREATE2".bright_blue(), hex);
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn call(runner: &mut Runner, bypass_static: bool) -> Result<(), ExecutionError> {
    // Check if static mode is enabled
    if runner.state.static_mode && !bypass_static {
        return Err(ExecutionError::StaticCallStateChanged);
    }

    // Get the values on the stack
    let gas = unsafe { runner.stack.pop()? };
    let to = unsafe { runner.stack.pop()? };
    let value = unsafe { runner.stack.pop()? };
    let calldata_offset = U256::from_big_endian(&unsafe { runner.stack.pop()? });
    let calldata_size = U256::from_big_endian(&unsafe { runner.stack.pop()? });
    let returndata_offset = U256::from_big_endian(&unsafe { runner.stack.pop()? });
    let returndata_size = U256::from_big_endian(&unsafe { runner.stack.pop()? });

    // Check if the address is out of bounds
    if calldata_offset.as_usize() + calldata_size.as_usize() > runner.memory.heap.len() {
        return Err(ExecutionError::OutOfBoundsMemory);
    }

    // Load the input data from memory
    let calldata = unsafe {
        runner
            .memory
            .read(calldata_offset.as_usize(), calldata_size.as_usize())?
    };

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let address_hex: String = utils::debug::to_hex_address(bytes32_to_address(&to));
        let calldata_hex: String = utils::debug::vec_to_hex_string(calldata.clone());
        println!(
            "\n{} 👉 {}\n  {}: {}\n",
            "CALL".yellow(),
            address_hex,
            "Calldata".bright_blue(),
            calldata_hex
        );
    }

    // Call the contract
    let call_result = runner.call(
        bytes32_to_address(&to),
        value,
        calldata,
        U256::from_big_endian(&gas).as_u64(),
        false
    );

    if call_result.is_err() {
        unsafe { runner.stack.push(pad_to_32_bytes(&[0x00]))? };
    } else {
        unsafe { runner.stack.push(pad_to_32_bytes(&[0x01]))? };
    }

    let return_data = runner.returndata.heap.clone();

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let address_hex: String = utils::debug::to_hex_address(runner.address);
        let returndata_hex: String = utils::debug::vec_to_hex_string(return_data.clone());
        println!(
            "\n{} {} {}\n  {}: {}\n",
            if call_result.is_err() {
                "CALL FAILED".red()
            } else {
                "CALL SUCCEEDED".green()
            },
            if call_result.is_err() { "❌" } else { "✅" },
            address_hex,
            "Returndata".bright_blue(),
            returndata_hex
        );
    }

    let mut return_data: Vec<u8> = runner.returndata.heap.clone();

    // Complete return data with zeros if returndata is smaller than returndata_size
    if return_data.len() < returndata_size.as_usize() {
        return_data.extend(vec![0; returndata_size.as_usize() - return_data.len()]);
    }

    return_data = return_data[0..returndata_size.as_usize()].to_vec();

    // Write the return data to memory
    unsafe {
        runner
            .memory
            .write(returndata_offset.as_usize(), return_data)?
    };

    // Increment PC
    runner.increment_pc(1)
}

pub fn callcode(_: &mut Runner) -> Result<(), ExecutionError> {
    Err(ExecutionError::NotImplemented(0xF2))
}

pub fn delegatecall(runner: &mut Runner) -> Result<(), ExecutionError> {
    // Get the values on the stack
    let gas = unsafe { runner.stack.pop()? };
    let to = unsafe { runner.stack.pop()? };
    let calldata_offset = U256::from_big_endian(&unsafe { runner.stack.pop()? });
    let calldata_size = U256::from_big_endian(&unsafe { runner.stack.pop()? });
    let returndata_offset = U256::from_big_endian(&unsafe { runner.stack.pop()? });
    let returndata_size = U256::from_big_endian(&unsafe { runner.stack.pop()? });

    // Check if the address is out of bounds
    if calldata_offset.as_usize() + calldata_size.as_usize() > runner.memory.heap.len() {
        return Err(ExecutionError::OutOfBoundsMemory);
    }

    // Load the input data from memory
    let calldata = unsafe {
        runner
            .memory
            .read(calldata_offset.as_usize(), calldata_size.as_usize())?
    };

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let address_hex: String = utils::debug::to_hex_address(bytes32_to_address(&to));
        let calldata_hex: String = utils::debug::vec_to_hex_string(calldata.clone());
        println!(
            "\n{} 👉 {}\n  {}: {}\n",
            "DELEGATE".yellow(),
            address_hex,
            "Calldata".bright_blue(),
            calldata_hex
        );
    }

    // Call the contract
    let call_result = runner.call(
        bytes32_to_address(&to),
        [0u8; 32],
        calldata,
        U256::from_big_endian(&gas).as_u64(),
        true
    );

    if call_result.is_err() {
        unsafe { runner.stack.push(pad_to_32_bytes(&[0x00]))? };
    } else {
        unsafe { runner.stack.push(pad_to_32_bytes(&[0x01]))? };
    }

    let return_data = runner.returndata.heap.clone();

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let address_hex: String = utils::debug::to_hex_address(runner.address);
        let returndata_hex: String = utils::debug::vec_to_hex_string(return_data.clone());
        println!(
            "\n{} {} {}\n  {}: {}\n",
            if call_result.is_err() {
                "DELEGATECALL FAILED".red()
            } else {
                "DELEGATECALL SUCCEEDED".green()
            },
            if call_result.is_err() { "❌" } else { "✅" },
            address_hex,
            "Returndata".bright_blue(),
            returndata_hex
        );
    }

    let mut return_data: Vec<u8> = runner.returndata.heap.clone();

    // Complete return data with zeros if returndata is smaller than returndata_size
    if return_data.len() < returndata_size.as_usize() {
        return_data.extend(vec![0; returndata_size.as_usize() - return_data.len()]);
    }

    return_data = return_data[0..returndata_size.as_usize()].to_vec();

    // Write the return data to memory
    unsafe {
        runner
            .memory
            .write(returndata_offset.as_usize(), return_data)?
    };

    // Increment PC
    runner.increment_pc(1)
}

pub fn staticcall(runner: &mut Runner) -> Result<(), ExecutionError> {
    runner.state.static_mode = true;
    let result = call(runner, true);
    runner.state.static_mode = false;
    
    result
}

// return
pub fn return_(runner: &mut Runner) -> Result<(), ExecutionError> {
    // Get the values on the stack
    let offset = U256::from_big_endian(&unsafe { runner.stack.pop()? });
    let size = U256::from_big_endian(&unsafe { runner.stack.pop()? });

    // Check if the address is out of bounds
    if offset.as_usize() + size.as_usize() > runner.memory.heap.len() {
        return Err(ExecutionError::OutOfBoundsMemory);
    }

    // Load the return data from memory
    let returndata = unsafe { runner.memory.read(offset.as_usize(), size.as_usize())? };

    // Set the return data
    runner.returndata.heap = returndata;

    // Increment PC
    runner.increment_pc(1)
}
