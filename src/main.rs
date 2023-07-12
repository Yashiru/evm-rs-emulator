mod core_module;
use std::fs;
use core_module::utils::{errors::ExecutionError, self};
use hex;


fn main() -> Result<(), ()>  {
    // Create a new interpreter
    let mut interpreter = core_module::runner::Runner::new();
    
    let result = fs::read_to_string("./bytecode.bin");

    match result {
        Ok(file_content) => {
            let bytecode = hex::decode(file_content.trim()).expect("Decoding failed");
            
            // Interpret the bytecode
            interpreter.interpret(bytecode, Some(true));
        },
        Err(_) => {
            return Err(());
        }
    }

    Ok(())
}