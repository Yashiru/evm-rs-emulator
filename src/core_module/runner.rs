use crate::core_module::utils::bytes::pad_left;

use super::memory::Memory;
use super::op_codes;
use super::stack::Stack;
use super::state::EvmState;
use super::utils;
use super::utils::environment::{increment_nonce, init_account};
use super::utils::errors::ExecutionError;

use ethers::types::U256;

// Colored output
use colored::*;

pub struct Runner {
    // Execution
    pub pc: usize,
    pub bytecode: Vec<u8>,
    pub debug_level: Option<u8>,
    pub call_depth: u32,

    // Environment
    pub gas: u64,
    pub origin: [u8; 20],
    pub caller: [u8; 20],
    pub callvalue: [u8; 32],
    pub address: [u8; 20],

    // Data
    pub state: EvmState,
    pub memory: Memory,
    pub calldata: Memory,
    pub returndata: Memory,
    pub stack: Stack,
}

/// Implementation of the Runner struct, which is responsible for executing EVM bytecode.
impl Runner {
    /// Creates a new instance of the EVM runner with the given parameters.
    ///
    /// # Arguments
    ///
    /// * `caller` - The address of the account that initiated the call.
    /// * `origin` - The address of the account that originally initiated the transaction.
    /// * `address` - The address of the account that will receive the call.
    /// * `callvalue` - The value (in wei) that was sent along with the call.
    /// * `calldata` - The input data for the call.
    /// * `state` - The initial state of the EVM.
    ///
    /// # Returns
    ///
    /// A new instance of the EVM runner.
    pub fn new(
        caller: [u8; 20],
        origin: Option<[u8; 20]>,
        address: Option<[u8; 20]>,
        callvalue: Option<[u8; 32]>,
        calldata: Option<Vec<u8>>,
        state: Option<EvmState>,
    ) -> Self {
        let mut instance = Self {
            // Set the program counter to 0
            pc: 0,
            gas: 30_000_000,
            // Create a new storage
            state: if state.is_some() {
                state.unwrap()
            } else {
                EvmState::new(None)
            },
            // Create an empty memory
            memory: Memory::new(None),
            // Create an empty memory for the call data
            calldata: Memory::new(calldata),
            // Create an empty memory for the return data
            returndata: Memory::new(None),
            // Create a new stack
            stack: Stack::new(),
            // Set the caller
            caller,
            // Set the address
            address: if address.is_some() {
                address.unwrap()
            } else {
                [0x5fu8; 20]
            },
            // Set the call value
            callvalue: if callvalue.is_some() {
                callvalue.unwrap()
            } else {
                [0u8; 32]
            },
            // Set the origin
            origin: if origin.is_some() {
                origin.unwrap()
            } else {
                caller
            },
            // Create a new empty bytecode
            bytecode: Vec::new(),
            // Set debug mode to false
            debug_level: None,
            // Set the call depth to 0
            call_depth: 0,
        };

        // Initialize accounts in the EVM state
        let _ = init_account(instance.address, &mut instance);
        let _ = init_account(instance.caller, &mut instance);
        let _ = init_account(instance.origin, &mut instance);

        // Set caller balance to 1000
        let mut result_bytes = [0u8; 32];
        U256::from("3635C9ADC5DEA00000").to_big_endian(&mut result_bytes);
        instance
            .state
            .accounts
            .get_mut(&instance.caller)
            .unwrap()
            .balance = result_bytes;

        // Return the instance
        instance
    }

    /// Creates a new `Runner` instance with default values and sets the debug level to the given value.
    ///
    /// # Arguments
    ///
    /// * `debug_level` - A `u8` value representing the debug level to set.
    ///
    /// # Returns
    ///
    /// A new `Runner` instance with default values and the debug level set to the given value.
    pub fn _default(debug_level: u8) -> Self {
        let mut runner = Self::new(
            [
                0xbe, 0x86, 0x2a, 0xd9, 0xab, 0xfe, 0x6f, 0x22, 0xbc, 0xb0, 0x87, 0x71, 0x6c, 0x7d,
                0x89, 0xa2, 0x60, 0x51, 0xf7, 0x4c,
            ],
            None,
            Some([0xab; 20]),
            None,
            None,
            Some(EvmState::new(Some(String::from(
                "https://eth.llamarpc.com",
            )))),
        );
        runner.debug_level = Some(debug_level);

        runner
    }

