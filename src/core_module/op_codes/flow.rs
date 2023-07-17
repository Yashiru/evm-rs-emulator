use crate::core_module::runner::Runner;
use crate::core_module::utils;
use crate::core_module::utils::bytes::pad_left;
use crate::core_module::utils::errors::ExecutionError;

// Primitive types
use ethers::types::U256;

// Colored output
use colored::*;

// Stop execution
pub fn stop(runner: &mut Runner) -> Result<(), ExecutionError> {
    // Set the program counter to the end of the bytecode
    runner.set_pc(runner.bytecode.len());

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        runner.print_debug(&format!(
            "{:<14} {}",
            "STOP".bright_blue(),
            "🛑 STOP 🛑".red()
        ));
    }

    Ok(())
}

// Revert data from memory heap
pub fn revert(runner: &mut Runner) -> Result<(), ExecutionError> {
    let offset = U256::from_big_endian(&unsafe { runner.stack.pop()? });
    let size = U256::from_big_endian(&unsafe { runner.stack.pop()? });

    let revert_data = unsafe { runner.memory.read(offset.as_usize(), size.as_usize()) };

    // Copy revert data to the returndata
    runner.returndata.heap = revert_data.as_ref().unwrap().to_owned();

    let err;
    let hex;

    if revert_data.is_ok() && revert_data.as_ref().unwrap().len() > 0 {
        hex = utils::debug::vec_to_hex_string(
            revert_data.as_ref().unwrap().as_slice().try_into().unwrap(),
        );
        err = ExecutionError::Revert(revert_data.unwrap());
    } else {
        hex = utils::debug::to_hex_string([0u8; 32]);
        err = ExecutionError::RevertWithoutData;
    }

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        runner.print_debug(&format!("\n{:<14} 💥 [ {} ]", "REVERT".red(), hex));
    }

    Err(err)
}

// jump
pub fn jump(runner: &mut Runner) -> Result<(), ExecutionError> {
    let mut bytes = [0u8; 32];
    let jump_address = U256::from_big_endian(&unsafe { runner.stack.pop()? });
    jump_address.to_big_endian(&mut bytes);

    // Check if the address is out of bounds
    if jump_address.as_usize() > runner.bytecode.len() {
        return Err(ExecutionError::OutOfBoundsByteCode);
    }

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let hex = utils::debug::to_hex_string(bytes);
        runner.print_debug(&format!("{:<14} 〰️ [ {} ]", "JUMP".bright_green(), hex));
    }

    // Set the program counter to the jump address
    runner.set_pc(jump_address.as_usize());
    Ok(())
}

// jumpi
pub fn jumpi(runner: &mut Runner) -> Result<(), ExecutionError> {
    let mut bytes = [0u8; 32];
    let jump_address = U256::from_big_endian(&unsafe { runner.stack.pop()? });
    jump_address.to_big_endian(&mut bytes);

    let condition = U256::from_big_endian(&unsafe { runner.stack.pop()? });

    // Check if the address is out of bounds
    if jump_address.as_usize() > runner.bytecode.len() {
        return Err(ExecutionError::OutOfBoundsByteCode);
    }

    // Check if the condition is true
    if !condition.is_zero() {
        // Set the program counter to the jump address
        runner.set_pc(jump_address.as_usize());
    } else {
        // Increment the program counter
        let _ = runner.increment_pc(1);
    }

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let hex = utils::debug::to_hex_string(bytes);

        runner.print_debug(&format!(
            "{:<14} 〰️ [ {} ]",
            if condition.is_zero() {
                "JUMPI".green()
            } else {
                "JUMPI".bright_red()
            },
            hex
        ));
    }

    Ok(())
}

// pc
pub fn pc(runner: &mut Runner) -> Result<(), ExecutionError> {
    let pc = runner.get_pc().to_be_bytes();
    let pc = pad_left(&pc.to_vec());

    // Push the program counter to the stack
    unsafe { runner.stack.push(pc)? };

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let hex = utils::debug::to_hex_string(pc);
        runner.print_debug(&format!("{:<14} 👉 [ {} ]", "PC".bright_blue(), hex));
    }
    
    // Increment the program counter
    runner.increment_pc(1)
}

