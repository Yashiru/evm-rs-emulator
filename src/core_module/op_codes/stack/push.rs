use crate::core_module::runner::Runner;
use crate::core_module::utils;
use crate::core_module::utils::errors::ExecutionError;

// Colored output
use colored::*;

pub fn push(runner: &mut Runner, data_len: usize) -> Result<(), ExecutionError> {
    // Check if the data length is out of bounds
    if runner.pc + 1 + data_len > runner.bytecode.len() {
        return Err(ExecutionError::OutOfBoundsByteCode);
    }

    unsafe {
        let data = &runner.bytecode[runner.pc + 1..runner.pc + 1 + data_len];

        let mut padded = [0u8; 32]; // Start with an array of zeroes
        let start = 32 - data.len(); // Calculate where to start copying
        padded[start..].copy_from_slice(data); // Copy the slice into the end of the array

        let result = runner.stack.push(padded);

        if result.is_err() {
            return Err(result.unwrap_err());
        }

        if runner.debug.is_some() && runner.debug.unwrap() {
            let hex: String = utils::debug::to_hex_string(padded);
            println!(
                "{}{} 👉 [ {} ]",
                "PUSH".magenta(),
                data_len.to_string().green(),
                hex
            );
        }
    }

    // Increment PC
    runner.increment_pc(1 + data_len)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push() {
        let mut runner = Runner::new();
        let _ = runner.interpret(vec![0x60, 0xff], Some(true));

        assert_eq!(runner.stack.stack.len(), 1);
        assert_eq!(unsafe { runner.stack.pop().unwrap() }, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255]);
    }
}