    /// Increments the program counter by the specified size.
    ///
    /// # Arguments
    ///
    /// * `size` - The number of bytes to increment the program counter by.
    ///
    /// # Errors
    ///
    /// Returns an `ExecutionError` if the program counter goes out of bounds.
    pub fn increment_pc(&mut self, size: usize) -> Result<(), ExecutionError> {
        self.pc += size;
        Ok(())
    }

    /// Sets the program counter to the specified value.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to set the program counter to.
    pub fn set_pc(&mut self, value: usize) {
        self.pc = value;
    }

    /// Returns the current value of the program counter.
    pub fn get_pc(&mut self) -> usize {
        self.pc
    }

    /// Interprets the given bytecode and executes it on the EVM.
    ///
    /// # Arguments
    ///
    /// * `bytecode` - A vector of bytes representing the bytecode to be executed.
    /// * `debug` - An optional u8 value representing the debug level. If set to 2 or higher, debug information will be printed.
    /// * `initial_interpretation` - A boolean value indicating whether this is the initial interpretation of the bytecode.
    ///
    /// # Errors
    ///
    /// Returns an `ExecutionError` if an error occurs during execution.
    ///
    /// # Examples
    ///
    /// ```
    /// use evm_rs_emulator::Runner;
    ///
    /// let mut runner = Runner::_default(255);
    /// let bytecode = vec![0x60, 0x01, 0x60, 0x02, 0x01, 0x00, 0x00];
    /// let result = runner.interpret(bytecode, Some(2), true);
    /// assert!(result.is_ok());
    /// ```
    pub fn interpret(
        &mut self,
        bytecode: Vec<u8>,
        debug: Option<u8>,
        initial_interpretation: bool,
    ) -> Result<(), ExecutionError> {
        // Set the bytecode
        self.bytecode = bytecode;

        if initial_interpretation {
            // Set the runner address code
            let put_code_result = self.state.put_code_at(self.address, self.bytecode.clone());
            if put_code_result.is_err() {
                return Err(put_code_result.unwrap_err());
            }
        }

        // Set the bytecode
        self.debug_level = debug;

        /* -------------------------------------------------------------------------- */
        /*                             Print debug header                             */
        /* -------------------------------------------------------------------------- */

        if debug.is_some() && debug.unwrap() >= 2 && self.call_depth == 0 {
            self.debug_header();
        }

        /* -------------------------------------------------------------------------- */
        /*                             Interpret bytecode                             */
        /* -------------------------------------------------------------------------- */
        let mut error: Option<ExecutionError> = None;

        // Check if the bytecode is empty
        if self.bytecode.is_empty() {
            // Return an error
            println!("{}: {}", "ERROR: ".red(), ExecutionError::EmptyByteCode);
            return Err(ExecutionError::EmptyByteCode);
        }

        // Interpret the bytecode
        while self.pc < self.bytecode.len() {
            // Interpret an opcode
            let result = self.interpret_op_code(self.bytecode[self.pc]);

            // Check if the interpretation was successful
            if result.is_err() {
                // Store the execution error
                error = Some(result.unwrap_err());
                break;
            }
        }

        /* -------------------------------------------------------------------------- */
        /*                             Print debug footer                             */
        /* -------------------------------------------------------------------------- */

        if debug.is_some() && debug.unwrap() >= 3 && self.call_depth == 0 {
            // Debug stack
            self.debug_stack();

            // Debug memory
            self.debug_memory();
        }

        if debug.is_some() && debug.unwrap() >= 4 && self.call_depth == 0 {
            // Debug storage
            self.debug_storage();
        }

        /* -------------------------------------------------------------------------- */
        /*                            Print execution error                           */
        /* -------------------------------------------------------------------------- */

        if error.is_some() {
            println!(
                "{} {}\n  {}: 0x{:X}\n  {}: 0x{:X}\n  {}",
                "ERROR:".red(),
                "Runtime error".red(),
                "PC".yellow(),
                self.pc,
                "OpCode".yellow(),
                self.bytecode[self.pc],
                error.as_ref().unwrap().to_string().red()
            );

            return Err(error.unwrap());
        }

        Ok(())
    }

