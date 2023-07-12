use crate::core_module::runner::Runner;
use crate::core_module::utils;
use crate::core_module::utils::errors::ExecutionError;

// Primitive types
use ethers::types::U256;

// Colored output
use colored::*;

pub fn log0(runner: &mut Runner) -> Result<(), ExecutionError> {
    let offset = U256::from_big_endian(&unsafe { runner.stack.pop()? });
    let size = U256::from_big_endian(&unsafe { runner.stack.pop()? });

    // Check if the address is out of bounds
    if offset.as_usize() + size.as_usize() > runner.heap.heap.len() {
        return Err(ExecutionError::OutOfBoundsMemory);
    }

    let log_data = unsafe { runner.heap.read(offset.as_usize(), size.as_usize())? };

    if runner.debug.is_some() && runner.debug.unwrap() {
        let hex = utils::debug::vec_to_hex_string(log_data);

        println!("{}", "┌───────────────────────────────────────────────────────────────────────────────────────────────────────────────────┐".cyan());
        println!(
            "{} 📝 {:<110} {}\n{}{:<115}{}",
            "│".cyan(),
            "LOG0".cyan(),
            "│".cyan(),
            "│".cyan(),
            "",
            "│".cyan()
        );
        println!(
            "{} {}: {}{:<12} {}",
            "│".cyan(),
            "Data".bright_magenta(),
            hex,
            "",
            "│".cyan()
        );
        println!("{}", "└───────────────────────────────────────────────────────────────────────────────────────────────────────────────────┘\n".cyan());
    }

    // Increment PC
    runner.increment_pc(1)
}

// Log1
pub fn log1(runner: &mut Runner) -> Result<(), ExecutionError> {
    let offset = U256::from_big_endian(&unsafe { runner.stack.pop()? });
    let size: U256 = U256::from_big_endian(&unsafe { runner.stack.pop()? });

    let raw_topic1: U256 = U256::from_big_endian(&unsafe { runner.stack.pop()? });
    let mut topic1 = [0u8; 32];
    raw_topic1.to_big_endian(&mut topic1);

    println!("topic1: {:?}", topic1);
    // print offset
    println!("offset: {:?}", offset);
    // print size
    println!("size: {:?}", size);

    // Check if the address is out of bounds
    if offset.as_usize() + size.as_usize() > runner.heap.heap.len() {
        return Err(ExecutionError::OutOfBoundsMemory);
    }

    let log_data = unsafe { runner.heap.read(offset.as_usize(), size.as_usize())? };

    if runner.debug.is_some() && runner.debug.unwrap() {
        let data_hex = utils::debug::vec_to_hex_string(log_data);
        let topic1_hex = utils::debug::to_hex_string(topic1);

        println!("{}", "┌───────────────────────────────────────────────────────────────────────────────────────────────────────────────────┐".cyan());
        println!(
            "{} 📝 {:<110} {}\n{}{:<115}{}",
            "│".cyan(),
            "LOG1".cyan(),
            "│".cyan(),
            "│".cyan(),
            "",
            "│".cyan()
        );
        println!(
            "{} {}: {}{:<12} {}",
            "│".cyan(),
            "Data".bright_magenta(),
            data_hex,
            "",
            "│".cyan()
        );
        println!(
            "{} {:<4}: {}{:<12} {}",
            "│".cyan(),
            "T1".bright_magenta(),
            topic1_hex,
            "",
            "│".cyan()
        );
        println!("{}", "└───────────────────────────────────────────────────────────────────────────────────────────────────────────────────┘\n".cyan());
    }

    // Increment PC
    runner.increment_pc(1)
}

