mod core_module;
use core_module::utils::errors::ExecutionError;
use std::{env, fs};

// Colored output
use colored::*;

fn main() -> Result<(), ExecutionError> {
    let mut caller = [
        0xbe, 0x86, 0x2a, 0xd9, 0xab, 0xfe, 0x6f, 0x22, 0xbc, 0xb0, 0x87, 0x71, 0x6c, 0x7d, 0x89,
        0xa2, 0x60, 0x51, 0xf7, 0x4c,
    ];
    let mut origin: Option<[u8; 20]> = None;
    let mut address: Option<[u8; 20]> = Some([0xab; 20]);
    let mut value: Option<[u8; 32]> = None;
    let mut data: Option<Vec<u8>> = None;
    let mut bytecodePath: String = String::from("");

    /* -------------------------------------------------------------------------- */
    /*                               Fetch arguments                              */
    /* -------------------------------------------------------------------------- */

    let args: Vec<String> = env::args().collect();

    // Print help
    if args.contains(&"--help".to_string()) {
        print_help();
        return Ok(());
    }

    // Print version
    if args.contains(&"--version".to_string()) {
        println!("evm-rs version: {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    /* ------------------------ Fetch the caller address ------------------------ */
    let caller_arg = args
        .iter()
        .position(|r| r == "--caller")
        .map(|p| &args[p + 1]);

    if let Some(caller_arg) = caller_arg {
        let bytes =
            hex::decode(&caller_arg[2..]).expect(&"Invalid argument: --caller".red().to_string());
        caller = bytes
            .try_into()
            .expect(&"Invalid argument: --caller".red().to_string());
    }

    /* -------------------------- Fetch origin address -------------------------- */
    let origin_arg = args
        .iter()
        .position(|r| r == "--origin")
        .map(|p| &args[p + 1]);

    if let Some(origin_arg) = origin_arg {
        let bytes =
            hex::decode(&origin_arg[2..]).expect(&"Invalid argument: --origin".red().to_string());
        origin = Some(
            bytes
                .try_into()
                .expect(&"Invalid argument: --origin".red().to_string()),
        );
    }

    /* -------------------------- Fetch callee address -------------------------- */
    let address_arg = args
        .iter()
        .position(|r| r == "--address")
        .map(|p| &args[p + 1]);

    if let Some(address_arg) = address_arg {
        let bytes =
            hex::decode(&address_arg[2..]).expect(&"Invalid argument: --address".red().to_string());
        address = Some(
            bytes
                .try_into()
                .expect(&"Invalid argument: --address".red().to_string()),
        );
    }

    /* ----------------------------- Fetch call value --------------------------- */
    let value_arg = args
        .iter()
        .position(|r| r == "--value")
        .map(|p| &args[p + 1]);

    if let Some(value_arg) = value_arg {
        let bytes =
            hex::decode(&value_arg[2..]).expect(&"Invalid argument: --value".red().to_string());

        let mut padded = [0u8; 32];
        let start_index = 32 - bytes.len();
        padded[start_index..].copy_from_slice(&bytes);

        value = Some(padded);
    }

    /* ----------------------------- Fetch call data ---------------------------- */
    let data_arg = args
        .iter()
        .position(|r| r == "--data")
        .map(|p| &args[p + 1]);

    if let Some(data_arg) = data_arg {
        let bytes =
            hex::decode(&data_arg[2..]).expect(&"Invalid argument: --data".red().to_string());
        data = Some(
            bytes
                .try_into()
                .expect(&"Invalid argument: --data".red().to_string()),
        );
    }

    /* ------------------------- Fetch the bytecode path ------------------------ */
    // The bytecode path is not an argument, but the last argument
    if args.len() > 1 {
        // Manage the current directory and compute the full path
        let current_dir = env::current_dir().unwrap();
        bytecodePath = current_dir.to_str().unwrap().to_string();
        bytecodePath.push_str("/");
        bytecodePath.push_str(&args[args.len() - 1]);
    } else {
        print_help();
        return Ok(());
    }
    // print path
    println!("Bytecode path: {}", bytecodePath);

    // Create a new interpreter
    let mut interpreter =
        core_module::runner::Runner::new(caller, origin, address, value, data, None);

    let result = fs::read_to_string(bytecodePath.to_string());

    match result {
        Ok(file_content) => {
            let bytecode = hex::decode(file_content.trim()).expect("Decoding failed");

            // Interpret the bytecode
            let _ = interpreter.interpret(bytecode, Some(255), true);
        }
        Err(_) => {
            return Err(ExecutionError::InvalidFile);
        }
    }

    Ok(())
}

fn print_help() {
    println!("Execute arbitrary bytecode on the Ethereum Virtual Machine (EVM)");
    println!("\nUsage: evm-rs [OPTIONS] [args]...\n");
    println!("OPTIONS:");
    println!("  --address [<ADDRESS>]     Override the default address of the contract containing the bytecode to be executed");
    println!("  --caller [<ADDRESS>]      Override the default caller address");
    println!(
        "  --data [<HEX_DATA>]       Override the default call data to be passed to the contract"
    );
    println!("  --origin [<ADDRESS>]      Override the default origin address");
    println!("  --value [<HEX_VALUE>]     Override the default value to be sent to the contract");
}
