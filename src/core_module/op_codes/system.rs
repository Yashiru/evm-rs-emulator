use crate::core_module::runner::Runner;
use crate::core_module::utils;
use crate::core_module::utils::bytes;
use crate::core_module::utils::bytes::{bytes32_to_address, pad_left};
use crate::core_module::utils::environment::{
    delete_account, get_balance, get_nonce, init_account,
};
use crate::core_module::utils::errors::ExecutionError;

// Primitive types
use ethers::types::U256;
use ethers::utils::keccak256;

// Colored output
use colored::*;

pub fn invalid(runner: &mut Runner) -> Result<(), ExecutionError> {
    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        runner.print_debug(&format!("{:} 0x{:X}", "INVALID".red(), runner.bytecode[runner.pc]));
    }

    Err(ExecutionError::InvalidOpcode(runner.bytecode[runner.pc]))
}

pub fn create(runner: &mut Runner) -> Result<(), ExecutionError> {
    // Get the values on the stack
    let value = unsafe { runner.stack.pop()? };
    let offset = U256::from_big_endian(&unsafe { runner.stack.pop()? });
    let size = U256::from_big_endian(&unsafe { runner.stack.pop()? });

    // Load the init code from memory
    let init_code = unsafe { runner.memory.read(offset.as_usize(), size.as_usize())? };

    // Compute the contract address
    let mut input = vec![0xd6, 0x94];
    input.extend_from_slice(&runner.caller);
    input.extend_from_slice(&bytes::strip_zero_padding(&get_nonce(
        runner.address,
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
        unsafe { runner.stack.push(pad_left(&[0x00]))? };
    } else {
        unsafe { runner.stack.push(pad_left(&contract_address)) }?;
    }

    // Get the return data to store the real contract code
    let returndata = runner.returndata.heap.clone();
    runner
        .state
        .put_code_at(contract_address, returndata.clone())?;

    // Transfer the value
    runner
        .state
        .transfer(runner.caller, contract_address, value)?;

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let hex: String = utils::debug::to_hex_string(pad_left(&contract_address));
        runner.print_debug(&format!("{:<14} 👉 [ {} ]", "CREATE".bright_blue(), hex));
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
    runner.debug_level = Some(2);
    let call_result = runner.call(contract_address, value, Vec::new(), runner.gas, false);
    runner.debug_level = temp_debug_level;

    // Check if the call failed
    if call_result.is_err() {
        unsafe { runner.stack.push(pad_left(&[0x00]))? };
    } else {
        unsafe { runner.stack.push(pad_left(&contract_address)) }?;
    }

    // Get the return data to store the real contract code
    let returndata = runner.returndata.heap.clone();
    runner.state.put_code_at(contract_address, returndata)?;

    // Transfer the value
    runner
        .state
        .transfer(runner.caller, contract_address, value)?;

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let hex: String = utils::debug::to_hex_string(pad_left(&contract_address));
        runner.print_debug(&format!("{:<14} 👉 [ {} ]", "CREATE2".bright_blue(), hex));
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

    let value = if bypass_static {
        [0u8; 32]
    } else {
        unsafe { runner.stack.pop()? }
    };
    let calldata_offset = U256::from_big_endian(&unsafe { runner.stack.pop()? });
    let calldata_size = U256::from_big_endian(&unsafe { runner.stack.pop()? });
    let returndata_offset = U256::from_big_endian(&unsafe { runner.stack.pop()? });
    let returndata_size = U256::from_big_endian(&unsafe { runner.stack.pop()? });

    // Load the input data from memory
    let calldata = unsafe {
        runner
            .memory
            .read(calldata_offset.as_usize(), calldata_size.as_usize())?
    };

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let address_hex: String = utils::debug::to_hex_address(bytes32_to_address(&to));
        let calldata_hex: String = utils::debug::vec_to_hex_string(calldata.clone());
        runner.print_debug(&format!(
            "\n{} 👉 {}\n  {}: {}\n",
            if bypass_static {
                "STATICCALL".yellow()
            } else {
                "CALL".yellow()
            },
            address_hex,
            "Calldata".bright_blue(),
            calldata_hex
        ));
    }

    // Call the contract
    let call_result = runner.call(
        bytes32_to_address(&to),
        value,
        calldata,
        U256::from_big_endian(&gas).as_u64(),
        false,
    );

    if call_result.is_err() {
        unsafe { runner.stack.push(pad_left(&[0x00]))? };
    } else {
        unsafe { runner.stack.push(pad_left(&[0x01]))? };
    }

    let return_data = runner.returndata.heap.clone();

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let returndata_hex: String = utils::debug::vec_to_hex_string(return_data.clone());
        runner.print_debug(&format!(
            "\n{} {} {:?}\n  {}: {}\n",
            if call_result.is_err() {
                if bypass_static {
                    "STATICCALL FAILED".red()
                } else {
                    "CALL FAILED".red()
                }
            } else {
                if bypass_static {
                    "STATICCALL SUCCEEDED".green()
                } else {
                    "CALL SUCCEEDED".green()
                }
            },
            if call_result.is_err() { "❌" } else { "✅" },
            if call_result.is_err() { call_result.unwrap_err().to_string() } else { "".to_string() },
            "Returndata".bright_blue(),
            returndata_hex
        ));
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

    // Load the input data from memory
    let calldata = unsafe {
        runner
            .memory
            .read(calldata_offset.as_usize(), calldata_size.as_usize())?
    };

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let address_hex: String = utils::debug::to_hex_address(bytes32_to_address(&to));
        let calldata_hex: String = utils::debug::vec_to_hex_string(calldata.clone());
        runner.print_debug(&format!(
            "\n{} 👉 {}\n  {}: {}\n",
            "DELEGATE".yellow(),
            address_hex,
            "Calldata".bright_blue(),
            calldata_hex
        ));
    }

    // Call the contract
    let call_result = runner.call(
        bytes32_to_address(&to),
        [0u8; 32],
        calldata,
        U256::from_big_endian(&gas).as_u64(),
        true,
    );

    if call_result.is_err() {
        unsafe { runner.stack.push(pad_left(&[0x00]))? };
    } else {
        unsafe { runner.stack.push(pad_left(&[0x01]))? };
    }

    let return_data = runner.returndata.heap.clone();

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let address_hex: String = utils::debug::to_hex_address(runner.address);
        let returndata_hex: String = utils::debug::vec_to_hex_string(return_data.clone());
        runner.print_debug(&format!(
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
        ));
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

pub fn selfdestruct(runner: &mut Runner) -> Result<(), ExecutionError> {
    // Get the values on the stack
    let address = unsafe { runner.stack.pop()? };

    let contract_balance = get_balance(runner.address, runner)?;

    // Transfer the balance
    runner.state.transfer(
        runner.address,
        bytes32_to_address(&address),
        contract_balance,
    )?;

    // Delete the account
    delete_account(runner.address, runner)?;

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        runner.print_debug(&format!(
            "{}",
            "SELFDESTRUCT".bright_blue()
        ));
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn return_(runner: &mut Runner) -> Result<(), ExecutionError> {
    // Get the values on the stack
    let offset = U256::from_big_endian(&unsafe { runner.stack.pop()? });
    let size = U256::from_big_endian(&unsafe { runner.stack.pop()? });

    // Load the return data from memory
    let returndata = unsafe { runner.memory.read(offset.as_usize(), size.as_usize())? };

    // Set the return data
    runner.returndata.heap = returndata;

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        runner.print_debug(&format!(
            "{}",
            "RETURN".red()
        ));
    }

    // Increment PC
    runner.increment_pc(1)
}

#[cfg(test)]
mod tests {
    use crate::core_module::runner::Runner;
    use crate::core_module::utils::bytes::{_hex_string_to_bytes, bytes32_to_address, pad_left};
    use crate::core_module::utils::environment::get_balance;
    use crate::core_module::utils::errors::ExecutionError;

    #[test]
    fn test_invalid() {
        let mut runner = Runner::_default(3);
        let interpret_result: Result<(), ExecutionError> =
            runner.interpret(_hex_string_to_bytes("60fffe50fe60fffe"), Some(2), true);
        assert!(interpret_result.is_err());

        let result = unsafe { runner.stack.pop().unwrap() };
        assert_eq!(result, pad_left(&[0xff]));
    }

    #[test]
    fn test_create() {
        let mut runner = Runner::_default(3);
        let interpret_result: Result<(), ExecutionError> = runner.interpret(
            _hex_string_to_bytes("6c63ffffffff6000526004601cf3600052600d601360fff0"),
            Some(2),
            true,
        );
        assert!(interpret_result.is_ok());

        let result = unsafe { runner.stack.pop().unwrap() };
        assert_eq!(
            result,
            pad_left(&[
                0x9b, 0xbf, 0xed, 0x68, 0x89, 0x32, 0x2e, 0x01, 0x6e, 0x0a, 0x02, 0xee, 0x45, 0x9d,
                0x30, 0x6f, 0xc1, 0x95, 0x45, 0xd8
            ])
        );

        let stored_code = runner.state.get_code_at(bytes32_to_address(&result));

        assert_eq!(stored_code.unwrap(), &_hex_string_to_bytes("ffffffff"));

        let balance = get_balance(bytes32_to_address(&result), &mut runner).unwrap();
        assert_eq!(balance, pad_left(&[0xff]));
    }

    #[test]
    fn test_create2() {
        let mut runner = Runner::_default(3);
        let interpret_result: Result<(), ExecutionError> = runner.interpret(
            _hex_string_to_bytes("6c63ffffffff6000526004601cf360005263aaa4aaaf600d601360aff5"),
            Some(2),
            true,
        );
        assert!(interpret_result.is_ok());

        let result = unsafe { runner.stack.pop().unwrap() };
        assert_eq!(
            result,
            pad_left(&[
                0x5b, 0xad, 0x4e, 0xb0, 0xa4, 0xc4, 0xcf, 0xb7, 0x7d, 0x6c, 0x3f, 0x9d, 0x56, 0xa8,
                0x49, 0x03, 0x2f, 0x22, 0x47, 0xd2
            ])
        );

        let stored_code = runner.state.get_code_at(bytes32_to_address(&result));

        assert_eq!(stored_code.unwrap(), &_hex_string_to_bytes("ffffffff"));

        let balance = get_balance(bytes32_to_address(&result), &mut runner).unwrap();
        assert_eq!(balance, pad_left(&[0xaf]));
    }

    #[test]
    fn test_call() {
        let mut runner = Runner::_default(3);
        // Create a contract that creates an exception if first word of calldata is 0.
        // Call it two time with no calldata and with calldata.
        let interpret_result: Result<(), ExecutionError> = runner.interpret(
            _hex_string_to_bytes("7067600035600757fe5b60005260086018f36000526011600f6000f0600060006000600060008561fffff1600060006020600060008661fffff1"),
            Some(2),
            true,
        );
        assert!(interpret_result.is_ok());

        // Second call succeeded
        let result = unsafe { runner.stack.pop().unwrap() };
        assert!(result == pad_left(&[0x01]));

        // First call failed
        let result = unsafe { runner.stack.pop().unwrap() };
        assert!(result == pad_left(&[0x00]));
    }

    #[test]
    fn test_callcode() {
        let mut runner = Runner::_default(3);
        // Create a contract that creates an exception if first word of calldata is 0.
        // Call it two time with no calldata and with calldata.
        let interpret_result: Result<(), ExecutionError> =
            runner.interpret(_hex_string_to_bytes("f2"), Some(2), true);
        assert!(interpret_result.is_err());
        assert_eq!(
            interpret_result.unwrap_err(),
            ExecutionError::NotImplemented(0xF2)
        );
    }

    #[test]
    fn test_delegatecall() {
        let mut runner = Runner::_default(3);
        // Create a contract that creates an exception if first slot of storage is 0
        // Call it two time with no calldata and with calldata.
        let interpret_result: Result<(), ExecutionError> = runner.interpret(
            _hex_string_to_bytes("7067600054600757fe5b60005260086018f36000526011600f6000f060006000600060008461fffff4600160005560006000602060008561fffff4"),
            Some(2),
            true,
        );
        assert!(interpret_result.is_ok());

        // Second call succeeded
        let result = unsafe { runner.stack.pop().unwrap() };
        assert!(result == pad_left(&[0x01]));

        // First call failed
        let result = unsafe { runner.stack.pop().unwrap() };
        assert!(result == pad_left(&[0x00]));
    }

    #[test]
    fn test_staticcall() {
        let mut runner = Runner::_default(3);
        // Create a contract that creates an exception if first word of calldata is 0.
        // Call it two time with storage to 0 and storage to 1 (in the caller contract).
        let interpret_result: Result<(), ExecutionError> = runner.interpret(
            _hex_string_to_bytes("746b600035600b5760ff6000555b600052600c6014f36000526015600b6000f060006000600060008461fffffa60006000602060008561fffffa"),
            Some(2),
            true,
        );
        assert!(interpret_result.is_ok());

        // Second call succeeded
        let result = unsafe { runner.stack.pop().unwrap() };
        assert!(result == pad_left(&[0x01]));

        // First call failed
        let result = unsafe { runner.stack.pop().unwrap() };
        assert!(result == pad_left(&[0x00]));
    }

    #[test]
    fn test_selfdestruct() {
        let mut runner = Runner::_default(3);

        // Create a contract that has ff as code
        let interpret_result: Result<(), ExecutionError> = runner.interpret(
            _hex_string_to_bytes("6960ff6000526001601ff3600052600a601660aaf0"),
            Some(2),
            true,
        );
        assert!(interpret_result.is_ok());

        let address = unsafe { runner.stack.pop().unwrap() };
        assert_eq!(
            address,
            pad_left(&[
                0x9b, 0xbf, 0xed, 0x68, 0x89, 0x32, 0x2e, 0x01, 0x6e, 0x0a, 0x02, 0xee, 0x45, 0x9d,
                0x30, 0x6f, 0xc1, 0x95, 0x45, 0xd8
            ])
        );

        let stored_code = runner.state.get_code_at(bytes32_to_address(&address));

        assert_eq!(stored_code.unwrap(), &_hex_string_to_bytes("ff"));

        let balance = get_balance(bytes32_to_address(&address), &mut runner).unwrap();
        assert_eq!(balance, pad_left(&[0xaa]));

        // Set the code to the new contract to CALLER SELFDESTRUCT
        let put_code_result = runner
            .state
            .put_code_at(bytes32_to_address(&address), _hex_string_to_bytes("33ff"));
        assert!(put_code_result.is_ok());

        let mut string_address = String::new();
        for &byte in bytes32_to_address(&address).iter() {
            string_address.push_str(&format!("{:02x}", byte));
        }
        let bytecode = format!("73{}600060006000600060008561fffff1", string_address);
        let bytecode: &str = &bytecode;

        runner.pc = 0;
        // Self destruct the contract by calling it
        let selfdestruct_result: Result<(), ExecutionError> =
            runner.interpret(_hex_string_to_bytes(bytecode), Some(2), true);
        assert!(selfdestruct_result.is_ok());

        let result = unsafe { runner.stack.pop().unwrap() };
        assert_eq!(result, pad_left(&[0x01]));

        let stored_code = runner.state.get_code_at(bytes32_to_address(&result));

        assert!(stored_code.is_err());
        assert_eq!(stored_code.unwrap_err(), ExecutionError::AccountNotFound);

        let balance_result = get_balance(bytes32_to_address(&result), &mut runner);
        assert!(balance_result.is_err());
        assert_eq!(balance_result.unwrap_err(), ExecutionError::AccountNotFound);

        let receiver_balance = get_balance(runner.address, &mut runner).unwrap();
        assert_eq!(receiver_balance, balance);
    }
}