// Log2
pub fn log2(runner: &mut Runner) -> Result<(), ExecutionError> {
    let offset = U256::from_big_endian(&unsafe { runner.stack.pop()? });
    let size: U256 = U256::from_big_endian(&unsafe { runner.stack.pop()? });

    let raw_topic1: U256 = U256::from_big_endian(&unsafe { runner.stack.pop()? });
    let mut topic1 = [0u8; 32];
    raw_topic1.to_big_endian(&mut topic1);

    let raw_topic2: U256 = U256::from_big_endian(&unsafe { runner.stack.pop()? });
    let mut topic2 = [0u8; 32];
    raw_topic2.to_big_endian(&mut topic2);

    // Check if the address is out of bounds
    if offset.as_usize() + size.as_usize() > runner.heap.heap.len() {
        return Err(ExecutionError::OutOfBoundsMemory);
    }

    let log_data = unsafe { runner.heap.read(offset.as_usize(), size.as_usize())? };

    if runner.debug.is_some() && runner.debug.unwrap() {
        let data_hex = utils::debug::vec_to_hex_string(log_data);
        let topic1_hex = utils::debug::to_hex_string(topic1);
        let topic2_hex = utils::debug::to_hex_string(topic2);

        println!("{}", "┌───────────────────────────────────────────────────────────────────────────────────────────────────────────────────┐".cyan());
        println!(
            "{} 📝 {:<110} {}\n{}{:<115}{}",
            "│".cyan(),
            "LOG2".cyan(),
            "│".cyan(),
            "│".cyan(),
            "",
            "│".cyan()
        );
        println!(
            "{} {}: {}{:<12} {}",
            "│".cyan(),
            "Data".bright_magenta(),
            data_hex,
            "",
            "│".cyan()
        );
        println!(
            "{} {:<4}: {}{:<12} {}",
            "│".cyan(),
            "T1".bright_magenta(),
            topic1_hex,
            "",
            "│".cyan()
        );
        println!(
            "{} {:<4}: {}{:<12} {}",
            "│".cyan(),
            "T2".bright_magenta(),
            topic2_hex,
            "",
            "│".cyan()
        );
        println!("{}", "└───────────────────────────────────────────────────────────────────────────────────────────────────────────────────┘\n".cyan());
    }

    // Increment PC
    runner.increment_pc(1)
}

// Log3
pub fn log3(runner: &mut Runner) -> Result<(), ExecutionError> {
    let offset = U256::from_big_endian(&unsafe { runner.stack.pop()? });
    let size: U256 = U256::from_big_endian(&unsafe { runner.stack.pop()? });

    let raw_topic1: U256 = U256::from_big_endian(&unsafe { runner.stack.pop()? });
    let mut topic1 = [0u8; 32];
    raw_topic1.to_big_endian(&mut topic1);

    let raw_topic2: U256 = U256::from_big_endian(&unsafe { runner.stack.pop()? });
    let mut topic2 = [0u8; 32];
    raw_topic2.to_big_endian(&mut topic2);

    let raw_topic3: U256 = U256::from_big_endian(&unsafe { runner.stack.pop()? });
    let mut topic3 = [0u8; 32];
    raw_topic3.to_big_endian(&mut topic3);

    // Check if the address is out of bounds
    if offset.as_usize() + size.as_usize() > runner.heap.heap.len() {
        return Err(ExecutionError::OutOfBoundsMemory);
    }

    let log_data = unsafe { runner.heap.read(offset.as_usize(), size.as_usize())? };

    if runner.debug.is_some() && runner.debug.unwrap() {
        let data_hex = utils::debug::vec_to_hex_string(log_data);
        let topic1_hex = utils::debug::to_hex_string(topic1);
        let topic2_hex = utils::debug::to_hex_string(topic2);
        let topic3_hex = utils::debug::to_hex_string(topic3);

        println!("{}", "┌───────────────────────────────────────────────────────────────────────────────────────────────────────────────────┐".cyan());
        println!(
            "{} 📝 {:<110} {}\n{}{:<115}{}",
            "│".cyan(),
            "LOG3".cyan(),
            "│".cyan(),
            "│".cyan(),
            "",
            "│".cyan()
        );
        println!(
            "{} {}: {}{:<12} {}",
            "│".cyan(),
            "Data".bright_magenta(),
            data_hex,
            "",
            "│".cyan()
        );
        println!(
            "{} {:<4}: {}{:<12} {}",
            "│".cyan(),
            "T1".bright_magenta(),
            topic1_hex,
            "",
            "│".cyan()
        );
        println!(
            "{} {:<4}: {}{:<12} {}",
            "│".cyan(),
            "T2".bright_magenta(),
            topic2_hex,
            "",
            "│".cyan()
        );
        println!(
            "{} {:<4}: {}{:<12} {}",
            "│".cyan(),
            "T3".bright_magenta(),
            topic3_hex,
            "",
            "│".cyan()
        );
        println!("{}", "└───────────────────────────────────────────────────────────────────────────────────────────────────────────────────┘\n".cyan());
    }

    // Increment PC
    runner.increment_pc(1)
}