    /// Interpret a single opcode.
    ///
    /// # Arguments
    ///
    /// * `opcode` - A single opcode to interpret.
    ///
    /// # Errors
    ///
    /// Returns an `ExecutionError` if the opcode is invalid or if an error occurs during execution.
    ///
    /// # OpCodes
    ///
    /// The function matches the given opcode with the corresponding function from the `op_codes` module.
    /// The OpCodes are divided into the following categories:
    ///
    /// * Execution OpCodes
    /// * Math operations OpCodes
    /// * Push OpCodes
    /// * Dup OpCodes
    /// * Swap OpCodes
    /// * Memory OpCodes
    /// * Storage OpCodes
    ///
    /// For more information on each OpCode, please refer to the `op_codes` module.
    pub fn interpret_op_code(&mut self, opcode: u8) -> Result<(), ExecutionError> {
        match opcode {
            /* ---------------------------- Execution OpCodes --------------------------- */
            0x00 => op_codes::flow::stop(self),

            /* ------------------------- Math operations OpCodes ------------------------ */
            0x01 => op_codes::arithmetic::unsigned::add(self),
            0x02 => op_codes::arithmetic::unsigned::mul(self),
            0x03 => op_codes::arithmetic::unsigned::sub(self),
            0x04 => op_codes::arithmetic::unsigned::div(self),
            0x06 => op_codes::arithmetic::unsigned::modulo(self),
            0x08 => op_codes::arithmetic::unsigned::addmod(self),
            0x09 => op_codes::arithmetic::unsigned::mulmod(self),
            0x0a => op_codes::arithmetic::unsigned::exp(self),
            0x05 => op_codes::arithmetic::signed::sdiv(self),
            0x07 => op_codes::arithmetic::signed::smodulo(self),

            /* ------------------------------ Push OpCodes ------------------------------ */
            0x50 => op_codes::stack::pop::pop(self),

            0x5f => op_codes::stack::push::push(self, 0),
            0x60 => op_codes::stack::push::push(self, 1),
            0x61 => op_codes::stack::push::push(self, 2),
            0x62 => op_codes::stack::push::push(self, 3),
            0x63 => op_codes::stack::push::push(self, 4),
            0x64 => op_codes::stack::push::push(self, 5),
            0x65 => op_codes::stack::push::push(self, 6),
            0x66 => op_codes::stack::push::push(self, 7),
            0x67 => op_codes::stack::push::push(self, 8),
            0x68 => op_codes::stack::push::push(self, 9),
            0x69 => op_codes::stack::push::push(self, 10),
            0x6a => op_codes::stack::push::push(self, 11),
            0x6b => op_codes::stack::push::push(self, 12),
            0x6c => op_codes::stack::push::push(self, 13),
            0x6d => op_codes::stack::push::push(self, 14),
            0x6e => op_codes::stack::push::push(self, 15),
            0x6f => op_codes::stack::push::push(self, 16),
            0x70 => op_codes::stack::push::push(self, 17),
            0x71 => op_codes::stack::push::push(self, 18),
            0x72 => op_codes::stack::push::push(self, 19),
            0x73 => op_codes::stack::push::push(self, 20),
            0x74 => op_codes::stack::push::push(self, 21),
            0x75 => op_codes::stack::push::push(self, 22),
            0x76 => op_codes::stack::push::push(self, 23),
            0x77 => op_codes::stack::push::push(self, 24),
            0x78 => op_codes::stack::push::push(self, 25),
            0x79 => op_codes::stack::push::push(self, 26),
            0x7a => op_codes::stack::push::push(self, 27),
            0x7b => op_codes::stack::push::push(self, 28),
            0x7c => op_codes::stack::push::push(self, 29),
            0x7d => op_codes::stack::push::push(self, 30),
            0x7e => op_codes::stack::push::push(self, 31),
            0x7f => op_codes::stack::push::push(self, 32),

            /* ------------------------------- Dup OpCodes ------------------------------ */
            0x80 => op_codes::stack::dup::dup1(self),
            0x81 => op_codes::stack::dup::dup2(self),
            0x82 => op_codes::stack::dup::dup3(self),
            0x83 => op_codes::stack::dup::dup4(self),
            0x84 => op_codes::stack::dup::dup5(self),
            0x85 => op_codes::stack::dup::dup6(self),
            0x86 => op_codes::stack::dup::dup7(self),
            0x87 => op_codes::stack::dup::dup8(self),
            0x88 => op_codes::stack::dup::dup9(self),
            0x89 => op_codes::stack::dup::dup10(self),
            0x8a => op_codes::stack::dup::dup11(self),
            0x8b => op_codes::stack::dup::dup12(self),
            0x8c => op_codes::stack::dup::dup13(self),
            0x8d => op_codes::stack::dup::dup14(self),
            0x8e => op_codes::stack::dup::dup15(self),
            0x8f => op_codes::stack::dup::dup16(self),

            /* ------------------------------- Swap OpCodes ----------------------------- */
            0x90 => op_codes::stack::swap::swap1(self),
            0x91 => op_codes::stack::swap::swap2(self),
            0x92 => op_codes::stack::swap::swap3(self),
            0x93 => op_codes::stack::swap::swap4(self),
            0x94 => op_codes::stack::swap::swap5(self),
            0x95 => op_codes::stack::swap::swap6(self),
            0x96 => op_codes::stack::swap::swap7(self),
            0x97 => op_codes::stack::swap::swap8(self),
            0x98 => op_codes::stack::swap::swap9(self),
            0x99 => op_codes::stack::swap::swap10(self),
            0x9a => op_codes::stack::swap::swap11(self),
            0x9b => op_codes::stack::swap::swap12(self),
            0x9c => op_codes::stack::swap::swap13(self),
            0x9d => op_codes::stack::swap::swap14(self),
            0x9e => op_codes::stack::swap::swap15(self),
            0x9f => op_codes::stack::swap::swap16(self),

            /* ----------------------------- Memory OpCodes ----------------------------- */
            0x51 => op_codes::memory::mload(self),
            0x52 => op_codes::memory::mstore(self),
            0x59 => op_codes::memory::msize(self),

            /* ----------------------------- Storage OpCodes ---------------------------- */
            0x54 => op_codes::storage::sload(self),
            0x55 => op_codes::storage::sstore(self),

            /* --------------------------- Comparison OpCodes --------------------------- */
            0x10 => op_codes::comparison::lt(self),
            0x11 => op_codes::comparison::gt(self),
            0x12 => op_codes::comparison::slt(self),
            0x13 => op_codes::comparison::sgt(self),
            0x14 => op_codes::comparison::eq(self),
            0x15 => op_codes::comparison::iszero(self),

            /* ----------------------- Bitwise Operations OpCodes ----------------------- */
            0x16 => op_codes::bitwise::and(self),
            0x17 => op_codes::bitwise::or(self),
            0x18 => op_codes::bitwise::xor(self),
            0x19 => op_codes::bitwise::not(self),
            0x1b => op_codes::bitwise::shl(self),
            0x1c => op_codes::bitwise::shr(self),
            0x20 => op_codes::bitwise::sha(self),

            /* ---------------------------- Environment OpCodes ------------------------- */
            0x30 => op_codes::environment::address(self),
            0x31 => op_codes::environment::balance(self),
            0x32 => op_codes::environment::origin(self),
            0x33 => op_codes::environment::caller(self),
            0x34 => op_codes::environment::callvalue(self),
            0x35 => op_codes::environment::calldataload(self),
            0x36 => op_codes::environment::calldatasize(self),
            0x37 => op_codes::environment::calldatacopy(self),
            0x38 => op_codes::environment::codesize(self),
            0x39 => op_codes::environment::codecopy(self),
            0x3a => op_codes::environment::gasprice(self),
            0x3b => op_codes::environment::extcodesize(self),
            0x3c => op_codes::environment::extcodecopy(self),
            0x3d => op_codes::environment::returndatasize(self),
            0x3e => op_codes::environment::returndatacopy(self),
            0x3f => op_codes::environment::extcodehash(self),
            0x40 => op_codes::environment::blockhash(self),
            0x41 => op_codes::environment::coinbase(self),
            0x42 => op_codes::environment::timestamp(self),
            0x43 => op_codes::environment::number(self),
            0x44 => op_codes::environment::difficulty(self),
            0x45 => op_codes::environment::gaslimit(self),
            0x46 => op_codes::environment::chainid(self),
            0x47 => op_codes::environment::selfbalance(self),
            0x48 => op_codes::environment::basefee(self),

            /* ------------------------------ Flow OpCodes ------------------------------ */
            0x56 => op_codes::flow::jump(self),
            0x57 => op_codes::flow::jumpi(self),
            0x58 => op_codes::flow::pc(self),
            0x5a => op_codes::flow::gas(self),
            0x5b => op_codes::flow::jumpdest(self),
            0xfd => op_codes::flow::revert(self),

            /* ------------------------------- Log OpCodes ------------------------------ */
            0xa0 => op_codes::log::log0(self),
            0xa1 => op_codes::log::log1(self),
            0xa2 => op_codes::log::log2(self),
            0xa3 => op_codes::log::log3(self),
            0xa4 => op_codes::log::log4(self),

            /* ----------------------------- System OpCodes ----------------------------- */
            0xf0 => op_codes::system::create(self),
            0xf1 => op_codes::system::call(self, false),
            0xf2 => op_codes::system::callcode(self),
            0xf3 => op_codes::system::return_(self),
            0xf4 => op_codes::system::delegatecall(self),
            0xf5 => op_codes::system::create2(self),
            0xfa => op_codes::system::staticcall(self),
            0xff => op_codes::system::selfdestruct(self),

            // Default case
            _ => op_codes::system::invalid(self),
        }
    }

