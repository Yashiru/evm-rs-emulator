use crate::core_module::runner::Runner;
use crate::core_module::utils;
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

    if runner.debug.is_some() && runner.debug.unwrap() {
        let hex: String = utils::debug::to_hex_string(address);
        println!("{:<14} 👉 [ {} ]", "ADDRESS".bright_blue(), hex);
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn balance(runner: &mut Runner) -> Result<(), ExecutionError> {
    let address: [u8; 32] = unsafe { runner.stack.pop()? };

    let balance_slot = keccak256([utils::constants::balance_slot().to_vec(), address.to_vec()].concat());
    let balance = unsafe { runner.storage.sload(balance_slot)? };

    let result = unsafe { runner.stack.push(balance) };

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug.is_some() && runner.debug.unwrap() {
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

    if runner.debug.is_some() && runner.debug.unwrap() {
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

    if runner.debug.is_some() && runner.debug.unwrap() {
        let hex: String = utils::debug::to_hex_string(caller);
        println!("{:<14} 👉 [ {} ]", "CALLER".bright_blue(), hex);
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn callvalue(runner: &mut Runner) -> Result<(), ExecutionError> {
    let result = unsafe { runner.stack.push(runner.callvalue) };

    if runner.debug.is_some() && runner.debug.unwrap() {
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

    if runner.debug.is_some() && runner.debug.unwrap() {
        let hex: String = utils::debug::to_hex_string(calldata);
        println!("{:<14} 👉 [ {} ]", "CALLDATALOAD".bright_blue(), hex);
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn calldatasize(runner: &mut Runner) -> Result<(), ExecutionError> {
    let mut size = runner.calldata.msize().to_le_bytes();
    size.reverse();

    // Convert the usize to bytes in little-endian order
    let calldatasize = [[0u8; 24].to_vec(), size.to_vec()].concat().as_slice().try_into().unwrap();

    let result = unsafe { runner.stack.push(calldatasize) };

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug.is_some() && runner.debug.unwrap() {
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

    if runner.debug.is_some() && runner.debug.unwrap() {
        println!("{}", "CALLDATACOPY".bright_blue());
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn codesize(runner: &mut Runner) -> Result<(), ExecutionError> {
    let codesize = [0u8; 32];

    let result = unsafe { runner.stack.push(codesize) };

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug.is_some() && runner.debug.unwrap() {
        let hex: String = utils::debug::to_hex_string(codesize);
        println!("{:<14} 👉 [ {} ]", "CODESIZE".bright_blue(), hex);
    }

    // Increment PC
    runner.increment_pc(1)
}

// Can be mocked with a fork
pub fn codecopy(runner: &mut Runner) -> Result<(), ExecutionError> {
    let _ = U256::from_big_endian(&unsafe { runner.stack.pop()? }).as_usize();
    let _ = U256::from_big_endian(&unsafe { runner.stack.pop()? }).as_usize();
    let _ = U256::from_big_endian(&unsafe { runner.stack.pop()? }).as_usize();

    let code = [0u8; 32];

    unsafe {runner.stack.push(code)?};

    if runner.debug.is_some() && runner.debug.unwrap() {
        let hex: String = utils::debug::to_hex_string(code);
        println!("{:<14} 👉 [ {} ]", "CODECOPY".bright_blue(), hex);
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn gasprice(runner: &mut Runner) -> Result<(), ExecutionError> {
    let gasprice = [
        [0; 31].to_vec(),
        [0xff].to_vec(),
    ]
    .concat()
    .as_slice()
    .try_into()
    .unwrap();

    let result = unsafe { runner.stack.push(gasprice) };

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug.is_some() && runner.debug.unwrap() {
        let hex: String = utils::debug::to_hex_string(gasprice);
        println!("{:<14} 👉 [ {} ]", "GASPRICE".bright_blue(), hex);
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn extcodesize(runner: &mut Runner) -> Result<(), ExecutionError> {
    let _ = U256::from_big_endian(&unsafe { runner.stack.pop()? }).as_usize();

    let codesize = [0u8; 32];

    let result = unsafe { runner.stack.push(codesize) };

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug.is_some() && runner.debug.unwrap() {
        let hex: String = utils::debug::to_hex_string(codesize);
        println!("{:<14} 👉 [ {} ]", "EXTCODESIZE".bright_blue(), hex);
    }

    // Increment PC
    runner.increment_pc(1)
}

// Can be mocked with a fork
pub fn extcodecopy(runner: &mut Runner) -> Result<(), ExecutionError> {
    let _ = U256::from_big_endian(&unsafe { runner.stack.pop()? }).as_usize();
    let _ = U256::from_big_endian(&unsafe { runner.stack.pop()? }).as_usize();
    let _ = U256::from_big_endian(&unsafe { runner.stack.pop()? }).as_usize();
    let _ = U256::from_big_endian(&unsafe { runner.stack.pop()? }).as_usize();

    let code = [0u8; 32];

    unsafe {runner.stack.push(code)?};

    if runner.debug.is_some() && runner.debug.unwrap() {
        println!("{}", "EXTCODECOPY".bright_blue());
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn returndatasize(runner: &mut Runner) -> Result<(), ExecutionError> {
    let mut size = runner.returndata.msize().to_le_bytes();
    size.reverse();

    // Convert the usize to bytes in little-endian order
    let returndatasize = [[0u8; 24].to_vec(), size.to_vec()].concat().as_slice().try_into().unwrap();

    let result = unsafe { runner.stack.push(returndatasize) };

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug.is_some() && runner.debug.unwrap() {
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

    if runner.debug.is_some() && runner.debug.unwrap() {
        println!("{}", "RETURNDATACOPY".bright_blue());
    }

    // Increment PC
    runner.increment_pc(1)
}

// Can be mocked with a fork
pub fn extcodehash(runner: &mut Runner) -> Result<(), ExecutionError> {
    let _ = U256::from_big_endian(&unsafe { runner.stack.pop()? }).as_usize();

    let code = [0u8; 32];
    let codehash = keccak256(code);

    let result = unsafe { runner.stack.push(codehash) };

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug.is_some() && runner.debug.unwrap() {
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

    if runner.debug.is_some() && runner.debug.unwrap() {
        let hex: String = utils::debug::to_hex_string(blockhash);
        println!("{:<14} 👉 [ {} ]", "BLOCKHASH".bright_blue(), hex);
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn coinbase(runner: &mut Runner) -> Result<(), ExecutionError> {
    let coinbase = [[0; 12].to_vec(), [0xc0u8; 20].to_vec()]
        .concat()
        .as_slice()
        .try_into()
        .unwrap();

    let result = unsafe { runner.stack.push(coinbase) };

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug.is_some() && runner.debug.unwrap() {
        let hex: String = utils::debug::to_hex_string(coinbase);
        println!("{:<14} 👉 [ {} ]", "COINBASE".bright_blue(), hex);
    }

    // Increment PC
    runner.increment_pc(1)
}

// Can be mocked with a fork
pub fn timestamp(runner: &mut Runner) -> Result<(), ExecutionError> {
    // Get the current timestamp
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    // Convert the timestamp to seconds
    let timestamp_secs = since_the_epoch.as_secs();

    // Convert the timestamp to bytes in big-endian order
    let timestamp_bytes = timestamp_secs.to_be_bytes();

    let bytes = [[0; 24].to_vec(), timestamp_bytes.to_vec()].concat().as_slice().try_into().unwrap();

    let result = unsafe { runner.stack.push(bytes) };

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug.is_some() && runner.debug.unwrap() {
        let hex: String = utils::debug::to_hex_string(bytes);
        println!("{:<14} 👉 [ {} ]", "TIMESTAMP".bright_blue(), hex);
    }

    // Increment PC
    runner.increment_pc(1)
}

// Can be mocked with a fork
pub fn number(runner: &mut Runner) -> Result<(), ExecutionError> {
    let number = [[0u8; 28].to_vec(), [0xff; 4].to_vec()].concat().as_slice().try_into().unwrap();

    let result = unsafe { runner.stack.push(number) };

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug.is_some() && runner.debug.unwrap() {
        let hex: String = utils::debug::to_hex_string(number);
        println!("{:<14} 👉 [ {} ]", "NUMBER".bright_blue(), hex);
    }

    // Increment PC
    runner.increment_pc(1)
}

// Can be mocked with a fork
pub fn difficulty(runner: &mut Runner) -> Result<(), ExecutionError> {
    let difficulty = [[0u8; 24].to_vec(), [0x45; 8].to_vec()].concat().as_slice().try_into().unwrap();

    let result = unsafe { runner.stack.push(difficulty) };

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug.is_some() && runner.debug.unwrap() {
        let hex: String = utils::debug::to_hex_string(difficulty);
        println!("{:<14} 👉 [ {} ]", "DIFFICULTY".bright_blue(), hex);
    }

    // Increment PC
    runner.increment_pc(1)
}

// Can be mocked with a fork
pub fn gaslimit(runner: &mut Runner) -> Result<(), ExecutionError> {
    let gaslimit = [[0u8; 28].to_vec(), [0x01, 0xC9, 0xC3, 0x80].to_vec()].concat().as_slice().try_into().unwrap();

    let result = unsafe { runner.stack.push(gaslimit) };

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug.is_some() && runner.debug.unwrap() {
        let hex: String = utils::debug::to_hex_string(gaslimit);
        println!("{:<14} 👉 [ {} ]", "GASLIMIT".bright_blue(), hex);
    }

    // Increment PC
    runner.increment_pc(1)
}

// Can be mocked with a fork
pub fn chainid(runner: &mut Runner) -> Result<(), ExecutionError> {
    let chainid = [[0u8; 31].to_vec(), [0x01].to_vec()].concat().as_slice().try_into().unwrap();

    let result = unsafe { runner.stack.push(chainid) };

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug.is_some() && runner.debug.unwrap() {
        let hex: String = utils::debug::to_hex_string(chainid);
        println!("{:<14} 👉 [ {} ]", "CHAINID".bright_blue(), hex);
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn selfbalance(runner: &mut Runner) -> Result<(), ExecutionError> {
    let selfbalance = [[0u8; 25].to_vec(), [0xbb; 7].to_vec()].concat().as_slice().try_into().unwrap();

    let result = unsafe { runner.stack.push(selfbalance) };

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug.is_some() && runner.debug.unwrap() {
        let hex: String = utils::debug::to_hex_string(selfbalance);
        println!("{:<14} 👉 [ {} ]", "SELFBALANCE".bright_blue(), hex);
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn basefee(runner: &mut Runner) -> Result<(), ExecutionError> {
    let basefee = [[0u8; 31].to_vec(), [0xa].to_vec()].concat().as_slice().try_into().unwrap();

    let result = unsafe { runner.stack.push(basefee) };

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug.is_some() && runner.debug.unwrap() {
        let hex: String = utils::debug::to_hex_string(basefee);
        println!("{:<14} 👉 [ {} ]", "BASEFEE".bright_blue(), hex);
    }

    // Increment PC
    runner.increment_pc(1)
}