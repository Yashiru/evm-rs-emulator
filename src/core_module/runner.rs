use crate::core_module::utils::bytes32::pad_to_32_bytes;

use super::memory::Memory;
use super::op_codes;
use super::stack::Stack;
use super::state::EvmState;
use super::utils;
use super::utils::environment::init_account;
use super::utils::errors::ExecutionError;

use ethers::types::U256;

// Colored output
use colored::*;

pub struct Runner {
    // Execution
    pub pc: usize,
    pub bytecode: Vec<u8>,
    pub debug: Option<bool>,

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

impl Runner {
    pub fn new(
        caller: [u8; 20],
        origin: Option<[u8; 20]>,
        address: Option<[u8; 20]>,
        callvalue: Option<[u8; 32]>,
        calldata: Option<Vec<u8>>,
        state: Option<EvmState>
    ) -> Self {
        let mut instance = Self {
            // Set the program counter to 0
            pc: 0,
            gas: 30_000_000,
            // Create a new storage
            state: if state.is_some() {
                state.unwrap()
            } else {
                EvmState::new()
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
            debug: None,
        };

        // Initialize accounts in the EVM state
        let _ = init_account(instance.address, &mut instance);
        let _ = init_account(instance.caller, &mut instance);
        let _ = init_account(instance.origin, &mut instance);

        // Set caller balance to 1000
        let mut result_bytes = [0u8; 32];
        U256::from("3635C9ADC5DEA00000").to_big_endian(&mut result_bytes);
        instance.state.accounts.get_mut(&instance.caller).unwrap().balance = result_bytes;

        // Return the instance
        instance
    }
    // Increment the program counter
    pub fn increment_pc(&mut self, size: usize) -> Result<(), ExecutionError> {
        self.pc += size;
        Ok(())
    }

    // Set the program counter
    pub fn set_pc(&mut self, value: usize) {
        self.pc = value;
    }

    // Set the program counter
    pub fn get_pc(&mut self) -> usize {
        self.pc
    }

    pub fn interpret(
        &mut self,
        bytecode: Vec<u8>,
        debug: Option<bool>,
    ) -> Result<(), ExecutionError> {
        // Set the bytecode
        self.bytecode = bytecode;

        // Set the bytecode
        self.debug = debug;

        // Mock returndata
        unsafe {
            self.returndata
                .write(0, [0x01, 0x02, 0x03, 0x04, 0x05, 0x06].to_vec())?;
        }

        /* -------------------------------------------------------------------------- */
        /*                             Print debug header                             */
        /* -------------------------------------------------------------------------- */

        if debug.is_some() && debug.unwrap() {
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

        if debug.is_some() && debug.unwrap() {
            // Debug stack
            self.debug_stack();

            // Debug memory
            self.debug_memory();

            // Debug storage
            self.debug_storage();
        }
        // Check if debug mode is enabled
        if debug.is_some() && debug.unwrap() {
            println!("\n\n");
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

    // Interpret a single opcode
    fn interpret_op_code(&mut self, opcode: u8) -> Result<(), ExecutionError> {
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

            // Default case
            _ => op_codes::system::invalid(self)
        }
    }

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

        /* -------------------------------------------------------------------------- */
        /*                              Contract address                              */
        /* -------------------------------------------------------------------------- */

        println!("{} {:<95} {}", "║".bright_magenta(), "", "║".bright_magenta());
        println!("{} {:<95} {}", "║".bright_magenta(), "To".cyan(), "║".bright_magenta());
        let hex_address = utils::debug::to_hex_string(pad_to_32_bytes(&self.address));
        println!("{} {:<95} {}", "║".bright_magenta(), hex_address, "║".bright_magenta());

        /* -------------------------------------------------------------------------- */
        /*                                  Calldata                                  */
        /* -------------------------------------------------------------------------- */
        println!("{} {:<95} {}", "║".bright_magenta(), "", "║".bright_magenta());
        println!(
            "{} {:<95} {}",
            "║".bright_magenta(),
            "Call data".cyan(),
            "║".bright_magenta()
        );

        // Print the memory heap 32 bytes by 32 bytes with a space between each bytes
        for chunk in self.calldata.heap.chunks(32) {
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
            println!("{} {} {}", "║".bright_magenta(), hex, "║".bright_magenta());
        }

        if self.calldata.heap.is_empty() {
            println!("{} {:<95} {}", "║".bright_magenta(), "No calldata.".truecolor(80, 80, 80), "║".bright_magenta());
        }

        /* -------------------------------------------------------------------------- */
        /*                                 Call value                                 */
        /* -------------------------------------------------------------------------- */

        println!("{} {:<95} {}", "║".bright_magenta(), "", "║".bright_magenta());
        println!("{} {:<95} {}", "║".bright_magenta(), "Call value".cyan(), "║".bright_magenta());
        let hex_callvalue = utils::debug::to_hex_string(self.callvalue);
        println!("{} {:<95} {}", "║".bright_magenta(), hex_callvalue, "║".bright_magenta());

        println!("{}", footer_line.clone().bright_magenta());
    }

    fn debug_stack(&self) {
        let border_line =
            "\n╔═══════════════════════════════════════════════════════════════════════════════════════════════════════╗";
        let footer_line =
            "╚═══════════════════════════════════════════════════════════════════════════════════════════════════════╝\n";

        println!("\n\n{}", border_line.clone().truecolor(0, 255, 255));
        println!("{} {:<101} {}", "║".truecolor(0, 255, 255), "Final stack".bright_yellow(), "║".truecolor(0, 255, 255));
        println!("{}", footer_line.clone().truecolor(0, 255, 255));

        let mut reversed_stack = self.stack.stack.clone();
        reversed_stack.reverse();

        // Print all the stack 32 bytes elements with a space between each bytes
        for (_, element) in reversed_stack.iter().enumerate() {
            let hex: String = utils::debug::to_hex_string(*element);
            println!("{}", hex);
        }
    }

    fn debug_memory(&self) {
        let border_line =
            "\n╔═══════════════════════════════════════════════════════════════════════════════════════════════════════╗";
        let footer_line =
            "╚═══════════════════════════════════════════════════════════════════════════════════════════════════════╝\n";

        println!("\n{}", border_line.clone().truecolor(0, 255, 150));
        println!("{} {:<101} {}", "║".truecolor(0, 255, 150), "Final memory heap".bright_yellow(), "║".truecolor(0, 255, 150));
        println!("{}", footer_line.clone().truecolor(0, 255, 150));

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
    }

    fn debug_storage(&self) {
        self.state.debug_state();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push0() {
        let mut runner = Runner::new([0xaa; 20], None, None, None, None, None);
        let _ = runner.interpret(vec![0x5f, 0x5f, 0x5f], Some(true));

        assert_eq!(unsafe { runner.stack.pop().unwrap() }, [0u8; 32]);
        assert_eq!(unsafe { runner.stack.pop().unwrap() }, [0u8; 32]);
        assert_eq!(unsafe { runner.stack.pop().unwrap() }, [0u8; 32]);
    }

    #[test]
    fn test_push1() {
        let mut runner = Runner::new([0xaa; 20], None, None, None, None, None);
        let _ = runner.interpret(vec![0x60, 0x01, 0x60, 0x02, 0x60, 0x03], Some(true));

        assert_eq!(
            unsafe { runner.stack.pop().unwrap() },
            [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 3
            ]
        );
        assert_eq!(
            unsafe { runner.stack.pop().unwrap() },
            [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 2
            ]
        );
        assert_eq!(
            unsafe { runner.stack.pop().unwrap() },
            [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 1
            ]
        );
    }
}
