use crate::core_module::runner::Runner;
use crate::core_module::utils;
use crate::core_module::utils::errors::ExecutionError;

// Colored output
use colored::*;

/// Duplicate the first stack element
///
/// # Arguments
///
/// * `runner` - A mutable reference to the `Runner`
///
/// # Errors
///
/// returns an `ExecutionError` if:
///
/// * Their is not enough items on the stack
pub fn dup1(runner: &mut Runner) -> Result<(), ExecutionError> {
    let result = runner.stack.dup(1);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let hex: String = utils::debug::to_hex_string(result.unwrap());
        runner.print_debug(&format!(
            "{}{:<11} 👉 [ {} ]",
            "DUP".bright_blue(),
            "1".magenta(),
            hex
        ));
    }

    // Decrement gas
    runner.decrement_gas(3);

    // Increment PC
    runner.increment_pc(1)
}

/// Duplicate the second stack element
///
/// # Arguments
///
/// * `runner` - A mutable reference to the `Runner`
///
/// # Errors
///
/// returns an `ExecutionError` if:
///
/// * Their is not enough items on the stack
pub fn dup2(runner: &mut Runner) -> Result<(), ExecutionError> {
    let result = runner.stack.dup(2);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let hex: String = utils::debug::to_hex_string(result.unwrap());
        runner.print_debug(&format!(
            "{}{:<11} 👉 [ {} ]",
            "DUP".bright_blue(),
            "2".magenta(),
            hex
        ));
    }

    // Decrement gas
    runner.decrement_gas(3);

    // Increment PC
    runner.increment_pc(1)
}

/// Duplicate the third stack element
///
/// # Arguments
///
/// * `runner` - A mutable reference to the `Runner`
///
/// # Errors
///
/// returns an `ExecutionError` if:
///
/// * Their is not enough items on the stack
pub fn dup3(runner: &mut Runner) -> Result<(), ExecutionError> {
    let result = runner.stack.dup(3);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let hex: String = utils::debug::to_hex_string(result.unwrap());
        runner.print_debug(&format!(
            "{}{:<11} 👉 [ {} ]",
            "DUP".bright_blue(),
            "3".magenta(),
            hex
        ));
    }

    // Decrement gas
    runner.decrement_gas(3);

    // Increment PC
    runner.increment_pc(1)
}

/// Duplicate the fourth stack element
///
/// # Arguments
///
/// * `runner` - A mutable reference to the `Runner`
///
/// # Errors
///
/// returns an `ExecutionError` if:
///
/// * Their is not enough items on the stack
pub fn dup4(runner: &mut Runner) -> Result<(), ExecutionError> {
    let result = runner.stack.dup(4);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let hex: String = utils::debug::to_hex_string(result.unwrap());
        runner.print_debug(&format!(
            "{}{:<11} 👉 [ {} ]",
            "DUP".bright_blue(),
            "4".magenta(),
            hex
        ));
    }

    // Decrement gas
    runner.decrement_gas(3);

    // Increment PC
    runner.increment_pc(1)
}

/// Duplicate the fifth stack element
///
/// # Arguments
///
/// * `runner` - A mutable reference to the `Runner`
///
/// # Errors
///
/// returns an `ExecutionError` if:
///
/// * Their is not enough items on the stack
pub fn dup5(runner: &mut Runner) -> Result<(), ExecutionError> {
    let result = runner.stack.dup(5);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let hex: String = utils::debug::to_hex_string(result.unwrap());
        runner.print_debug(&format!(
            "{}{:<11} 👉 [ {} ]",
            "DUP".bright_blue(),
            "5".magenta(),
            hex
        ));
    }

    // Decrement gas
    runner.decrement_gas(3);

    // Increment PC
    runner.increment_pc(1)
}

/// Duplicate the sixth stack element
///
/// # Arguments
///
/// * `runner` - A mutable reference to the `Runner`
///
/// # Errors
///
/// returns an `ExecutionError` if:
///
/// * Their is not enough items on the stack
pub fn dup6(runner: &mut Runner) -> Result<(), ExecutionError> {
    let result = runner.stack.dup(6);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let hex: String = utils::debug::to_hex_string(result.unwrap());
        runner.print_debug(&format!(
            "{}{:<11} 👉 [ {} ]",
            "DUP".bright_blue(),
            "6".magenta(),
            hex
        ));
    }

    // Decrement gas
    runner.decrement_gas(3);

    // Increment PC
    runner.increment_pc(1)
}

