use crate::core_module::runner::Runner;
use crate::core_module::utils;
use crate::core_module::utils::errors::ExecutionError;

// Primitive types
use ethers::types::{I256, U256};

// Colored output
use colored::*;

pub fn iszero(runner: &mut Runner) -> Result<(), ExecutionError> {
    let pop1 = runner.stack.pop()?;

    let a = U256::from_big_endian(&pop1);

    let bool = a.is_zero();

    let result_bytes = [
        [0u8; 31].to_vec(),
        [if bool { 1u8 } else { 0u8 }; 1].to_vec(),
    ]
    .concat()
    .as_slice()
    .try_into()
    .unwrap();

    let result = runner.stack.push(result_bytes);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let hex: String = utils::debug::to_hex_string(result_bytes);
        runner.print_debug(&format!("{:<14} 👉 [ {} ]", "ISZERO".bright_blue(), hex));
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn eq(runner: &mut Runner) -> Result<(), ExecutionError> {
    let pop1 = runner.stack.pop()?;
    let pop2 = runner.stack.pop()?;

    let a = U256::from_big_endian(&pop1);
    let b = U256::from_big_endian(&pop2);

    let bool = a.eq(&b);

    let result_bytes = [
        [0u8; 31].to_vec(),
        [if bool { 1u8 } else { 0u8 }; 1].to_vec(),
    ]
    .concat()
    .as_slice()
    .try_into()
    .unwrap();

    let result = runner.stack.push(result_bytes);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let hex: String = utils::debug::to_hex_string(result_bytes);
        runner.print_debug(&format!("{:<14} 👉 [ {} ]", "EQ".bright_blue(), hex));
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn lt(runner: &mut Runner) -> Result<(), ExecutionError> {
    let pop1 = runner.stack.pop()?;
    let pop2 = runner.stack.pop()?;

    let a = U256::from_big_endian(&pop1);
    let b = U256::from_big_endian(&pop2);

    let bool = a.lt(&b);

    let result_bytes = [
        [0u8; 31].to_vec(),
        [if bool { 1u8 } else { 0u8 }; 1].to_vec(),
    ]
    .concat()
    .as_slice()
    .try_into()
    .expect("Wrong length");

    let result = runner.stack.push(result_bytes);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let hex: String = utils::debug::to_hex_string(result_bytes);
        runner.print_debug(&format!("{:<14} 👉 [ {} ]", "LT".bright_blue(), hex));
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn gt(runner: &mut Runner) -> Result<(), ExecutionError> {
    let pop1 = runner.stack.pop()?;
    let pop2 = runner.stack.pop()?;

    let a = U256::from_big_endian(&pop1);
    let b = U256::from_big_endian(&pop2);

    let bool = a.gt(&b);

    let result_bytes = [
        [0u8; 31].to_vec(),
        [if bool { 1u8 } else { 0u8 }; 1].to_vec(),
    ]
    .concat()
    .as_slice()
    .try_into()
    .unwrap();

    let result = runner.stack.push(result_bytes);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let hex: String = utils::debug::to_hex_string(result_bytes);
        runner.print_debug(&format!("{:<14} 👉 [ {} ]", "GT".bright_blue(), hex));
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn slt(runner: &mut Runner) -> Result<(), ExecutionError> {
    let pop1 = runner.stack.pop()?;
    let pop2 = runner.stack.pop()?;

    let a = I256::from_raw(U256::from_big_endian(&pop1));
    let b = I256::from_raw(U256::from_big_endian(&pop2));

    let bool = a.lt(&b);

    let result_bytes = [
        [0u8; 31].to_vec(),
        [if bool { 1u8 } else { 0u8 }; 1].to_vec(),
    ]
    .concat()
    .as_slice()
    .try_into()
    .unwrap();

    let result = runner.stack.push(result_bytes);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let hex: String = utils::debug::to_hex_string(result_bytes);
        runner.print_debug(&format!("{:<14} 👉 [ {} ]", "SLT".bright_blue(), hex));
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn sgt(runner: &mut Runner) -> Result<(), ExecutionError> {
    let pop1 = runner.stack.pop()?;
    let pop2 = runner.stack.pop()?;

    let a = I256::from_raw(U256::from_big_endian(&pop1));
    let b = I256::from_raw(U256::from_big_endian(&pop2));

    let bool = a.gt(&b);

    let result_bytes = [
        [0u8; 31].to_vec(),
        [if bool { 1u8 } else { 0u8 }; 1].to_vec(),
    ]
    .concat()
    .as_slice()
    .try_into()
    .unwrap();

    let result = runner.stack.push(result_bytes);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let hex: String = utils::debug::to_hex_string(result_bytes);
        runner.print_debug(&format!("{:<14} 👉 [ {} ]", "SGT".bright_blue(), hex));
    }

    // Increment PC
    runner.increment_pc(1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core_module::utils::bytes::pad_left;
    #[test]
    fn iszero_test() {
        let mut runner = Runner::_default(3);
        let _ = runner.stack.push(pad_left(&[0x00]));

        iszero(&mut runner).unwrap();

        let result = runner.stack.pop().unwrap();
        let expected_result = pad_left(&[1]);

        assert_eq!(result, expected_result);
        assert_eq!(runner.stack.stack.len(), 0);

        let mut runner = Runner::_default(3);
        let _ = runner.stack.push(pad_left(&[0x01]));

        iszero(&mut runner).unwrap();

        let result = runner.stack.pop().unwrap();
        let expected_result = pad_left(&[0]);

        assert_eq!(result, expected_result);
        assert_eq!(runner.stack.stack.len(), 0);
    }

    #[test]
    fn eq_test() {
        let mut runner = Runner::_default(3);
        let _ = runner.stack.push(pad_left(&[0x04]));
        let _ = runner.stack.push(pad_left(&[0x04]));

        eq(&mut runner).unwrap();

        let result = runner.stack.pop().unwrap();
        let expected_result = pad_left(&[1]);

        assert_eq!(result, expected_result);
        assert_eq!(runner.stack.stack.len(), 0);

        let mut runner = Runner::_default(3);
        let _ = runner.stack.push(pad_left(&[0x04]));
        let _ = runner.stack.push(pad_left(&[0x05]));

        eq(&mut runner).unwrap();

        let result = runner.stack.pop().unwrap();
        let expected_result = pad_left(&[0]);

        assert_eq!(result, expected_result);
        assert_eq!(runner.stack.stack.len(), 0);
    }

    #[test]
    fn lt_test() {
        let mut runner = Runner::_default(3);
        let _ = runner.stack.push(pad_left(&[0x08]));
        let _ = runner.stack.push(pad_left(&[0x04]));

        lt(&mut runner).unwrap();

        let result = runner.stack.pop().unwrap();
        let expected_result = pad_left(&[1]);

        assert_eq!(result, expected_result);
        assert_eq!(runner.stack.stack.len(), 0);
        let mut runner = Runner::_default(3);

        let _ = runner.stack.push(pad_left(&[0x04]));
        let _ = runner.stack.push(pad_left(&[0x08]));

        lt(&mut runner).unwrap();

        let result = runner.stack.pop().unwrap();
        let expected_result = pad_left(&[0]);

        assert_eq!(result, expected_result);
        assert_eq!(runner.stack.stack.len(), 0);
    }

    #[test]
    fn gt_test() {
        let mut runner = Runner::_default(3);
        let _ = runner.stack.push(pad_left(&[0x04]));
        let _ = runner.stack.push(pad_left(&[0x08]));

        gt(&mut runner).unwrap();

        let result = runner.stack.pop().unwrap();
        let expected_result = pad_left(&[1]);

        assert_eq!(result, expected_result);
        assert_eq!(runner.stack.stack.len(), 0);

        let mut runner = Runner::_default(3);
        let _ = runner.stack.push(pad_left(&[0x08]));
        let _ = runner.stack.push(pad_left(&[0x04]));

        gt(&mut runner).unwrap();

        let result = runner.stack.pop().unwrap();
        let expected_result = pad_left(&[0]);

        assert_eq!(result, expected_result);
        assert_eq!(runner.stack.stack.len(), 0);
    }

    #[test]
    fn slt_test() {
        let mut runner = Runner::_default(3);
        let _ = runner.stack.push(pad_left(&[0x09]));
        let _ = runner.stack.push([
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff,
        ]);

        slt(&mut runner).unwrap();

        let result = runner.stack.pop().unwrap();
        let expected_result = pad_left(&[1]);

        assert_eq!(result, expected_result);
        assert_eq!(runner.stack.stack.len(), 0);

        let mut runner = Runner::_default(3);
        let _ = runner.stack.push([
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff,
        ]);
        let _ = runner.stack.push(pad_left(&[0x09]));

        slt(&mut runner).unwrap();

        let result = runner.stack.pop().unwrap();
        let expected_result = pad_left(&[0]);

        assert_eq!(result, expected_result);
        assert_eq!(runner.stack.stack.len(), 0);
    }

    #[test]
    fn sgt_test() {
        let mut runner = Runner::_default(3);
        let _ = runner.stack.push(pad_left(&[0x09]));
        let _ = runner.stack.push([
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff,
        ]);

        sgt(&mut runner).unwrap();

        let result = runner.stack.pop().unwrap();
        let expected_result = pad_left(&[0]);

        assert_eq!(result, expected_result);
        assert_eq!(runner.stack.stack.len(), 0);

        let mut runner = Runner::_default(3);
        let _ = runner.stack.push([
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff,
        ]);
        let _ = runner.stack.push(pad_left(&[0x09]));

        sgt(&mut runner).unwrap();

        let result = runner.stack.pop().unwrap();
        let expected_result = pad_left(&[1]);

        assert_eq!(result, expected_result);
        assert_eq!(runner.stack.stack.len(), 0);
    }
}
