use super::utils::errors::ExecutionError;
use super::memory::Memory;
use super::stack::Stack;
use super::op_codes;
use super::utils;

// Colored output
use colored::*;

pub struct Runner {
    pub pc: usize,
    pub heap: Memory,
    pub stack: Stack,
    pub bytecode: Vec<u8>,
    pub debug: Option<bool>,
}

impl Runner {
    pub fn new() -> Self {
        Self {
            // Set the program counter to 0
            pc: 0,
            // Create an empty memory size
            heap: Memory::new(0),
            // Create a new stack
            stack: Stack::new(),
            // Create a new empty bytecode
            bytecode: Vec::new(),
            // Set debug mode to false
            debug: None,
        }
    }
    // Increment the program counter
    pub fn increment_pc(&mut self, size: usize) -> Result<(), ExecutionError> {
        self.pc += size;
        Ok(())
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

        /* -------------------------------------------------------------------------- */
        /*                             Print debug header                             */
        /* -------------------------------------------------------------------------- */
        let border_line =
            "╔══════════════════════════════════════════════════════════════════════════╗".blue();
        let footer_line =
            "╚══════════════════════════════════════════════════════════════════════════╝\n".blue();

        if debug.is_some() && debug.unwrap() {
            println!("{}", border_line.clone().bright_magenta());
            println!("{}", "  Program start".bright_green());
            println!("{}", footer_line.clone().bright_magenta());
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

        // Check if debug mode is enabled
        if debug.is_some() && debug.unwrap() {
            println!("\n\n{}", border_line.clone().truecolor(0, 255, 255));
            println!("{}", "  Final stack".bright_green());
            println!("{}", footer_line.clone().truecolor(0, 255, 255));

            // Print all the stack 32 bytes elements with a space between each bytes
            for (_, element) in self.stack.stack.iter().enumerate() {
                let hex: String = utils::debug::to_hex_string(*element);
                println!("{}", hex);
            }

            println!("\n{}", border_line.truecolor(0, 255, 150));
            println!("{}", "  Final memory heap".bright_green());
            println!("{}", footer_line.truecolor(0, 255, 150));

            // Print the memory heap 32 bytes by 32 bytes with a space between each bytes
            for (_, element) in self.heap.heap.chunks(32).enumerate() {
                let hex: String = utils::debug::to_hex_string(element.try_into().unwrap());
                println!("{}", hex);
            }

            println!("\n\n");
        }

        /* -------------------------------------------------------------------------- */
        /*                            Print execution error                           */
        /* -------------------------------------------------------------------------- */

        if error.is_some() {
            println!(
                "{} {:?}\n  {}: {}\n  {}: {}\n  {}",
                "ERROR:".red(),
                error.as_ref().unwrap(),
                "PC".yellow(),
                self.pc,
                "OpCode".yellow(),
                self.bytecode[self.pc-1],
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
            0x00 => self.stop(),

            /* ------------------------- Math operations OpCodes ------------------------ */
            0x01 => op_codes::math::signed::add(self),
            0x02 => op_codes::math::signed::mul(self),
            0x03 => op_codes::math::signed::sub(self),
            0x04 => op_codes::math::signed::div(self),
            0x06 => op_codes::math::signed::modulo(self),
            0x08 => op_codes::math::signed::addmod(self),
            0x09 => op_codes::math::signed::mulmod(self),
            0x0a => op_codes::math::signed::exp(self),
            0x05 => op_codes::math::unsigned::sdiv(self),
            0x07 => op_codes::math::unsigned::smodulo(self),

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

            // Default case
            _ => {
                // Return an error
                return Err(ExecutionError::InvalidOpcode(opcode));
            }
        }
    }

    /* -------------------------------------------------------------------------- */
    /*                              Execution OpCodes                             */
    /* -------------------------------------------------------------------------- */

    // Stop execution
    pub fn stop(&mut self) -> Result<(), ExecutionError> {
        // Set the program counter to the end of the bytecode
        self.pc = self.bytecode.len();
        Ok(())
    }

    // Revert datas from memory heap
    pub fn revert(&mut self, address: usize, size: usize) -> Result<Vec<u8>, ExecutionError> {
        // Check if the address is out of bounds
        if address + size > self.heap.heap.len() {
            return Err(ExecutionError::OutOfBoundsMemory);
        }

        unsafe { self.heap.read(address, size) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push0() {
        let mut runner = Runner::new();
        let _ = runner.interpret(vec![0x5f, 0x5f, 0x5f], Some(true));

        assert_eq!(unsafe {runner.stack.pop().unwrap()}, [0u8; 32]);
        assert_eq!(unsafe {runner.stack.pop().unwrap()}, [0u8; 32]);
        assert_eq!(unsafe {runner.stack.pop().unwrap()}, [0u8; 32]);
    }

    #[test]
    fn test_push1() {
        let mut runner = Runner::new();
        let _ = runner.interpret(vec![0x60, 0x01, 0x60, 0x02, 0x60, 0x03], Some(true));

        assert_eq!(unsafe {runner.stack.pop().unwrap()}, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3]);
        assert_eq!(unsafe {runner.stack.pop().unwrap()}, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2]);
        assert_eq!(unsafe {runner.stack.pop().unwrap()}, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]);
    }
}