/// Duplicate the seventh stack element
///
/// # Arguments
///
/// * `runner` - A mutable reference to the `Runner`
///
/// # Errors
///
/// returns an `ExecutionError` if:
///
/// * Their is not enough items on the stack
pub fn dup7(runner: &mut Runner) -> Result<(), ExecutionError> {
    let result = runner.stack.dup(7);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let hex: String = utils::debug::to_hex_string(result.unwrap());
        runner.print_debug(&format!(
            "{}{:<11} 👉 [ {} ]",
            "DUP".bright_blue(),
            "7".magenta(),
            hex
        ));
    }

    // Decrement gas
    runner.decrement_gas(3);

    // Increment PC
    runner.increment_pc(1)
}

/// Duplicate the eighth stack element
///
/// # Arguments
///
/// * `runner` - A mutable reference to the `Runner`
///
/// # Errors
///
/// returns an `ExecutionError` if:
///
/// * Their is not enough items on the stack
pub fn dup8(runner: &mut Runner) -> Result<(), ExecutionError> {
    let result = runner.stack.dup(8);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let hex: String = utils::debug::to_hex_string(result.unwrap());
        runner.print_debug(&format!(
            "{}{:<11} 👉 [ {} ]",
            "DUP".bright_blue(),
            "8".magenta(),
            hex
        ));
    }

    // Decrement gas
    runner.decrement_gas(3);

    // Increment PC
    runner.increment_pc(1)
}

/// Duplicate the ninth stack element
///
/// # Arguments
///
/// * `runner` - A mutable reference to the `Runner`
///
/// # Errors
///
/// returns an `ExecutionError` if:
///
/// * Their is not enough items on the stack
pub fn dup9(runner: &mut Runner) -> Result<(), ExecutionError> {
    let result = runner.stack.dup(9);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let hex: String = utils::debug::to_hex_string(result.unwrap());
        runner.print_debug(&format!(
            "{}{:<11} 👉 [ {} ]",
            "DUP".bright_blue(),
            "9".magenta(),
            hex
        ));
    }

    // Decrement gas
    runner.decrement_gas(3);

    // Increment PC
    runner.increment_pc(1)
}

/// Duplicate the tenth stack element
///
/// # Arguments
///
/// * `runner` - A mutable reference to the `Runner`
///
/// # Errors
///
/// returns an `ExecutionError` if:
///
/// * Their is not enough items on the stack
pub fn dup10(runner: &mut Runner) -> Result<(), ExecutionError> {
    let result = runner.stack.dup(10);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let hex: String = utils::debug::to_hex_string(result.unwrap());
        runner.print_debug(&format!(
            "{}{:<11} 👉 [ {} ]",
            "DUP".bright_blue(),
            "10".magenta(),
            hex
        ));
    }

    // Decrement gas
    runner.decrement_gas(3);

    // Increment PC
    runner.increment_pc(1)
}

/// Duplicate the eleventh stack element
///
/// # Arguments
///
/// * `runner` - A mutable reference to the `Runner`
///
/// # Errors
///
/// returns an `ExecutionError` if:
///
/// * Their is not enough items on the stack
pub fn dup11(runner: &mut Runner) -> Result<(), ExecutionError> {
    let result = runner.stack.dup(11);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let hex: String = utils::debug::to_hex_string(result.unwrap());
        runner.print_debug(&format!(
            "{}{:<11} 👉 [ {} ]",
            "DUP".bright_blue(),
            "11".magenta(),
            hex
        ));
    }

    // Decrement gas
    runner.decrement_gas(3);

    // Increment PC
    runner.increment_pc(1)
}

/// Duplicate the twelfth stack element
///
/// # Arguments
///
/// * `runner` - A mutable reference to the `Runner`
///
/// # Errors
///
/// returns an `ExecutionError` if:
///
/// * Their is not enough items on the stack
pub fn dup12(runner: &mut Runner) -> Result<(), ExecutionError> {
    let result = runner.stack.dup(12);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let hex: String = utils::debug::to_hex_string(result.unwrap());
        runner.print_debug(&format!(
            "{}{:<11} 👉 [ {} ]",
            "DUP".bright_blue(),
            "12".magenta(),
            hex
        ));
    }

    // Decrement gas
    runner.decrement_gas(3);

    // Increment PC
    runner.increment_pc(1)
}

/// Duplicate the thirteenth stack element
///
/// # Arguments
///
/// * `runner` - A mutable reference to the `Runner`
///
/// # Errors
///
/// returns an `ExecutionError` if:
///
/// * Their is not enough items on the stack
pub fn dup13(runner: &mut Runner) -> Result<(), ExecutionError> {
    let result = runner.stack.dup(13);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let hex: String = utils::debug::to_hex_string(result.unwrap());
        runner.print_debug(&format!(
            "{}{:<11} 👉 [ {} ]",
            "DUP".bright_blue(),
            "13".magenta(),
            hex
        ));
    }

    // Decrement gas
    runner.decrement_gas(3);

    // Increment PC
    runner.increment_pc(1)
}

