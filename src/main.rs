mod core_module;
use core_module::utils::errors::ExecutionError;
use hex;
use std::fs;

fn main() -> Result<(), ExecutionError> {
    // Create a new interpreter
    let mut interpreter = core_module::runner::Runner::new(
        [
            0xbe, 0x86, 0x2a, 0xd9, 0xab, 0xfe, 0x6f, 0x22, 0xbc, 0xb0, 0x87, 0x71, 0x6c, 0x7d,
            0x89, 0xa2, 0x60, 0x51, 0xf7, 0x4c,
        ],
        None,
        Some([0xab; 20]),
        None,
        None,
        None,
    );

    let result = fs::read_to_string("./bytecode.bin");

    match result {
        Ok(file_content) => {
            let bytecode = hex::decode(file_content.trim()).expect("Decoding failed");

            // Interpret the bytecode
            let _ = interpreter.interpret(bytecode, Some(4), true);
        }
        Err(_) => {
            return Err(ExecutionError::InvalidFile);
        }
    }

    Ok(())
}
