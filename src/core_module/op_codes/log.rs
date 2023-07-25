use crate::core_module::runner::Runner;
use crate::core_module::state::Log;
use crate::core_module::utils;
use crate::core_module::utils::errors::ExecutionError;

// Primitive types
use ethers::types::U256;

// Colored output
use colored::*;

/// Executes the LOG0 opcode, which logs a message with no topics.
/// 
/// # Arguments
/// 
/// * `runner` - A mutable reference to the `Runner` struct.
/// 
/// # Errors
/// 
/// Returns an `ExecutionError` if static mode is enabled.
pub fn log0(runner: &mut Runner) -> Result<(), ExecutionError> {
    // Check if static mode is enabled
    if runner.state.static_mode {
        return Err(ExecutionError::StaticCallStateChanged);
    }
    let offset = U256::from_big_endian(&runner.stack.pop()?);
    let size = U256::from_big_endian(&runner.stack.pop()?);

    let log_data = unsafe { runner.memory.read(offset.as_usize(), size.as_usize())? };

    let log = Log {
        address: runner.address,
        topics: vec![],
        data: log_data.clone(),
    };

    runner.state.logs.push(log);

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let hex = utils::debug::vec_to_hex_string(log_data);

        runner.print_debug(&format!("{}", "┌───────────────────────────────────────────────────────────────────────────────────────────────────────────────────┐".cyan()));
        runner.print_debug(&format!(
            "{} 📝 {:<110} {}\n{}{:<115}{}",
            "│".cyan(),
            "LOG0".cyan(),
            "│".cyan(),
            "│".cyan(),
            "",
            "│".cyan()
        ));
        runner.print_debug(&format!(
            "{} {}: {}{:<12} {}",
            "│".cyan(),
            "Data".bright_magenta(),
            hex,
            "",
            "│".cyan()
        ));
        runner.print_debug(&format!("{}", "└───────────────────────────────────────────────────────────────────────────────────────────────────────────────────┘\n".cyan()));
    }

    // Increment PC
    runner.increment_pc(1)
}

/// Executes the LOG1 opcode, which logs a single 32-byte word of data and a single topic.
/// 
/// # Arguments
/// 
/// * `runner` - A mutable reference to the `Runner` struct.
/// 
/// # Errors
/// 
/// Returns an `ExecutionError` if static mode is enabled.
/// 
/// # Panics
/// 
/// This function panics if the `offset` or `size` values are not valid `U256` values.
pub fn log1(runner: &mut Runner) -> Result<(), ExecutionError> {
    // Check if static mode is enabled
    if runner.state.static_mode {
        return Err(ExecutionError::StaticCallStateChanged);
    }
    let offset = U256::from_big_endian(&runner.stack.pop()?);
    let size: U256 = U256::from_big_endian(&runner.stack.pop()?);

    let raw_topic1: U256 = U256::from_big_endian(&runner.stack.pop()?);
    let mut topic1 = [0u8; 32];
    raw_topic1.to_big_endian(&mut topic1);

    let log_data = unsafe { runner.memory.read(offset.as_usize(), size.as_usize())? };

    let log = Log {
        address: runner.address,
        topics: vec![topic1],
        data: log_data.clone(),
    };

    runner.state.logs.push(log);

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let data_hex = utils::debug::vec_to_hex_string(log_data);
        let topic1_hex = utils::debug::to_hex_string(topic1);

        runner.print_debug(&format!("{}", "┌───────────────────────────────────────────────────────────────────────────────────────────────────────────────────┐".cyan()));
        runner.print_debug(&format!(
            "{} 📝 {:<110} {}\n{}{:<115}{}",
            "│".cyan(),
            "LOG1".cyan(),
            "│".cyan(),
            "│".cyan(),
            "",
            "│".cyan()
        ));
        runner.print_debug(&format!(
            "{} {}: {}{:<12} {}",
            "│".cyan(),
            "Data".bright_magenta(),
            data_hex,
            "",
            "│".cyan()
        ));
        runner.print_debug(&format!(
            "{} {:<4}: {}{:<12} {}",
            "│".cyan(),
            "T1".bright_magenta(),
            topic1_hex,
            "",
            "│".cyan()
        ));
        runner.print_debug(&format!("{}", "└───────────────────────────────────────────────────────────────────────────────────────────────────────────────────┘\n".cyan()));
    }

    // Increment PC
    runner.increment_pc(1)
}