/// Duplicate the fourteenth stack element
///
/// # Arguments
///
/// * `runner` - A mutable reference to the `Runner`
///
/// # Errors
///
/// returns an `ExecutionError` if:
///
/// * Their is not enough items on the stack
pub fn dup14(runner: &mut Runner) -> Result<(), ExecutionError> {
    let result = runner.stack.dup(14);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let hex: String = utils::debug::to_hex_string(result.unwrap());
        runner.print_debug(&format!(
            "{}{:<11} 👉 [ {} ]",
            "DUP".bright_blue(),
            "14".magenta(),
            hex
        ));
    }

    // Decrement gas
    runner.decrement_gas(3);

    // Increment PC
    runner.increment_pc(1)
}

/// Duplicate the fifteenth stack element
///
/// # Arguments
///
/// * `runner` - A mutable reference to the `Runner`
///
/// # Errors
///
/// returns an `ExecutionError` if:
///
/// * Their is not enough items on the stack
pub fn dup15(runner: &mut Runner) -> Result<(), ExecutionError> {
    let result = runner.stack.dup(15);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let hex: String = utils::debug::to_hex_string(result.unwrap());
        runner.print_debug(&format!(
            "{}{:<11} 👉 [ {} ]",
            "DUP".bright_blue(),
            "15".magenta(),
            hex
        ));
    }

    // Decrement gas
    runner.decrement_gas(3);

    // Increment PC
    runner.increment_pc(1)
}

/// Duplicate the sixteenth stack element
///
/// # Arguments
///
/// * `runner` - A mutable reference to the `Runner`
///
/// # Errors
///
/// returns an `ExecutionError` if:
///
/// * Their is not enough items on the stack
pub fn dup16(runner: &mut Runner) -> Result<(), ExecutionError> {
    let result = runner.stack.dup(16);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let hex: String = utils::debug::to_hex_string(result.unwrap());
        runner.print_debug(&format!(
            "{}{:<11} 👉 [ {} ]",
            "DUP".bright_blue(),
            "16".magenta(),
            hex
        ));
    }

    // Decrement gas
    runner.decrement_gas(3);

    // Increment PC
    runner.increment_pc(1)
}