    /// Executes a call to a contract.
    /// Set up a new runner environment for the call and interpret the bytecode.
    ///
    /// # Arguments
    ///
    /// * `to` - The address of the contract to call.
    /// * `value` - The value to send with the call.
    /// * `calldata` - The input data to the contract.
    /// * `_gas` - The gas limit for the call (currently unused).
    /// * `delegate` - Whether the call is a delegate call.
    ///
    /// # Errors
    ///
    /// Returns an `ExecutionError` if the call fails.
    pub fn call(
        &mut self,
        to: [u8; 20],
        value: [u8; 32],
        calldata: Vec<u8>,
        _gas: u64,
        delegate: bool,
    ) -> Result<(), ExecutionError> {
        let mut error: Option<ExecutionError> = None;

        // Store the initial runner state
        let initial_caller = self.caller.clone();
        let initial_callvalue = self.callvalue.clone();
        let initial_address = self.address.clone();
        let initial_calldata = self.calldata.clone();
        let initial_returndata = self.returndata.clone();
        let initial_memory = self.memory.clone();
        let initial_stack = self.stack.clone();
        let initial_pc = self.pc.clone();
        let initial_debug_level = self.debug_level.clone();
        let initial_bytecode = self.bytecode.clone();

        // Update runner state
        if !delegate {
            self.caller = self.address.clone();
            self.callvalue = value;
            self.address = to;
        }
        self.call_depth += 1;
        self.calldata = Memory::new(Some(calldata));
        self.returndata = Memory::new(None);
        self.memory = Memory::new(None);
        self.stack = Stack::new();
        self.pc = 0;
        self.debug_level = if self.debug_level.is_some() && self.debug_level.unwrap() > 1 {
            Some(self.debug_level.unwrap() - 1)
        } else {
            Some(0)
        };

        // Interpret the bytecode
        let code = self.state.get_code_at(to)?.to_owned();
        let interpret_result = self.interpret(code, self.debug_level, false);

        // Check if the interpretation was successful
        if interpret_result.is_err() {
            error = Some(interpret_result.unwrap_err());
        }

        // Get the return data
        let return_data = self.returndata.heap.clone();

        // Restore the initial runner state
        if !delegate {
            self.caller = initial_caller;
            self.callvalue = initial_callvalue;
            self.address = initial_address;
        }
        self.calldata = initial_calldata;
        self.returndata = initial_returndata;
        self.memory = initial_memory;
        self.stack = initial_stack;
        self.pc = initial_pc;
        self.debug_level = initial_debug_level;
        self.bytecode = initial_bytecode;
        self.call_depth -= 1;

        // Write the return data to the initial state
        self.returndata.heap = return_data;

        // Increment the nonce of the caller
        increment_nonce(self.address, self)?;

        if error.is_some() {
            return Err(error.unwrap());
        }

        // Return Ok
        Ok(())
    }

