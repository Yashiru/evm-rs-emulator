use std::{collections::HashMap, fmt};

use ethers::{types::U256, utils::keccak256};

use crate::core_module::utils;

use super::utils::errors::ExecutionError;

// Colored output
use colored::*;

pub struct AccountState {
    pub nonce: u64,
    pub balance: [u8; 32],
    pub storage: HashMap<[u8; 32], [u8; 32]>,
    pub code_hash: [u8; 32],
}

impl fmt::Debug for AccountState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut code_hash: String = utils::debug::to_hex_string(self.code_hash);
        if self.code_hash == [0u8; 32] {
            code_hash = format!("{}", "Empty code".red()).to_string()
        }

        writeln!(f, "{}: {}", "Nonce".magenta(), self.nonce)?;
        writeln!(
            f,
            "{}: {}",
            "Balance".magenta(),
            U256::from(self.balance).to_string()
        )?;
        writeln!(f, "{}: {}", "Code Hash".magenta(), code_hash)?;
        write!(f, "{}: ", "Storage".magenta())?;
        for (slot, value) in &self.storage {
            println!("\n┌────────────────────────────────────────────────────────────────────────────────────────────────────────┐");
            // Print the slot
            let hex: String = utils::debug::to_hex_string(slot.to_owned());
            println!("│ {}:  {} │", "Slot".bright_blue(), hex);

            // Print the value
            let hex: String = utils::debug::to_hex_string(value.to_owned());
            println!("│ {}: {} │", "Value".blue(), hex);

            println!("└────────────────────────────────────────────────────────────────────────────────────────────────────────┘");
        }
        if self.storage.is_empty() {
            write!(f, "{}", "Empty storage".red())?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct EvmState {
    pub accounts: HashMap<[u8; 20], AccountState>,
    pub codes: HashMap<[u8; 32], Vec<u8>>,
}

impl EvmState {
    pub fn new() -> Self {
        Self {
            accounts: HashMap::new(),
            codes: HashMap::new(),
        }
    }

    // Transfer value from one account to another
    pub fn transfer(
        &mut self,
        from: [u8; 20],
        to: [u8; 20],
        value: [u8; 32],
    ) -> Result<(), ExecutionError> {
        let value_u256 = U256::from_big_endian(&value);

        let from_balance = U256::from_big_endian(
            &self
                .accounts
                .get(&from)
                .ok_or(ExecutionError::AccountNotFound)?
                .balance,
        );

        let to_balance = U256::from_big_endian(
            &self
                .accounts
                .get(&to)
                .ok_or(ExecutionError::AccountNotFound)?
                .balance,
        );

        // Check if the balance is sufficient
        if from_balance < value_u256 {
            return Err(ExecutionError::InsufficientBalance);
        }

        // Transfer the value
        let new_from_balance = from_balance - value_u256;
        let new_to_balance = to_balance + value_u256;

        if let Some(from_account) = self.accounts.get_mut(&from) {
            let mut result_bytes = [0u8; 32];
            new_from_balance.to_big_endian(&mut result_bytes);
            from_account.balance = result_bytes;
        }

        if let Some(to_account) = self.accounts.get_mut(&to) {
            let mut result_bytes = [0u8; 32];
            new_to_balance.to_big_endian(&mut result_bytes);
            to_account.balance = result_bytes;
        }

        Ok(())
    }

    // Load a 32 bytes word from storage of a specific account
    pub fn sload(&self, account: [u8; 20], slot: [u8; 32]) -> Result<[u8; 32], ExecutionError> {
        match self.accounts.get(&account) {
            Some(account_state) => match account_state.storage.get(&slot) {
                Some(value) => Ok(*value),
                None => Ok([0u8; 32]),
            },
            None => Ok([0u8; 32]),
        }
    }

    // Store a 32 bytes word in storage of a specific account
    pub fn sstore(
        &mut self,
        account: [u8; 20],
        slot: [u8; 32],
        value: [u8; 32],
    ) -> Result<(), ExecutionError> {
        match self.accounts.get_mut(&account) {
            Some(account_state) => {
                account_state.storage.insert(slot, value);
                Ok(())
            }
            None => Err(ExecutionError::AccountNotFound),
        }
    }

    // Get the code of an account
    pub fn get_code_at(&self, address: [u8; 20]) -> Result<&Vec<u8>, ExecutionError> {
        match self.accounts.get(&address) {
            Some(account_state) => {
                let code_hash = account_state.code_hash;
                self.get_code(code_hash)
            }
            None => Err(ExecutionError::AccountNotFound),
        }
    }

    // Store the code of an account
    pub fn put_code_at(&mut self, address: [u8; 20], code: Vec<u8>) -> Result<(), ExecutionError> {
        let code_hash = self.put_code(code);

        match self.accounts.get_mut(&address) {
            Some(account_state) => {
                account_state.code_hash = code_hash.to_owned();
                Ok(())
            }
            None => Err(ExecutionError::AccountNotFound),
        }
    }

    // Load contract code
    pub fn get_code(&self, code_hash: [u8; 32]) -> Result<&Vec<u8>, ExecutionError> {
        self.codes
            .get(&code_hash)
            .ok_or(ExecutionError::CodeNotFound)
    }

    // Store contract code and return its hash
    pub fn put_code(&mut self, code: Vec<u8>) -> [u8; 32] {
        let code_hash = keccak256(&code);
        self.codes.insert(code_hash, code);
        code_hash
    }

    pub fn debug_state(&self) {
        let border_line =
            "\n╔═══════════════════════════════════════════════════════════════════════════════════════════════════════╗";
        let footer_line =
            "╚═══════════════════════════════════════════════════════════════════════════════════════════════════════╝";

        // Print out the storage
        println!("\n{}", border_line.clone().truecolor(0, 150, 255));
        println!(
            "{} {:<101} {}",
            "║".truecolor(0, 150, 255),
            "Final storage".bright_yellow(),
            "║".truecolor(0, 150, 255)
        );
        println!("{}", footer_line.clone().truecolor(0, 150, 255));

        for (address, account_state) in &self.accounts {
            let border_line =
                "\n┌──────────────────────────────────────────────────────────────────────────┐";
            let footer_line =
                "└──────────────────────────────────────────────────────────────────────────┘\n";
            let hex: String = utils::debug::to_hex_address(address.to_owned());
            println!("\n{}", border_line.clone().truecolor(0, 255, 150));
            println!(
                "{} {:<94} {}",
                "│".truecolor(0, 255, 150),
                hex,
                "│".truecolor(0, 255, 150)
            );
            println!("{}", footer_line.clone().truecolor(0, 255, 150));
            println!("{:?}", account_state);
        }

        if self.accounts.is_empty() {
            println!("Empty EVM state");
        }
    }
}