/// Executes the LOG2 opcode, which logs a message with two topics to the Ethereum event log.
/// 
/// # Arguments
/// 
/// * `runner` - A mutable reference to the `Runner` struct.
/// 
/// # Errors
/// 
/// Returns an `ExecutionError` if static mode is enabled.
/// 
/// # Panics
/// 
/// Panics if the stack is empty or if there is an error reading from memory.
pub fn log2(runner: &mut Runner) -> Result<(), ExecutionError> {
    // Check if static mode is enabled
    if runner.state.static_mode {
        return Err(ExecutionError::StaticCallStateChanged);
    }
    let offset = U256::from_big_endian(&runner.stack.pop()?);
    let size: U256 = U256::from_big_endian(&runner.stack.pop()?);

    let raw_topic1: U256 = U256::from_big_endian(&runner.stack.pop()?);
    let mut topic1 = [0u8; 32];
    raw_topic1.to_big_endian(&mut topic1);

    let raw_topic2: U256 = U256::from_big_endian(&runner.stack.pop()?);
    let mut topic2 = [0u8; 32];
    raw_topic2.to_big_endian(&mut topic2);

    let log_data = unsafe { runner.memory.read(offset.as_usize(), size.as_usize())? };

    let log = Log {
        address: runner.address,
        topics: vec![topic1, topic2],
        data: log_data.clone(),
    };

    runner.state.logs.push(log);

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let data_hex = utils::debug::vec_to_hex_string(log_data);
        let topic1_hex = utils::debug::to_hex_string(topic1);
        let topic2_hex = utils::debug::to_hex_string(topic2);

        runner.print_debug(&format!("{}", "┌───────────────────────────────────────────────────────────────────────────────────────────────────────────────────┐".cyan()));
        runner.print_debug(&format!(
            "{} 📝 {:<110} {}\n{}{:<115}{}",
            "│".cyan(),
            "LOG2".cyan(),
            "│".cyan(),
            "│".cyan(),
            "",
            "│".cyan()
        ));
        runner.print_debug(&format!(
            "{} {}: {}{:<12} {}",
            "│".cyan(),
            "Data".bright_magenta(),
            data_hex,
            "",
            "│".cyan()
        ));
        runner.print_debug(&format!(
            "{} {:<4}: {}{:<12} {}",
            "│".cyan(),
            "T1".bright_magenta(),
            topic1_hex,
            "",
            "│".cyan()
        ));
        runner.print_debug(&format!(
            "{} {:<4}: {}{:<12} {}",
            "│".cyan(),
            "T2".bright_magenta(),
            topic2_hex,
            "",
            "│".cyan()
        ));
        runner.print_debug(&format!("{}", "└───────────────────────────────────────────────────────────────────────────────────────────────────────────────────┘\n".cyan()));
    }

    // Increment PC
    runner.increment_pc(1)
}