    /* -------------------------------------------------------------------------- */
    /*                               Debug functions                              */
    /* -------------------------------------------------------------------------- */

    /// Prints a debug message with a tab prefix that indicates the current call depth.
    /// The more the call depth is high, the more the tab prefix will be long.
    pub fn print_debug(&self, s: &str) {
        let prefix = "    ".repeat(self.call_depth as usize);
        println!("{}{}", prefix, s);
    }

    /// Print a debug message that indicate the start of the runner interpretation with the contract address.
    fn debug_header(&self) {
        let border_line =
            "╔═════════════════════════════════════════════════════════════════════════════════════════════════╗";
        let footer_line =
            "╚═════════════════════════════════════════════════════════════════════════════════════════════════╝\n";

        /* -------------------------------------------------------------------------- */
        /*                                   Header                                   */
        /* -------------------------------------------------------------------------- */

        println!("{}", border_line.clone().bright_magenta());
        println!(
            "{} {:<95} {}",
            "║".bright_magenta(),
            "CALL START".green(),
            "║".bright_magenta()
        );
        println!(
            "{} {:<95} {}",
            "║".bright_magenta(),
            "",
            "║".bright_magenta()
        );

        /* ------------------------------ From address ------------------------------ */
        println!(
            "{} {:<95} {}",
            "║".bright_magenta(),
            "FROM".cyan(),
            "║".bright_magenta()
        );
        let hex_address = utils::debug::to_hex_string(pad_left(&self.caller));
        println!(
            "{} {:<95} {}",
            "║".bright_magenta(),
            hex_address,
            "║".bright_magenta()
        );

        /* ------------------------------- To address ------------------------------- */
        println!(
            "{} {:<95} {}",
            "║".bright_magenta(),
            "TO".cyan(),
            "║".bright_magenta()
        );
        let hex_address = utils::debug::to_hex_string(pad_left(&self.address));
        println!(
            "{} {:<95} {}",
            "║".bright_magenta(),
            hex_address,
            "║".bright_magenta()
        );

        /* ---------------------------------- Value --------------------------------- */
        println!(
            "{} {:<95} {}",
            "║".bright_magenta(),
            "VALUE".cyan(),
            "║".bright_magenta()
        );
        let hex = utils::debug::to_hex_string(pad_left(&self.callvalue));
        println!(
            "{} {:<95} {}",
            "║".bright_magenta(),
            hex,
            "║".bright_magenta()
        );

        /* ---------------------------------- Data ---------------------------------- */
        println!(
            "{} {:<95} {}",
            "║".bright_magenta(),
            "DATA".cyan(),
            "║".bright_magenta()
        );
        let hex = utils::debug::to_hex_string(pad_left(&self.calldata.heap));
        println!(
            "{} {:<95} {}",
            "║".bright_magenta(),
            hex,
            "║".bright_magenta()
        );

        println!("{}", footer_line.clone().bright_magenta());
    }

