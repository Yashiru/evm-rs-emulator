use crate::core_module::runner::Runner;
use crate::core_module::utils;
use crate::core_module::utils::bytes::{bytes32_to_address, pad_left};
use crate::core_module::utils::environment::get_balance;
use crate::core_module::utils::errors::ExecutionError;
use std::time::{SystemTime, UNIX_EPOCH};

// Primitive types
use ethers::types::U256;
use ethers::utils::keccak256;

// Colored output
use colored::*;

pub fn address(runner: &mut Runner) -> Result<(), ExecutionError> {
    let address = [[0; 12].to_vec(), runner.address.to_vec()]
        .concat()
        .as_slice()
        .try_into()
        .unwrap();

    let result = unsafe { runner.stack.push(address) };

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let hex: String = utils::debug::to_hex_string(address);
        println!("{:<14} 👉 [ {} ]", "ADDRESS".bright_blue(), hex);
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn balance(runner: &mut Runner) -> Result<(), ExecutionError> {
    let address: [u8; 32] = unsafe { runner.stack.pop()? };
    let address: [u8; 20] = address[12..].try_into().unwrap();

    let balance = get_balance(address, runner)?;

    let result = unsafe { runner.stack.push(pad_left(&balance)) };

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let hex: String = utils::debug::to_hex_string(balance);
        println!("{:<14} 👉 [ {} ]", "BALANCE".bright_blue(), hex);
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn origin(runner: &mut Runner) -> Result<(), ExecutionError> {
    let origin = [[0; 12].to_vec(), runner.origin.to_vec()]
        .concat()
        .as_slice()
        .try_into()
        .unwrap();

    let result = unsafe { runner.stack.push(origin) };

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let hex: String = utils::debug::to_hex_string(origin);
        println!("{:<14} 👉 [ {} ]", "ORIGIN".bright_blue(), hex);
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn caller(runner: &mut Runner) -> Result<(), ExecutionError> {
    let caller = [[0; 12].to_vec(), runner.caller.to_vec()]
        .concat()
        .as_slice()
        .try_into()
        .unwrap();

    let result = unsafe { runner.stack.push(caller) };

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let hex: String = utils::debug::to_hex_string(caller);
        println!("{:<14} 👉 [ {} ]", "CALLER".bright_blue(), hex);
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn callvalue(runner: &mut Runner) -> Result<(), ExecutionError> {
    let result = unsafe { runner.stack.push(runner.callvalue) };

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let hex: String = utils::debug::to_hex_string(runner.callvalue);
        println!("{:<14} 👉 [ {} ]", "CALLVALUE".bright_blue(), hex);
    }

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn calldataload(runner: &mut Runner) -> Result<(), ExecutionError> {
    let address = unsafe { runner.stack.pop()? };
    let address = U256::from_big_endian(&address).as_usize();

    let calldata = unsafe { runner.calldata.read(address, 32)? };
    let calldata = calldata.as_slice().try_into().unwrap();

    let result = unsafe { runner.stack.push(calldata) };

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let hex: String = utils::debug::to_hex_string(calldata);
        println!("{:<14} 👉 [ {} ]", "CALLDATALOAD".bright_blue(), hex);
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn calldatasize(runner: &mut Runner) -> Result<(), ExecutionError> {
    let size = runner.calldata.msize().to_be_bytes();

    // Convert the usize to bytes in little-endian order
    let calldatasize = pad_left(&size);

    let result = unsafe { runner.stack.push(calldatasize) };

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let hex: String = utils::debug::to_hex_string(calldatasize);
        println!("{:<14} 👉 [ {} ]", "CALLDATASIZE".bright_blue(), hex);
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn calldatacopy(runner: &mut Runner) -> Result<(), ExecutionError> {
    let dest_offset = U256::from_big_endian(&unsafe { runner.stack.pop()? }).as_usize();
    let _offset = U256::from_big_endian(&unsafe { runner.stack.pop()? }).as_usize();
    let _size = U256::from_big_endian(&unsafe { runner.stack.pop()? }).as_usize();

    let calldata = unsafe { runner.calldata.read(_offset, _size)? };

    let result = unsafe { runner.memory.write(dest_offset, calldata) };

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        println!("{}", "CALLDATACOPY".bright_blue());
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn codesize(runner: &mut Runner) -> Result<(), ExecutionError> {
    let code = runner.state.get_code_at(runner.address);

    let codesize = if code.is_err() {
        [0u8; 32]
    } else {
        pad_left(&code.unwrap().len().to_be_bytes())
    };

    let result = unsafe { runner.stack.push(codesize) };

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let hex: String = utils::debug::to_hex_string(codesize);
        println!("{:<14} 👉 [ {} ]", "CODESIZE".bright_blue(), hex);
    }

    // Increment PC
    runner.increment_pc(1)
}

// Can be mocked with a fork
pub fn codecopy(runner: &mut Runner) -> Result<(), ExecutionError> {
    let dest_offset = U256::from_big_endian(&unsafe { runner.stack.pop()? }).as_usize();
    let offset = U256::from_big_endian(&unsafe { runner.stack.pop()? }).as_usize();
    let size = U256::from_big_endian(&unsafe { runner.stack.pop()? }).as_usize();

    let code = runner.state.get_code_at(runner.address);

    // Slice the code to the correct size
    let code = if code.is_err() {
        vec![]
    } else {
        // complete the code with 0s
        let code = code.unwrap();
        let mut code_vec = code.to_vec();
        code_vec.resize(32, 0);
        let code = code_vec.as_slice();
        code[offset..offset + size].to_vec()
    };

    // Copy the code to memory
    unsafe { runner.memory.write(dest_offset, code) }?;

    // print the memory heap
    println!("{:?}", runner.memory.heap);

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        println!("{}", "CODECOPY".bright_blue());
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn gasprice(runner: &mut Runner) -> Result<(), ExecutionError> {
    let gasprice = [[0; 31].to_vec(), [0xff].to_vec()]
        .concat()
        .as_slice()
        .try_into()
        .unwrap();

    let result = unsafe { runner.stack.push(gasprice) };

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let hex: String = utils::debug::to_hex_string(gasprice);
        println!("{:<14} 👉 [ {} ]", "GASPRICE".bright_blue(), hex);
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn extcodesize(runner: &mut Runner) -> Result<(), ExecutionError> {
    let address = unsafe { runner.stack.pop()? };

    let code = runner.state.get_code_at(bytes32_to_address(&address));

    let codesize = if code.is_err() {
        [0u8; 32]
    } else {
        pad_left(&code.unwrap().len().to_be_bytes())
    };

    let result = unsafe { runner.stack.push(codesize) };

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let hex: String = utils::debug::to_hex_string(codesize);
        println!("{:<14} 👉 [ {} ]", "EXTCODESIZE".bright_blue(), hex);
    }

    // Increment PC
    runner.increment_pc(1)
}

// Can be mocked with a fork
pub fn extcodecopy(runner: &mut Runner) -> Result<(), ExecutionError> {
    let address = unsafe { runner.stack.pop()? };
    let dest_offset = U256::from_big_endian(&unsafe { runner.stack.pop()? }).as_usize();
    let offset = U256::from_big_endian(&unsafe { runner.stack.pop()? }).as_usize();
    let size = U256::from_big_endian(&unsafe { runner.stack.pop()? }).as_usize();

    let code = runner.state.get_code_at(bytes32_to_address(&address));

    // Slice the code to the correct size
    let code = if code.is_err() {
        vec![]
    } else {
        // complete the code with 0s
        let code = code.unwrap();
        let mut code_vec = code.to_vec();
        code_vec.resize(32, 0);
        let code = code_vec.as_slice();
        code[offset..offset + size].to_vec()
    };

    // Copy the code to memory
    unsafe { runner.memory.write(dest_offset, code) }?;

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        println!("{}", "EXTCODECOPY".bright_blue());
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn returndatasize(runner: &mut Runner) -> Result<(), ExecutionError> {
    let size = runner.returndata.msize().to_be_bytes();

    // Convert the usize to bytes in little-endian order
    let returndatasize = pad_left(&size);

    let result = unsafe { runner.stack.push(returndatasize) };

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let hex: String = utils::debug::to_hex_string(returndatasize);
        println!("{:<14} 👉 [ {} ]", "RETURNDATASIZE".bright_blue(), hex);
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn returndatacopy(runner: &mut Runner) -> Result<(), ExecutionError> {
    let dest_offset = U256::from_big_endian(&unsafe { runner.stack.pop()? }).as_usize();
    let _offset = U256::from_big_endian(&unsafe { runner.stack.pop()? }).as_usize();
    let _size = U256::from_big_endian(&unsafe { runner.stack.pop()? }).as_usize();

    let returndata = unsafe { runner.returndata.read(_offset, _size)? };

    let result = unsafe { runner.memory.write(dest_offset, returndata) };

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        println!("{}", "RETURNDATACOPY".bright_blue());
    }

    // Increment PC
    runner.increment_pc(1)
}

// Can be mocked with a fork
pub fn extcodehash(runner: &mut Runner) -> Result<(), ExecutionError> {
    let address = unsafe { runner.stack.pop()? };

    let code = runner.state.get_code_at(bytes32_to_address(&address))?;
    let codehash = keccak256(&code);

    let result = unsafe { runner.stack.push(codehash) };

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let hex: String = utils::debug::to_hex_string(codehash);
        println!("{:<14} 👉 [ {} ]", "EXTCODEHASH".bright_blue(), hex);
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn blockhash(runner: &mut Runner) -> Result<(), ExecutionError> {
    let block: U256 = U256::from_big_endian(&unsafe { runner.stack.pop()? });
    let mut bytes = [0; 32];
    block.to_big_endian(&mut bytes);

    let blockhash = keccak256(bytes);

    let result = unsafe { runner.stack.push(blockhash) };

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let hex: String = utils::debug::to_hex_string(blockhash);
        println!("{:<14} 👉 [ {} ]", "BLOCKHASH".bright_blue(), hex);
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn coinbase(runner: &mut Runner) -> Result<(), ExecutionError> {
    let coinbase = pad_left(&[0xc0u8; 20]);

    let result = unsafe { runner.stack.push(coinbase) };

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let hex: String = utils::debug::to_hex_string(coinbase);
        println!("{:<14} 👉 [ {} ]", "COINBASE".bright_blue(), hex);
    }

    // Increment PC
    runner.increment_pc(1)
}

// Can be mocked with a fork
pub fn timestamp(runner: &mut Runner) -> Result<(), ExecutionError> {
    // Get the current timestamp
    let now = SystemTime::now();
    let since_the_epoch = now
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    // Convert the timestamp to seconds
    let timestamp_secs = since_the_epoch.as_secs();

    // Convert the timestamp to bytes in big-endian order
    let timestamp_bytes = timestamp_secs.to_be_bytes();

    let bytes = pad_left(&timestamp_bytes);

    let result = unsafe { runner.stack.push(bytes) };

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let hex: String = utils::debug::to_hex_string(bytes);
        println!("{:<14} 👉 [ {} ]", "TIMESTAMP".bright_blue(), hex);
    }

    // Increment PC
    runner.increment_pc(1)
}

// Can be mocked with a fork
pub fn number(runner: &mut Runner) -> Result<(), ExecutionError> {
    let number = pad_left(&[0xff; 4]);

    let result = unsafe { runner.stack.push(number) };

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let hex: String = utils::debug::to_hex_string(number);
        println!("{:<14} 👉 [ {} ]", "NUMBER".bright_blue(), hex);
    }

    // Increment PC
    runner.increment_pc(1)
}

// Can be mocked with a fork
pub fn difficulty(runner: &mut Runner) -> Result<(), ExecutionError> {
    let difficulty = pad_left(&[0x45; 8]);

    let result = unsafe { runner.stack.push(difficulty) };

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let hex: String = utils::debug::to_hex_string(difficulty);
        println!("{:<14} 👉 [ {} ]", "DIFFICULTY".bright_blue(), hex);
    }

    // Increment PC
    runner.increment_pc(1)
}

// Can be mocked with a fork
pub fn gaslimit(runner: &mut Runner) -> Result<(), ExecutionError> {
    let gaslimit = pad_left(&[0x01, 0xC9, 0xC3, 0x80]);

    let result = unsafe { runner.stack.push(gaslimit) };

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let hex: String = utils::debug::to_hex_string(gaslimit);
        println!("{:<14} 👉 [ {} ]", "GASLIMIT".bright_blue(), hex);
    }

    // Increment PC
    runner.increment_pc(1)
}

// Can be mocked with a fork
pub fn chainid(runner: &mut Runner) -> Result<(), ExecutionError> {
    let chainid = pad_left(&[0x01]);

    let result = unsafe { runner.stack.push(chainid) };

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let hex: String = utils::debug::to_hex_string(chainid);
        println!("{:<14} 👉 [ {} ]", "CHAINID".bright_blue(), hex);
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn selfbalance(runner: &mut Runner) -> Result<(), ExecutionError> {
    let balance = get_balance(runner.address, runner)?;

    let result = unsafe { runner.stack.push(balance) };

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let hex: String = utils::debug::to_hex_string(balance);
        println!("{:<14} 👉 [ {} ]", "SELFBALANCE".bright_blue(), hex);
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn basefee(runner: &mut Runner) -> Result<(), ExecutionError> {
    let basefee = pad_left(&[0x0a]);

    let result = unsafe { runner.stack.push(basefee) };

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let hex: String = utils::debug::to_hex_string(basefee);
        println!("{:<14} 👉 [ {} ]", "BASEFEE".bright_blue(), hex);
    }

    // Increment PC
    runner.increment_pc(1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core_module::utils::bytes::{hex_string_to_bytes, pad_left, pad_right};

    #[test]
    fn test_address() {
        let mut runner = Runner::default(3);
        address(&mut runner).unwrap();

        let result = unsafe { runner.stack.pop().unwrap() };
        assert_eq!(result, pad_left(&runner.address));
    }

    #[test]
    fn test_balance() {
        let mut runner = Runner::default(3);
        let _ = unsafe { runner.stack.push(pad_left(&runner.caller)) };
        balance(&mut runner).unwrap();

        let result = unsafe { runner.stack.pop().unwrap() };
        assert_eq!(
            result,
            pad_left(&[0x36, 0x35, 0xC9, 0xAD, 0xC5, 0xDE, 0xA0, 0x00, 0x00])
        );

        // transfer 100 wei to the contract
        let _ = runner
            .state
            .transfer(runner.caller, runner.address, pad_left(&[0x01]));

        let _ = unsafe { runner.stack.push(pad_left(&runner.caller)) };
        balance(&mut runner).unwrap();

        let result = unsafe { runner.stack.pop().unwrap() };
        assert_eq!(
            result,
            pad_left(&[0x36, 0x35, 0xC9, 0xAD, 0xC5, 0xDE, 0x9F, 0xFF, 0xFF])
        );
    }

    #[test]
    fn test_origin() {
        let mut runner = Runner::default(3);
        origin(&mut runner).unwrap();

        let result = unsafe { runner.stack.pop().unwrap() };
        assert_eq!(result, pad_left(&runner.origin));
    }

    #[test]
    fn test_caller() {
        let mut runner = Runner::default(3);
        caller(&mut runner).unwrap();

        let result = unsafe { runner.stack.pop().unwrap() };
        assert_eq!(result, pad_left(&runner.caller));
    }

    #[test]
    fn test_callvalue() {
        let mut runner = Runner::default(3);
        callvalue(&mut runner).unwrap();

        let result = unsafe { runner.stack.pop().unwrap() };
        assert_eq!(result, pad_left(&[0x00]));
    }

    #[test]
    fn test_calldataload() {
        let mut runner = Runner::default(3);
        runner.calldata.heap = vec![0xff, 0xff, 0xff, 0xff];

        let _ = unsafe { runner.stack.push(pad_left(&[0x00])) };
        calldataload(&mut runner).unwrap();

        let result = unsafe { runner.stack.pop().unwrap() };
        assert_eq!(result, pad_right(&[0xff, 0xff, 0xff, 0xff]));

        let _ = unsafe { runner.stack.push(pad_left(&[0x02])) };
        calldataload(&mut runner).unwrap();

        let result = unsafe { runner.stack.pop().unwrap() };
        assert_eq!(result, pad_right(&[0xff, 0xff]));
    }

    #[test]
    fn test_calldatasize() {
        let mut runner = Runner::default(3);
        runner.calldata.heap = vec![0xff, 0xff, 0xff, 0xff];

        calldatasize(&mut runner).unwrap();

        let result = unsafe { runner.stack.pop().unwrap() };
        assert_eq!(result, pad_left(&[0x04]));
    }

    #[test]
    fn test_calldatacopy() {
        let mut runner = Runner::default(3);
        runner.calldata.heap = [0xff; 32].to_vec();

        let _ = unsafe { runner.stack.push(pad_left(&[0x20])) };
        let _ = unsafe { runner.stack.push(pad_left(&[0x00])) };
        let _ = unsafe { runner.stack.push(pad_left(&[0x00])) };
        calldatacopy(&mut runner).unwrap();

        let result = unsafe { runner.memory.read(0x00, 0x20).unwrap() };
        assert_eq!(result, [0xff; 32].to_vec());

        let _ = unsafe { runner.stack.push(pad_left(&[0x10])) };
        let _ = unsafe { runner.stack.push(pad_left(&[0x00])) };
        let _ = unsafe { runner.stack.push(pad_left(&[0x00])) };
        calldatacopy(&mut runner).unwrap();

        let result = unsafe { runner.memory.read(0x00, 0x20).unwrap() };
        assert_eq!(result, [0xff; 32].to_vec());

        runner.memory.heap = vec![0x00; 32];
        let _ = unsafe { runner.stack.push(pad_left(&[0x10])) };
        let _ = unsafe { runner.stack.push(pad_left(&[0x00])) };
        let _ = unsafe { runner.stack.push(pad_left(&[0x00])) };
        calldatacopy(&mut runner).unwrap();

        let result = unsafe { runner.memory.read(0x00, 0x20).unwrap() };
        assert_eq!(result, pad_right(&[0xff; 16]).to_vec());
    }

    #[test]
    fn test_codesize() {
        let mut runner = Runner::default(3);

        // Interpret some code to make set the runner code to something
        runner
            .interpret(hex_string_to_bytes("60ff6000526001601ff3"), Some(5), true)
            .unwrap();

        codesize(&mut runner).unwrap();

        let result = unsafe { runner.stack.pop().unwrap() };
        assert_eq!(result, pad_left(&[0xa]));
    }

    #[test]
    fn test_codecopy() {
        let mut runner = Runner::default(3);

        // Create a contract with a bytecode length of 23
        let interpret_result = runner.interpret(
            hex_string_to_bytes(
                "7dffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff6000",
            ),
            Some(5),
            true
        );
        assert!(interpret_result.is_ok());

        let _ = unsafe { runner.stack.push(pad_left(&[0x20])) };
        let _ = unsafe { runner.stack.push(pad_left(&[0x00])) };
        let _ = unsafe { runner.stack.push(pad_left(&[0x00])) };
        codecopy(&mut runner).unwrap();

        let result = unsafe { runner.memory.read(0x00, 0x20).unwrap() };
        assert_eq!(
            result,
            hex_string_to_bytes("7dffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff60")
        );

        // reset memory
        runner.memory.heap = vec![];

        let _ = unsafe { runner.stack.push(pad_left(&[0x05])) };
        let _ = unsafe { runner.stack.push(pad_left(&[0x00])) };
        let _ = unsafe { runner.stack.push(pad_left(&[0x00])) };
        codecopy(&mut runner).unwrap();

        let result = unsafe { runner.memory.read(0x00, 0x20).unwrap() };
        assert_eq!(result, pad_right(&hex_string_to_bytes("7dffffffff")));
    }

    #[test]
    fn test_gasprice() {
        let mut runner = Runner::default(3);
        gasprice(&mut runner).unwrap();

        let result = unsafe { runner.stack.pop().unwrap() };
        assert_eq!(result, pad_left(&[0xff]));
    }

    #[test]
    fn test_extcodesize() {
        let mut runner = Runner::default(3);

        // Create a contract with a bytecode length of 23
        let interpret_result = runner.interpret(
            hex_string_to_bytes("7f76ffffffffffffffffffffffffffffffffffffffffffffff60005260176009f3600052602060006000f0"),
            Some(5),
            true
        );
        assert!(interpret_result.is_ok());

        extcodesize(&mut runner).unwrap();

        let result = unsafe { runner.stack.pop().unwrap() };
        assert_eq!(result, pad_left(&[0x17]));
    }

    #[test]
    fn test_extcodecopy() {
        let mut runner = Runner::default(3);

        // Create a contract with a bytecode length of 23
        let interpret_result = runner.interpret(
            hex_string_to_bytes("7f76ffffffffffffffffffffffffffffffffffffffffffffff60005260176009f3600052602060006000f0"),
            Some(5),
            true
        );
        assert!(interpret_result.is_ok());

        // reset memory
        runner.memory.heap = vec![];

        let _ = unsafe { runner.stack.push(pad_left(&[0x17])) };
        let _ = unsafe { runner.stack.push(pad_left(&[0x00])) };
        let _ = unsafe { runner.stack.push(pad_left(&[0x00])) };
        let _ = unsafe { runner.stack.dup(4) };
        extcodecopy(&mut runner).unwrap();

        let result = unsafe { runner.memory.read(0x00, 0x20).unwrap() };
        assert_eq!(
            result,
            pad_right(&hex_string_to_bytes(
                "ffffffffffffffffffffffffffffffffffffffffffffff"
            ))
        );

        // reset memory
        runner.memory.heap = vec![];

        let _ = unsafe { runner.stack.push(pad_left(&[0xa])) };
        let _ = unsafe { runner.stack.push(pad_left(&[0x00])) };
        let _ = unsafe { runner.stack.push(pad_left(&[0x20])) };
        let _ = unsafe { runner.stack.dup(4) };
        extcodecopy(&mut runner).unwrap();

        let result = unsafe { runner.memory.read(0x00, 0x20).unwrap() };
        assert_eq!(result, [0u8; 32]);
        let result = unsafe { runner.memory.read(0x20, 0x20).unwrap() };
        assert_eq!(
            result,
            pad_right(&hex_string_to_bytes("ffffffffffffffffffff"))
        );
    }

    #[test]
    fn test_returndatasize() {
        let mut runner = Runner::default(3);

        // Create a contract that return 0x20 sized data and call it
        let interpret_result = runner.interpret(
            hex_string_to_bytes("7f7f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff6000527fff6000527fff60005260206000f30000000000000000000000000000000000006020527f000000000060205260296000f300000000000000000000000000000000000000604052604d60006000f060006000600060008463fffffffffa3d"),
            Some(255),
            true
        );
        assert!(interpret_result.is_ok());

        let result = unsafe { runner.stack.pop().unwrap() };
        assert_eq!(result, pad_left(&[0x20]));
    }

    #[test]
    fn test_returndatacopy() {
        let mut runner = Runner::default(3);

        // Create a contract that return 0x20 sized data and call it
        let interpret_result = runner.interpret(
            hex_string_to_bytes("7f7f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff6000527fff6000527fff60005260206000f30000000000000000000000000000000000006020527f000000000060205260296000f300000000000000000000000000000000000000604052604d60006000f060006000600060008463fffffffffa50506000600052600060205260006040526020600060003e6001601f60203e"),
            Some(255),
            true
        );
        assert!(interpret_result.is_ok());

        let result = unsafe { runner.memory.read(0x00, 0x20).unwrap() };
        assert_eq!(result, [0xff; 32]);
        let result = unsafe { runner.memory.read(0x20, 0x20).unwrap() };
        assert_eq!(result, pad_right(&[0xff]));
        let result = unsafe { runner.memory.read(0x40, 0x20).unwrap() };
        assert_eq!(result, [0x00; 32]);
    }

    #[test]
    fn test_extcodehash() {
        let mut runner = Runner::default(3);

        // Create a contract with a bytecode length of 23
        let interpret_result = runner.interpret(
            hex_string_to_bytes("6c63ffffffff60005260046000f3600052600d60006000f03f"),
            Some(5),
            true
        );
        assert!(interpret_result.is_ok());

        let result = unsafe { runner.stack.pop().unwrap() };
        assert_eq!(
            result,
            pad_left(&hex_string_to_bytes("c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470"))
        );
    }

    #[test]
    fn test_blockhash() {
        // TODO: test with a fork
    }

    #[test]
    fn test_coinbase() {
        let mut runner = Runner::default(3);
        coinbase(&mut runner).unwrap();

        let result = unsafe { runner.stack.pop().unwrap() };
        assert_eq!(result, pad_left(&[0xc0; 20]));
    }

    #[test]
    fn test_timestamp() {
        let mut runner = Runner::default(3);
        timestamp(&mut runner).unwrap();

        // Get the current timestamp
        let now = SystemTime::now();
        let since_the_epoch = now
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        // Convert the timestamp to seconds
        let timestamp_secs = since_the_epoch.as_secs();

        // Convert the timestamp to bytes in big-endian order
        let timestamp_bytes = timestamp_secs.to_be_bytes();

        let result = unsafe { runner.stack.pop().unwrap() };
        assert_eq!(result, pad_left(&timestamp_bytes));
    }

    #[test]
    fn test_number() {
        // TODO: test with a fork
        let mut runner = Runner::default(3);
        number(&mut runner).unwrap();

        let result = unsafe { runner.stack.pop().unwrap() };
        assert_eq!(result, pad_left(&[0xff; 4]));
    }

    #[test]
    fn test_difficulty() {
        // TODO: test with a fork
        let mut runner = Runner::default(3);
        difficulty(&mut runner).unwrap();

        let result = unsafe { runner.stack.pop().unwrap() };
        assert_eq!(result, pad_left(&[0x45; 8]));
    }

    #[test]
    fn test_gaslimit() {
        let mut runner = Runner::default(3);
        gaslimit(&mut runner).unwrap();

        let result = unsafe { runner.stack.pop().unwrap() };
        assert_eq!(result, pad_left(&[0x01, 0xC9, 0xC3, 0x80]));
    }

    #[test]
    fn test_chainid() {
        let mut runner = Runner::default(3);
        chainid(&mut runner).unwrap();

        let result = unsafe { runner.stack.pop().unwrap() };
        assert_eq!(result, pad_left(&[0x01]));
    }

    #[test]
    fn test_selfbalance() {
        let mut runner = Runner::default(3);
        selfbalance(&mut runner).unwrap();

        let result = unsafe { runner.stack.pop().unwrap() };
        assert_eq!(result, pad_left(&[0x00]));

        // transfer 100 wei to the contract
        let _ = runner
            .state
            .transfer(runner.caller, runner.address, pad_left(&[0x64]));
        selfbalance(&mut runner).unwrap();

        let result = unsafe { runner.stack.pop().unwrap() };
        assert_eq!(result, pad_left(&[0x64]));
    }

    #[test]
    fn test_basefee() {
        let mut runner = Runner::default(3);
        basefee(&mut runner).unwrap();

        let result = unsafe { runner.stack.pop().unwrap() };
        assert_eq!(result, pad_left(&[0x0a]));
    }
}
