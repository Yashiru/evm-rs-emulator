
use crate::core_module::utils::errors::ExecutionError;
use crate::core_module::runner::Runner;
use crate::core_module::utils;

// Primitive types
use ethers::types::I256;
use ethers::types::U256;

// Colored output
use colored::*;

/// Divides the second item on the stack by the topmost item, treating both values as signed.
/// If the divisor is zero, the result is zero.
///
/// # Example
///
/// ```
/// use bytecode_rs::core_module::op_codes::math::unsigned::sdiv;
/// use bytecode_rs::core_module::runner::Runner;
///
/// let mut runner = Runner::new([0xaa; 20], None, None, None);
/// runner.stack.push([0x01, 0x00, 0x00, 0x00].to_vec()).unwrap();
/// runner.stack.push([0x00, 0x00, 0x00, 0x00].to_vec()).unwrap();
///
/// sdiv(&mut runner).unwrap();
///
/// assert_eq!(runner.stack.pop().unwrap(), [0x00, 0x00, 0x00, 0x00].to_vec());
/// ```
pub fn sdiv(runner: &mut Runner) -> Result<(), ExecutionError> {
    unsafe {
        let pop1 = runner.stack.pop()?;
        let pop2 = runner.stack.pop()?;

        let a = I256::from_raw(U256::from_big_endian(&pop1));
        let b = I256::from_raw(U256::from_big_endian(&pop2));

        let result = a.checked_div(b);

        let mut result_bytes = [0u8; 32];
        result
            .unwrap_or(I256::from(0))
            .to_big_endian(&mut result_bytes);

        let result = runner.stack.push(result_bytes);

        if result.is_err() {
            return Err(result.unwrap_err());
        }

        if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
            let hex: String = utils::debug::to_hex_string(result_bytes);
            println!(
                "{:<14} 👉 [ {} ]",
                "SDIV".bright_blue(),
                hex
            );
        }
    }

    // Increment PC
    runner.increment_pc(1)
}

/// Computes the remainder of the signed division of the two topmost items on the stack.
/// If the divisor is zero, the result is zero.
///
/// # Example
///
/// ```
/// use bytecode_rs::core_module::op_codes::math::unsigned::smodulo;
/// use bytecode_rs::core_module::runner::Runner;
///
/// let mut runner = Runner::new([0xaa; 20], None, None, None);
/// runner.stack.push([0x01, 0x00, 0x00, 0x00].to_vec()).unwrap();
/// runner.stack.push([0x00, 0x00, 0x00, 0x00].to_vec()).unwrap();
///
/// smodulo(&mut runner).unwrap();
///
/// assert_eq!(runner.stack.pop().unwrap(), [0x00, 0x00, 0x00, 0x00].to_vec());
/// ```
pub fn smodulo(runner: &mut Runner) -> Result<(), ExecutionError> {
    unsafe {
        let pop1 = runner.stack.pop()?;
        let pop2 = runner.stack.pop()?;

        let a = I256::from_raw(U256::from_big_endian(&pop1));
        let b = I256::from_raw(U256::from_big_endian(&pop2));

        let result = a.checked_rem(b);

        let mut result_bytes = [0u8; 32];
        result
            .unwrap_or(I256::from(0))
            .to_big_endian(&mut result_bytes);

        let result = runner.stack.push(result_bytes);


        if result.is_err() {
            return Err(result.unwrap_err());
        }

        if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
            let hex: String = utils::debug::to_hex_string(result_bytes);
            println!(
                "{:<14} 👉 [ {} ]",
                "SMOD".bright_blue(),
                hex
            );
        }
    }

    // Increment PC
    runner.increment_pc(1)
}