    /// Print a debug message that display the final stack.
    fn debug_stack(&self) {
        let border_line =
            "\n╔═══════════════════════════════════════════════════════════════════════════════════════════════════════╗";
        let footer_line =
            "╚═══════════════════════════════════════════════════════════════════════════════════════════════════════╝\n";

        println!("\n\n{}", border_line.clone().green());
        println!(
            "{} {:<101} {}",
            "║".green(),
            "Final stack".yellow(),
            "║".green()
        );
        println!("{}", footer_line.clone().green());

        let mut reversed_stack = self.stack.stack.clone();
        reversed_stack.reverse();

        // Print all the stack 32 bytes elements with a space between each bytes
        for (_, element) in reversed_stack.iter().enumerate() {
            let hex: String = utils::debug::to_hex_string(*element);
            println!("{}", hex);
        }
    }

    /// Print a debug message that display the final memory.
    fn debug_memory(&self) {
        let border_line =
            "\n╔═══════════════════════════════════════════════════════════════════════════════════════════════════════╗";
        let footer_line =
            "╚═══════════════════════════════════════════════════════════════════════════════════════════════════════╝\n";

        println!("\n{}", border_line.clone().blue());
        println!(
            "{} {:<101} {}",
            "║".blue(),
            "Final memory heap".yellow(),
            "║".blue()
        );
        println!("{}", footer_line.clone().blue());

        // Print the memory heap 32 bytes by 32 bytes with a space between each bytes
        for chunk in self.memory.heap.chunks(32) {
            let padded_chunk: Vec<u8>;

            if chunk.len() < 32 {
                // If the chunk size is less than 32, create a new vector with enough zeros to reach a total size of 32
                padded_chunk = [chunk.to_vec(), vec![0u8; 32 - chunk.len()]].concat();
            } else {
                // If the chunk size is exactly 32, use it as is
                padded_chunk = chunk.to_vec();
            }

            let hex: String =
                utils::debug::to_hex_string(padded_chunk.as_slice().try_into().unwrap());
            println!("{}", hex);
        }

        if self.memory.heap.is_empty() {
            println!("🚧 {} 🚧", "Empty memory".red());
        }

        println!();
    }

    /// Print a debug message that display the final storage in depth.
    fn debug_storage(&mut self) {
        self.state.debug_state();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push0() {
        let mut runner = Runner::new([0xaa; 20], None, None, None, None, None);
        let _ = runner.interpret(vec![0x5f, 0x5f, 0x5f], Some(1), true);

        assert_eq!(runner.stack.pop().unwrap(), [0u8; 32]);
        assert_eq!(runner.stack.pop().unwrap(), [0u8; 32]);
        assert_eq!(runner.stack.pop().unwrap(), [0u8; 32]);
    }

    #[test]
    fn test_push1() {
        let mut runner = Runner::new([0xaa; 20], None, None, None, None, None);
        let _ = runner.interpret(vec![0x60, 0x01, 0x60, 0x02, 0x60, 0x03], Some(1), true);

        assert_eq!(
            runner.stack.pop().unwrap(),
            [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 3
            ]
        );
        assert_eq!(
            runner.stack.pop().unwrap(),
            [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 2
            ]
        );
        assert_eq!(
            runner.stack.pop().unwrap(),
            [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 1
            ]
        );
    }
}