/* -------------------------------------------------------------------------- */
/*                                    TESTS                                   */
/* -------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dup1() {
        let mut runner = Runner::new([0xaa; 20], None, None, None, None, None);

        let _ = runner.interpret(vec![0x60, 0xff, 0x80], Some(2), true);
        assert_eq!(runner.stack.stack.len(), 2);
        assert_eq!(runner.stack.stack.first(), runner.stack.stack.last());
    }

    #[test]
    fn test_dup2() {
        let mut runner = Runner::new([0xaa; 20], None, None, None, None, None);

        let _ = runner.interpret(vec![0x60, 0xff, 0x60, 0x01, 0x81], Some(2), true);
        assert_eq!(runner.stack.stack.len(), 3);
        assert_eq!(runner.stack.stack.first(), runner.stack.stack.last());
    }

    #[test]
    fn test_dup3() {
        let mut runner = Runner::new([0xaa; 20], None, None, None, None, None);

        let _ = runner.interpret(
            vec![0x60, 0xff, 0x60, 0x01, 0x60, 0x01, 0x82],
            Some(2),
            true,
        );
        assert_eq!(runner.stack.stack.len(), 4);
        assert_eq!(runner.stack.stack.first(), runner.stack.stack.last());
    }

    #[test]
    fn test_dup4() {
        let mut runner = Runner::new([0xaa; 20], None, None, None, None, None);

        let _ = runner.interpret(
            vec![0x60, 0xff, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x83],
            Some(2),
            true,
        );
        assert_eq!(runner.stack.stack.len(), 5);
        assert_eq!(runner.stack.stack.first(), runner.stack.stack.last());
    }

    #[test]
    fn test_dup5() {
        let mut runner = Runner::new([0xaa; 20], None, None, None, None, None);

        let _ = runner.interpret(
            vec![
                0x60, 0xff, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x84,
            ],
            Some(2),
            true,
        );
        assert_eq!(runner.stack.stack.len(), 6);
        assert_eq!(runner.stack.stack.first(), runner.stack.stack.last());
    }

    #[test]
    fn test_dup6() {
        let mut runner = Runner::new([0xaa; 20], None, None, None, None, None);

        let _ = runner.interpret(
            vec![
                0x60, 0xff, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x85,
            ],
            Some(2),
            true,
        );
        assert_eq!(runner.stack.stack.len(), 7);
        assert_eq!(runner.stack.stack.first(), runner.stack.stack.last());
    }

    #[test]
    fn test_dup7() {
        let mut runner = Runner::new([0xaa; 20], None, None, None, None, None);

        let _ = runner.interpret(
            vec![
                0x60, 0xff, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01,
                0x86,
            ],
            Some(2),
            true,
        );
        assert_eq!(runner.stack.stack.len(), 8);
        assert_eq!(runner.stack.stack.first(), runner.stack.stack.last());
    }

    #[test]
    fn test_dup8() {
        let mut runner = Runner::new([0xaa; 20], None, None, None, None, None);

        let _ = runner.interpret(
            vec![
                0x60, 0xff, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01,
                0x60, 0x01, 0x87,
            ],
            Some(2),
            true,
        );
        assert_eq!(runner.stack.stack.len(), 9);
        assert_eq!(runner.stack.stack.first(), runner.stack.stack.last());
    }

    #[test]
    fn test_dup9() {
        let mut runner = Runner::new([0xaa; 20], None, None, None, None, None);

        let _ = runner.interpret(
            vec![
                0x60, 0xff, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01,
                0x60, 0x01, 0x60, 0x01, 0x88,
            ],
            Some(2),
            true,
        );
        assert_eq!(runner.stack.stack.len(), 10);
        assert_eq!(runner.stack.stack.first(), runner.stack.stack.last());
    }

    #[test]
    fn test_dup10() {
        let mut runner = Runner::new([0xaa; 20], None, None, None, None, None);

        let _ = runner.interpret(
            vec![
                0x60, 0xff, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01,
                0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x89,
            ],
            Some(2),
            true,
        );
        assert_eq!(runner.stack.stack.len(), 11);
        assert_eq!(runner.stack.stack.first(), runner.stack.stack.last());
    }

    #[test]
    fn test_dup11() {
        let mut runner = Runner::new([0xaa; 20], None, None, None, None, None);

        let _ = runner.interpret(
            vec![
                0x60, 0xff, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01,
                0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x8a,
            ],
            Some(2),
            true,
        );
        assert_eq!(runner.stack.stack.len(), 12);
        assert_eq!(runner.stack.stack.first(), runner.stack.stack.last());
    }

    #[test]
    fn test_dup12() {
        let mut runner = Runner::new([0xaa; 20], None, None, None, None, None);

        let _ = runner.interpret(
            vec![
                0x60, 0xff, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01,
                0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x8b,
            ],
            Some(2),
            true,
        );
        assert_eq!(runner.stack.stack.len(), 13);
        assert_eq!(runner.stack.stack.first(), runner.stack.stack.last());
    }

    #[test]
    fn test_dup13() {
        let mut runner = Runner::new([0xaa; 20], None, None, None, None, None);

        let _ = runner.interpret(
            vec![
                0x60, 0xff, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01,
                0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x8c,
            ],
            Some(2),
            true,
        );
        assert_eq!(runner.stack.stack.len(), 14);
        assert_eq!(runner.stack.stack.first(), runner.stack.stack.last());
    }

    #[test]
    fn test_dup14() {
        let mut runner = Runner::new([0xaa; 20], None, None, None, None, None);

        let _ = runner.interpret(
            vec![
                0x60, 0xff, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01,
                0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01,
                0x8d,
            ],
            Some(2),
            true,
        );
        assert_eq!(runner.stack.stack.len(), 15);
        assert_eq!(runner.stack.stack.first(), runner.stack.stack.last());
    }

    #[test]
    fn test_dup15() {
        let mut runner = Runner::new([0xaa; 20], None, None, None, None, None);

        let _ = runner.interpret(
            vec![
                0x60, 0xff, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01,
                0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01,
                0x60, 0x01, 0x8e,
            ],
            Some(2),
            true,
        );
        assert_eq!(runner.stack.stack.len(), 16);
        assert_eq!(runner.stack.stack.first(), runner.stack.stack.last());
    }

    #[test]
    fn test_dup16() {
        let mut runner = Runner::new([0xaa; 20], None, None, None, None, None);

        let _ = runner.interpret(
            vec![
                0x60, 0xff, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01,
                0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01,
                0x60, 0x01, 0x60, 0x01, 0x8f,
            ],
            Some(2),
            true,
        );
        assert_eq!(runner.stack.stack.len(), 17);
        assert_eq!(runner.stack.stack.first(), runner.stack.stack.last());
    }
}
