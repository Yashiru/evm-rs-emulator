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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core_module::{op_codes::memory::mstore, utils::bytes32::pad_to_32_bytes};

    #[test]
    fn test_not() {
        let mut runner = Runner::default();
        let _ = unsafe { runner.stack.push(pad_to_32_bytes(&[0x04])) };

        not(&mut runner).unwrap();

        let result = unsafe { runner.stack.pop() }.unwrap();
        let expected_result = pad_to_32_bytes(&[
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xfb,
        ]);

        assert_eq!(result, expected_result);
        assert_eq!(runner.stack.stack.len(), 0);
    }

    #[test]
    fn test_xor() {
        let mut runner = Runner::default();
        let _ = unsafe { runner.stack.push(pad_to_32_bytes(&[0x04])) };
        let _ = unsafe { runner.stack.push(pad_to_32_bytes(&[0x08])) };

        xor(&mut runner).unwrap();

        let result = unsafe { runner.stack.pop() }.unwrap();
        let expected_result = pad_to_32_bytes(&[0x04 ^ 0x08]);

        assert_eq!(result, expected_result);
        assert_eq!(runner.stack.stack.len(), 0);
    }

    #[test]
    fn or_test() {
        let mut runner = Runner::default();
        let _ = unsafe { runner.stack.push(pad_to_32_bytes(&[0x04])) };
        let _ = unsafe { runner.stack.push(pad_to_32_bytes(&[0x08])) };

        or(&mut runner).unwrap();

        let result = unsafe { runner.stack.pop() }.unwrap();
        let expected_result = pad_to_32_bytes(&[0x04 | 0x08]);

        assert_eq!(result, expected_result);
        assert_eq!(runner.stack.stack.len(), 0);
    }

    #[test]
    fn test_and() {
        let mut runner = Runner::default();
        let _ = unsafe { runner.stack.push(pad_to_32_bytes(&[0x04])) };
        let _ = unsafe { runner.stack.push(pad_to_32_bytes(&[0x08])) };

        and(&mut runner).unwrap();

        let result = unsafe { runner.stack.pop() }.unwrap();
        let expected_result = pad_to_32_bytes(&[0x04 & 0x08]);

        assert_eq!(result, expected_result);
        assert_eq!(runner.stack.stack.len(), 0);
    }

    #[test]
    fn test_shl() {
        let mut runner = Runner::default();
        let _ = unsafe { runner.stack.push(pad_to_32_bytes(&[0x04])) };
        let _ = unsafe { runner.stack.push(pad_to_32_bytes(&[0x02])) };

        shl(&mut runner).unwrap();

        let result = unsafe { runner.stack.pop() }.unwrap();
        let expected_result = pad_to_32_bytes(&[0x10]);

        assert_eq!(result, expected_result);
        assert_eq!(runner.stack.stack.len(), 0);
    }

    #[test]
    fn test_shr() {
        let mut runner = Runner::default();
        let _ = unsafe { runner.stack.push(pad_to_32_bytes(&[0x04])) };
        let _ = unsafe { runner.stack.push(pad_to_32_bytes(&[0x02])) };

        shr(&mut runner).unwrap();

        let result = unsafe { runner.stack.pop() }.unwrap();
        let expected_result = pad_to_32_bytes(&[0x01]);

        assert_eq!(result, expected_result);
        assert_eq!(runner.stack.stack.len(), 0);
    }

    #[test]
    fn test_sha256() {
        let mut runner = Runner::default();

        let _ = unsafe {
            runner
                .stack
                .push(pad_to_32_bytes(&[0xff, 0xff, 0xff, 0xff]))
        };
        let _ = unsafe { runner.stack.push(pad_to_32_bytes(&[0x00])) };
        mstore(&mut runner).unwrap();
        let _ = unsafe { runner.stack.push(pad_to_32_bytes(&[0x04])) };
        let _ = unsafe { runner.stack.push(pad_to_32_bytes(&[0x00])) };
        sha(&mut runner).unwrap();

        let result = unsafe { runner.stack.pop() }.unwrap();
        let expected_output = [
            0xe8, 0xe7, 0x76, 0x26, 0x58, 0x6f, 0x73, 0xb9, 0x55, 0x36, 0x4c, 0x7b, 0x4b, 0xbf,
            0x0b, 0xb7, 0xf7, 0x68, 0x5e, 0xbd, 0x40, 0xe8, 0x52, 0xb1, 0x64, 0x63, 0x3a, 0x4a,
            0xcb, 0xd3, 0x24, 0x4c,
        ];

        assert_eq!(result, expected_output);
        assert_eq!(runner.stack.stack.len(), 0);
    }
}