/// Executes the LOG3 operation, which logs a message with three topics.
/// 
/// # Arguments
/// 
/// * `runner` - A mutable reference to the `Runner` struct.
/// 
/// # Errors
/// 
/// Returns an `ExecutionError` if static mode is enabled.
/// 
/// # Panics
/// 
/// This function panics if the `offset` or `size` values are invalid.
pub fn log3(runner: &mut Runner) -> Result<(), ExecutionError> {
    // Check if static mode is enabled
    if runner.state.static_mode {
        return Err(ExecutionError::StaticCallStateChanged);
    }
    let offset = U256::from_big_endian(&runner.stack.pop()?);
    let size: U256 = U256::from_big_endian(&runner.stack.pop()?);

    let raw_topic1: U256 = U256::from_big_endian(&runner.stack.pop()?);
    let mut topic1 = [0u8; 32];
    raw_topic1.to_big_endian(&mut topic1);

    let raw_topic2: U256 = U256::from_big_endian(&runner.stack.pop()?);
    let mut topic2 = [0u8; 32];
    raw_topic2.to_big_endian(&mut topic2);

    let raw_topic3: U256 = U256::from_big_endian(&runner.stack.pop()?);
    let mut topic3 = [0u8; 32];
    raw_topic3.to_big_endian(&mut topic3);

    let log_data = unsafe { runner.memory.read(offset.as_usize(), size.as_usize())? };

    let log = Log {
        address: runner.address,
        topics: vec![topic1, topic2, topic3],
        data: log_data.clone(),
    };

    runner.state.logs.push(log);

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let data_hex = utils::debug::vec_to_hex_string(log_data);
        let topic1_hex = utils::debug::to_hex_string(topic1);
        let topic2_hex = utils::debug::to_hex_string(topic2);
        let topic3_hex = utils::debug::to_hex_string(topic3);

        runner.print_debug(&format!("{}", "┌───────────────────────────────────────────────────────────────────────────────────────────────────────────────────┐".cyan()));
        runner.print_debug(&format!(
            "{} 📝 {:<110} {}\n{}{:<115}{}",
            "│".cyan(),
            "LOG3".cyan(),
            "│".cyan(),
            "│".cyan(),
            "",
            "│".cyan()
        ));
        runner.print_debug(&format!(
            "{} {}: {}{:<12} {}",
            "│".cyan(),
            "Data".bright_magenta(),
            data_hex,
            "",
            "│".cyan()
        ));
        runner.print_debug(&format!(
            "{} {:<4}: {}{:<12} {}",
            "│".cyan(),
            "T1".bright_magenta(),
            topic1_hex,
            "",
            "│".cyan()
        ));
        runner.print_debug(&format!(
            "{} {:<4}: {}{:<12} {}",
            "│".cyan(),
            "T2".bright_magenta(),
            topic2_hex,
            "",
            "│".cyan()
        ));
        runner.print_debug(&format!(
            "{} {:<4}: {}{:<12} {}",
            "│".cyan(),
            "T3".bright_magenta(),
            topic3_hex,
            "",
            "│".cyan()
        ));
        runner.print_debug(&format!("{}", "└───────────────────────────────────────────────────────────────────────────────────────────────────────────────────┘\n".cyan()));
    }

    // Increment PC
    runner.increment_pc(1)
}

/// Executes the LOG4 operation, which logs a message with four topics.
/// 
/// # Arguments
/// 
/// * `runner` - A mutable reference to the `Runner` struct.
/// 
/// # Errors
/// 
/// Returns an `ExecutionError` if static mode is enabled.
/// 
/// # Panics
/// 
/// This function panics if the `offset` or `size` values are invalid.
pub fn log4(runner: &mut Runner) -> Result<(), ExecutionError> {
    // Check if static mode is enabled
    if runner.state.static_mode {
        return Err(ExecutionError::StaticCallStateChanged);
    }
    let offset = U256::from_big_endian(&runner.stack.pop()?);
    let size: U256 = U256::from_big_endian(&runner.stack.pop()?);

    let raw_topic1: U256 = U256::from_big_endian(&runner.stack.pop()?);
    let mut topic1 = [0u8; 32];
    raw_topic1.to_big_endian(&mut topic1);

    let raw_topic2: U256 = U256::from_big_endian(&runner.stack.pop()?);
    let mut topic2 = [0u8; 32];
    raw_topic2.to_big_endian(&mut topic2);

    let raw_topic3: U256 = U256::from_big_endian(&runner.stack.pop()?);
    let mut topic3 = [0u8; 32];
    raw_topic3.to_big_endian(&mut topic3);

    let raw_topic4: U256 = U256::from_big_endian(&runner.stack.pop()?);
    let mut topic4 = [0u8; 32];
    raw_topic4.to_big_endian(&mut topic4);

    let log_data = unsafe { runner.memory.read(offset.as_usize(), size.as_usize())? };

    let log = Log {
        address: runner.address,
        topics: vec![topic1, topic2, topic3, topic4],
        data: log_data.clone(),
    };

    runner.state.logs.push(log);

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let data_hex = utils::debug::vec_to_hex_string(log_data);
        let topic1_hex = utils::debug::to_hex_string(topic1);
        let topic2_hex = utils::debug::to_hex_string(topic2);
        let topic3_hex = utils::debug::to_hex_string(topic3);
        let topic4_hex = utils::debug::to_hex_string(topic4);

        runner.print_debug(&format!("{}", "┌───────────────────────────────────────────────────────────────────────────────────────────────────────────────────┐".cyan()));
        runner.print_debug(&format!(
            "{} 📝 {:<110} {}\n{}{:<115}{}",
            "│".cyan(),
            "LOG4".cyan(),
            "│".cyan(),
            "│".cyan(),
            "",
            "│".cyan()
        ));
        runner.print_debug(&format!(
            "{} {}: {}{:<12} {}",
            "│".cyan(),
            "Data".bright_magenta(),
            data_hex,
            "",
            "│".cyan()
        ));
        runner.print_debug(&format!(
            "{} {:<4}: {}{:<12} {}",
            "│".cyan(),
            "T1".bright_magenta(),
            topic1_hex,
            "",
            "│".cyan()
        ));
        runner.print_debug(&format!(
            "{} {:<4}: {}{:<12} {}",
            "│".cyan(),
            "T2".bright_magenta(),
            topic2_hex,
            "",
            "│".cyan()
        ));
        runner.print_debug(&format!(
            "{} {:<4}: {}{:<12} {}",
            "│".cyan(),
            "T3".bright_magenta(),
            topic3_hex,
            "",
            "│".cyan()
        ));
        runner.print_debug(&format!(
            "{} {:<4}: {}{:<12} {}",
            "│".cyan(),
            "T4".bright_magenta(),
            topic4_hex,
            "",
            "│".cyan()
        ));
        runner.print_debug(&format!("{}", "└───────────────────────────────────────────────────────────────────────────────────────────────────────────────────┘\n".cyan()));
    }

    // Increment PC
    runner.increment_pc(1)
}

