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
        Some([
            0x9b, 0xbf, 0xed, 0x68, 0x89, 0x32, 0x2e, 0x01, 0x6e, 0x0a, 0x02, 0xee, 0x45, 0x9d,
            0x30, 0x6f, 0xc1, 0x95, 0x45, 0xd8,
        ]),
        None,
        None,
        None,
    );

    let result = fs::read_to_string("./bytecode.bin");

    match result {
        Ok(file_content) => {
            let bytecode = hex::decode(file_content.trim()).expect("Decoding failed");

            // Interpret the bytecode
            let _ = interpreter.interpret(bytecode, Some(true));
        }
        Err(_) => {
            return Err(ExecutionError::InvalidFile);
        }
    }

    Ok(())
}