// gas
pub fn gas(runner: &mut Runner) -> Result<(), ExecutionError> {
    let gas = runner.gas.to_be_bytes();
    let gas = pad_left(&gas.to_vec());

    // Push the gas to the stack
    unsafe { runner.stack.push(gas)? };

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let hex = utils::debug::to_hex_string(gas);
        runner.print_debug(&format!("{:<14} 👉 [ {} ]", "GAS".bright_blue(), hex));
    }
    // Increment the program counter
    runner.increment_pc(1)
}

// jumpdest
pub fn jumpdest(runner: &mut Runner) -> Result<(), ExecutionError> {
    // Increment the program counter
    runner.increment_pc(1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core_module::utils::bytes::{_hex_string_to_bytes, pad_left};

    #[test]
    fn test_stop() {
        let mut runner = Runner::_default(3);
        let interpret_result = runner.interpret(
            _hex_string_to_bytes("600160026003600400600560066007"),
            Some(2),
            true,
        );
        assert!(interpret_result.is_ok());

        let result: [u8; 32] = unsafe { runner.stack.pop().unwrap() };
        assert_eq!(result, pad_left(&[0x04]));
        assert_eq!(runner.pc, 15);
    }

    #[test]
    fn test_revert() {
        let mut runner = Runner::_default(3);
        let interpret_result = runner.interpret(_hex_string_to_bytes("7fff0100000000000000000000000000000000000000000000000000000000000060005260026000fd"), Some(2), true);

        assert!(interpret_result.is_err());
        assert_eq!(runner.returndata.heap, vec![0xff, 0x01]);
    }

    #[test]
    fn test_jump() {
        let mut runner = Runner::_default(3);
        let interpret_result =
            runner.interpret(_hex_string_to_bytes("600456fe5b6001"), Some(2), true);
        assert!(interpret_result.is_ok());

        let result: [u8; 32] = unsafe { runner.stack.pop().unwrap() };
        assert_eq!(result, pad_left(&[0x01]));
        assert_eq!(runner.pc, 7);
    }

    #[test]
    fn test_jumpi() {
        let mut runner = Runner::_default(3);
        let interpret_result = runner.interpret(
            _hex_string_to_bytes("6000600a576001600c575bfe5b6001"),
            Some(2),
            true,
        );
        assert!(interpret_result.is_ok());

        let result: [u8; 32] = unsafe { runner.stack.pop().unwrap() };
        assert_eq!(result, pad_left(&[0x01]));
        assert_eq!(runner.pc, 15);
    }

    #[test]
    fn test_pc() {
        let mut runner = Runner::_default(3);
        let interpret_result = runner.interpret(_hex_string_to_bytes("58"), Some(2), true);
        assert!(interpret_result.is_ok());

        let result: [u8; 32] = unsafe { runner.stack.pop().unwrap() };
        assert_eq!(result, pad_left(&[0x00]));
        assert_eq!(runner.pc, 1);

        let mut runner = Runner::_default(3);
        let interpret_result = runner.interpret(
            _hex_string_to_bytes("60ff60ff60ff60ff60ff58"),
            Some(2),
            true,
        );
        assert!(interpret_result.is_ok());

        let result: [u8; 32] = unsafe { runner.stack.pop().unwrap() };
        assert_eq!(result, pad_left(&[0x0a]));
        assert_eq!(runner.pc, 11);
    }

    #[test]
    fn test_gas() {
        let mut runner = Runner::_default(3);
        let interpret_result = runner.interpret(_hex_string_to_bytes("5a"), Some(2), true);
        assert!(interpret_result.is_ok());

        let result: [u8; 32] = unsafe { runner.stack.pop().unwrap() };
        assert_eq!(result, pad_left(&runner.gas.to_be_bytes()));
    }
}
