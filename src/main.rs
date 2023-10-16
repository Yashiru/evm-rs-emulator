mod core_module;
use core_module::state::EvmState;
use core_module::utils::errors::ExecutionError;
use std::{env, fs};

// Colored output
use colored::*;

fn main() -> Result<(), ExecutionError> {
    let mut caller = [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0xc4, 0x11, 0xe8,
    ];
    let mut origin: Option<[u8; 20]> = None;
    let mut address: Option<[u8; 20]> = Some([
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0xc4, 0x11, 0xee,
    ]);
    let mut value: Option<[u8; 32]> = None;
    let mut data: Option<Vec<u8>> = None;
    let mut bytecode: String;
    let state: EvmState;
    let mut debug_level: Option<u8> = Some(255);

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
        println!(
            "{}: {}",
            "evm-rs version".magenta(),
            env!("CARGO_PKG_VERSION").green()
        );
        return Ok(());
    }

    /* ------------------------ Fetch the caller address ------------------------ */
    let caller_arg = args
        .iter()
        .position(|r| r == "--caller")
        .map(|p| &args[p + 1]);

    if let Some(caller_arg) = caller_arg {
        if caller_arg.len() != 42 {
            unexpected_arg_value("Caller", "an address");
            return Ok(());
        }

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
        if origin_arg.len() != 42 {
            unexpected_arg_value("Origin", "an address");
            return Ok(());
        }

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
        if address_arg.len() != 42 {
            unexpected_arg_value("Address", "an address");
            return Ok(());
        }

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
        if value_arg.len() < 3 || !value_arg.starts_with("0x") {
            unexpected_arg_value("Value", "a hex value");
            return Ok(());
        }

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
        if data_arg.len() < 3 || !data_arg.starts_with("0x") {
            unexpected_arg_value("Data", "a hex value");
            return Ok(());
        }

        let bytes =
            hex::decode(&data_arg[2..]).expect(&"Invalid argument: --data".red().to_string());
        data = Some(
            bytes
                .try_into()
                .expect(&"Invalid argument: --data".red().to_string()),
        );
    }

    /* --------------------------- Fetch the fork url --------------------------- */
    let fork_arg = args
        .iter()
        .position(|r| r == "--fork")
        .map(|p| &args[p + 1]);

    if let Some(fork_arg) = fork_arg {
        // Check if the fork url is valid
        if !fork_arg.starts_with("http") {
            unexpected_arg_value("Fork", "a valid RPC url");
            return Ok(());
        }

        state = EvmState::new(Some(fork_arg.to_string()));
    } else {
        state = EvmState::new(None);
    }

    /* ------------------------- Fetch the bytecode path ------------------------ */
    // The bytecode path is not an argument, but the last argument
    if args.len() > 1 {
        let arg = &args[args.len() - 1];

        if !arg.starts_with("0x") {
            let current_dir = env::current_dir().unwrap();
            bytecode = current_dir.to_str().unwrap().to_string();
            bytecode.push_str("/");
        } else {
            bytecode = "".to_string();
        }
        bytecode.push_str(arg);
    } else {
        print_help();
        return Ok(());
    }

    /* -------------------------- Fetch the debug level ------------------------- */
    let debug_level_arg = args
        .iter()
        .position(|r| r == "--debug-level")
        .map(|p| &args[p + 1]);

    if let Some(debug_level_arg) = debug_level_arg {
        match debug_level_arg.parse::<u8>() {
            Ok(level) => debug_level = Some(level),
            Err(_) => unexpected_arg_value("Debug level", "a 8 bytes usigned integer"),
        }
    }

    // Create a new interpreter
    let mut interpreter =
        core_module::runner::Runner::new(caller, origin, address, value, data, Some(state));

    // Check if bytecode is an hex value of a file path
    if bytecode.starts_with("0x") {
        let bytecode = hex::decode(&bytecode[2..]).expect("Invalid bytecode");

        // Interpret the bytecode
        let _ = interpreter.interpret(bytecode, debug_level, true);
        return Ok(());
    }

    let result = fs::read_to_string(bytecode.to_string());

    match result {
        Ok(file_content) => {
            let bytecode = hex::decode(file_content.trim()).expect("Decoding failed");

            // Interpret the bytecode
            let _ = interpreter.interpret(bytecode, debug_level, true);
        }
        Err(_) => {
            // Print the error
            println!("{} {}", "Error:".red(), "file not found");
            println!(
                "\n  Run this command with {} for more information.",
                "--help".green()
            );
            return Ok(());
        }
    }

    Ok(())
}

fn unexpected_arg_value(arg: &str, arg_type: &str) {
    println!(
        "{} unexpected value for '{}' argument.",
        "Error:".red(),
        format!("--{}", arg.to_lowercase()).yellow()
    );
    println!("  {} should be {arg_type}.", arg.yellow());
    println!(
        "\n  Run this command with {} for more information.",
        "--help".green()
    );
}

fn print_help() {
    println!("Execute arbitrary bytecode on the Ethereum Virtual Machine (EVM)");
    println!(
        "\nUsage: {} [{}] [{}] <{}>",
        "evm-rs".green(),
        "OPTIONS".magenta(),
        "args".blue(),
        "bytecode".cyan()
    );
    println!(
        "       {} can be {} or {} containing the bytecode to be executed.\n",
        "bytecode".cyan(),
        "a raw hex value".yellow(),
        "a path to a file".yellow()
    );
    println!("Options:");
    println!("  --{} <{}>     Override the default address of the contract containing the bytecode to be executed", "address".magenta(), "ADDRESS".blue());
    println!(
        "  --{} <{}>      Override the default caller address",
        "caller".magenta(),
        "ADDRESS".blue()
    );
    println!(
        "  --{} <{}>       Override the default calldata to be passed to the callee contract",
        "data".magenta(),
        "HEX_DATA".blue()
    );
    println!(
        "  --{} <{}>      Override the default origin address",
        "origin".magenta(),
        "ADDRESS".blue()
    );
    println!(
        "  --{} <{}>     Override the default value to be sent to the contract",
        "value".magenta(),
        "HEX_VALUE".blue()
    );
    println!(
        "  --{} <{}>   Override the default debug level (0 to 255)",
        "debug-level".magenta(),
        "LEVEL".blue()
    );
    println!(
        "  --{} <{}>        Set the fork url",
        "fork".magenta(),
        "RPC_URL".blue()
    );
}