// Log4
pub fn log4(runner: &mut Runner) -> Result<(), ExecutionError> {
    let offset = U256::from_big_endian(&unsafe { runner.stack.pop()? });
    let size: U256 = U256::from_big_endian(&unsafe { runner.stack.pop()? });

    let raw_topic1: U256 = U256::from_big_endian(&unsafe { runner.stack.pop()? });
    let mut topic1 = [0u8; 32];
    raw_topic1.to_big_endian(&mut topic1);

    let raw_topic2: U256 = U256::from_big_endian(&unsafe { runner.stack.pop()? });
    let mut topic2 = [0u8; 32];
    raw_topic2.to_big_endian(&mut topic2);

    let raw_topic3: U256 = U256::from_big_endian(&unsafe { runner.stack.pop()? });
    let mut topic3 = [0u8; 32];
    raw_topic3.to_big_endian(&mut topic3);

    let raw_topic4: U256 = U256::from_big_endian(&unsafe { runner.stack.pop()? });
    let mut topic4 = [0u8; 32];
    raw_topic4.to_big_endian(&mut topic4);

    // Check if the address is out of bounds
    if offset.as_usize() + size.as_usize() > runner.heap.heap.len() {
        return Err(ExecutionError::OutOfBoundsMemory);
    }

    let log_data = unsafe { runner.heap.read(offset.as_usize(), size.as_usize())? };

    if runner.debug.is_some() && runner.debug.unwrap() {
        let data_hex = utils::debug::vec_to_hex_string(log_data);
        let topic1_hex = utils::debug::to_hex_string(topic1);
        let topic2_hex = utils::debug::to_hex_string(topic2);
        let topic3_hex = utils::debug::to_hex_string(topic3);
        let topic4_hex = utils::debug::to_hex_string(topic4);

        println!("{}", "┌───────────────────────────────────────────────────────────────────────────────────────────────────────────────────┐".cyan());
        println!(
            "{} 📝 {:<110} {}\n{}{:<115}{}",
            "│".cyan(),
            "LOG4".cyan(),
            "│".cyan(),
            "│".cyan(),
            "",
            "│".cyan()
        );
        println!(
            "{} {}: {}{:<12} {}",
            "│".cyan(),
            "Data".bright_magenta(),
            data_hex,
            "",
            "│".cyan()
        );
        println!(
            "{} {:<4}: {}{:<12} {}",
            "│".cyan(),
            "T1".bright_magenta(),
            topic1_hex,
            "",
            "│".cyan()
        );
        println!(
            "{} {:<4}: {}{:<12} {}",
            "│".cyan(),
            "T2".bright_magenta(),
            topic2_hex,
            "",
            "│".cyan()
        );
        println!(
            "{} {:<4}: {}{:<12} {}",
            "│".cyan(),
            "T3".bright_magenta(),
            topic3_hex,
            "",
            "│".cyan()
        );
        println!(
            "{} {:<4}: {}{:<12} {}",
            "│".cyan(),
            "T4".bright_magenta(),
            topic4_hex,
            "",
            "│".cyan()
        );
        println!("{}", "└───────────────────────────────────────────────────────────────────────────────────────────────────────────────────┘\n".cyan());
    }

    // Increment PC
    runner.increment_pc(1)
}