#[cfg(test)]
mod tests {
    use crate::core_module::runner::Runner;
    use crate::core_module::utils::bytes::{_hex_string_to_bytes, pad_left};
    use crate::core_module::utils::errors::ExecutionError;

    #[test]
    fn test_log0() {
        let mut runner = Runner::_default(3);
        let interpret_result: Result<(), ExecutionError> =
            runner.interpret(_hex_string_to_bytes("604260005260206000a0"), Some(2), true);
        assert!(interpret_result.is_ok());

        let log = runner.state.logs.get(0).unwrap();

        assert!(log.topics.len() == 0);
        assert!(log.address == runner.address);
        assert!(log.data == pad_left(&[0x42]));
    }

    #[test]
    fn test_log1() {
        let mut runner = Runner::_default(3);
        let interpret_result: Result<(), ExecutionError> = runner.interpret(
            _hex_string_to_bytes("604260005260ff60206000a1"),
            Some(2),
            true,
        );
        assert!(interpret_result.is_ok());

        let log = runner.state.logs.get(0).unwrap();

        assert!(log.topics.len() == 1);
        assert!(log.topics[0] == pad_left(&[0xff]));
        assert!(log.address == runner.address);
        assert!(log.data == pad_left(&[0x42]));
    }

    #[test]
    fn test_log2() {
        let mut runner = Runner::_default(3);
        let interpret_result: Result<(), ExecutionError> = runner.interpret(
            _hex_string_to_bytes("6042600052606060ff60206000a2"),
            Some(2),
            true,
        );
        assert!(interpret_result.is_ok());

        let log = runner.state.logs.get(0).unwrap();

        assert!(log.topics.len() == 2);
        assert!(log.topics[0] == pad_left(&[0xff]));
        assert!(log.topics[1] == pad_left(&[0x60]));
        assert!(log.address == runner.address);
        assert!(log.data == pad_left(&[0x42]));
    }

    #[test]
    fn test_log3() {
        let mut runner = Runner::_default(3);
        let interpret_result: Result<(), ExecutionError> = runner.interpret(
            _hex_string_to_bytes("604260005260ac606060ff60206000a3"),
            Some(2),
            true,
        );
        assert!(interpret_result.is_ok());

        let log = runner.state.logs.get(0).unwrap();

        assert!(log.topics.len() == 3);
        assert!(log.topics[0] == pad_left(&[0xff]));
        assert!(log.topics[1] == pad_left(&[0x60]));
        assert!(log.topics[2] == pad_left(&[0xac]));
        assert!(log.address == runner.address);
        assert!(log.data == pad_left(&[0x42]));
    }

    #[test]
    fn test_log4() {
        let mut runner = Runner::_default(3);
        let interpret_result: Result<(), ExecutionError> = runner.interpret(
            _hex_string_to_bytes("6042600052601d60ac606060ff60206000a4"),
            Some(2),
            true,
        );
        assert!(interpret_result.is_ok());

        let log = runner.state.logs.get(0).unwrap();

        assert!(log.topics.len() == 4);
        assert!(log.topics[0] == pad_left(&[0xff]));
        assert!(log.topics[1] == pad_left(&[0x60]));
        assert!(log.topics[2] == pad_left(&[0xac]));
        assert!(log.topics[3] == pad_left(&[0x1d]));
        assert!(log.address == runner.address);
        assert!(log.data == pad_left(&[0x42]));
    }
}
