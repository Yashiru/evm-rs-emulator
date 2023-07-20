use std::{collections::HashMap, fmt};

use ethers::prelude::*;
use ethers::{types::U256, utils::keccak256};

use crate::core_module::utils;

use super::utils::errors::ExecutionError;

// Colored output
use colored::*;

/* -------------------------------------------------------------------------- */
/*                             AccountState struct                            */
/* -------------------------------------------------------------------------- */

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

        writeln!(f, "  {}: {}", "Nonce".magenta(), self.nonce)?;
        writeln!(
            f,
            "  {}: {}",
            "Balance".magenta(),
            U256::from(self.balance).to_string()
        )?;
        writeln!(f, "  {}: {}", "Code Hash".magenta(), code_hash)?;
        write!(f, "  {}: ", "Storage".magenta())?;
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
            write!(f, "  {}", "Empty storage".red())?;
        }
        Ok(())
    }
}

/* -------------------------------------------------------------------------- */
/*                                 Log struct                                 */
/* -------------------------------------------------------------------------- */

pub struct Log {
    pub address: [u8; 20],
    pub topics: Vec<[u8; 32]>,
    pub data: Vec<u8>,
}

impl fmt::Debug for Log {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "{}: {}",
            "Address".magenta(),
            utils::debug::to_hex_address(self.address)
        )?;

        write!(f, "{}: ", "Topics".magenta())?;
        if !self.topics.is_empty() {
            for (idx, topic) in self.topics.iter().enumerate() {
                println!("\n┌────────────────────────────────────────────────────────────────────────────────────────────────────────┐");
                let hex: String = utils::debug::to_hex_string(topic.to_owned());
                println!("│ {}: {} {} │", "Topic".bright_blue(), idx, hex);
                println!("└────────────────────────────────────────────────────────────────────────────────────────────────────────┘");
            }
        } else {
            writeln!(f, "{}", "No topics".red())?;
        }

        writeln!(
            f,
            "{}: {}",
            "Data".magenta(),
            utils::debug::vec_to_hex_string(self.data.clone())
        )?;

        Ok(())
    }
}

/* -------------------------------------------------------------------------- */
/*                              EVM state struct                              */
/* -------------------------------------------------------------------------- */

#[derive(Debug)]
pub struct EvmState {
    pub accounts: HashMap<[u8; 20], AccountState>,
    pub codes: HashMap<[u8; 32], Vec<u8>>,
    pub logs: Vec<Log>,
    pub static_mode: bool,
    pub provider: Option<Provider<Http>>,
}

impl EvmState {
    pub fn new(fork_url: Option<String>) -> Self {
        Self {
            accounts: HashMap::new(),
            codes: HashMap::new(),
            logs: Vec::new(),
            static_mode: false,
            provider: if fork_url.is_some() {
                Some(Provider::<Http>::try_from(fork_url.unwrap()).unwrap())
            } else {
                None
            },
        }
    }

    // Transfer value from one account to another
    pub fn transfer(
        &mut self,
        from: [u8; 20],
        to: [u8; 20],
        value: [u8; 32],
    ) -> Result<(), ExecutionError> {
        // Check if static mode is enabled
        if self.static_mode {
            return Err(ExecutionError::StaticCallStateChanged);
        }

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

    pub fn sload(&mut self, account: [u8; 20], slot: [u8; 32]) -> Result<[u8; 32], ExecutionError> {
        match self.accounts.get(&account) {
            Some(account_state) => match account_state.storage.get(&slot) {
                Some(value) => Ok(*value),
                None => Ok([0u8; 32]),
            },
            None => {
                let provider = self.provider.as_ref();

                if provider.is_none() {
                    return Ok([0u8; 32]);
                }

                let contract_address = Address::from(account);
                let future =
                    provider
                        .unwrap()
                        .get_storage_at(contract_address, H256::from(&slot), None);

                // Block on the future and get the result
                let storage_result = tokio::runtime::Runtime::new()
                    .expect("Could not create a Runtime")
                    .block_on(future);

                match storage_result {
                    Ok(storage) => {
                        let storage_bytes = storage.to_fixed_bytes();

                        // Save the fetched storage data locally
                        if let Some(account_state) = self.accounts.get_mut(&account) {
                            account_state.storage.insert(slot, storage_bytes);
                        }

                        Ok(storage_bytes)
                    }
                    Err(_) => Ok([0u8; 32]),
                }
            }
        }
    }

    // Store a 32 bytes word in storage of a specific account
    pub fn sstore(
        &mut self,
        account: [u8; 20],
        slot: [u8; 32],
        value: [u8; 32],
    ) -> Result<(), ExecutionError> {
        // Check if static mode is enabled
        if self.static_mode {
            return Err(ExecutionError::StaticCallStateChanged);
        }

        match self.accounts.get_mut(&account) {
            Some(account_state) => {
                account_state.storage.insert(slot, value);
                Ok(())
            }
            None => Err(ExecutionError::AccountNotFound),
        }
    }

    // Get the code of an account
    pub fn get_code_at(&mut self, address: [u8; 20]) -> Result<&Vec<u8>, ExecutionError> {
        match self.accounts.get(&address) {
            Some(account_state) => {
                let code_hash = account_state.code_hash;
                self.get_code(code_hash)
            }
            None => {
                let provider = self.provider.as_ref();

                if provider.is_none() {
                    return Err(ExecutionError::CodeNotFound);
                }

                // Asynchronously fetch the code from the blockchain
                let contract_address = Address::from(address);

                let future = provider.unwrap().get_code(contract_address, None);

                // Block on the future and get the result
                let code_result = tokio::runtime::Runtime::new()
                    .expect("Could not create a Runtime")
                    .block_on(future);

                match code_result {
                    Ok(code) => {
                        let code_hash = keccak256(&code.0);
                        if let Some(account) = self.accounts.get_mut(&address) {
                            account.code_hash = code_hash;
                        }
                        self.codes.insert(code_hash, code.to_vec());
                        Ok(&self.codes[&code_hash])
                    }
                    Err(_) => Err(ExecutionError::CodeNotFound),
                }
            }
        }
    }

    // Store the code of an account
    pub fn put_code_at(&mut self, address: [u8; 20], code: Vec<u8>) -> Result<(), ExecutionError> {
        let code_hash = self.put_code(code)?;

        match self.accounts.get_mut(&address) {
            Some(account_state) => {
                account_state.code_hash = code_hash.to_owned();
                Ok(())
            }
            None => Err(ExecutionError::AccountNotFound),
        }
    }

    // Load contract code
    fn get_code(&self, code_hash: [u8; 32]) -> Result<&Vec<u8>, ExecutionError> {
        self.codes
            .get(&code_hash)
            .ok_or(ExecutionError::CodeNotFound)
    }

    // Store contract code and return its hash
    fn put_code(&mut self, code: Vec<u8>) -> Result<[u8; 32], ExecutionError> {
        // Check if static mode is enabled
        if self.static_mode {
            return Err(ExecutionError::StaticCallStateChanged);
        }

        let code_hash = keccak256(&code);
        self.codes.insert(code_hash, code);
        Ok(code_hash)
    }

    pub fn debug_state(&mut self) {
        let border_line =
            "\n╔═══════════════════════════════════════════════════════════════════════════════════════════════════════╗";
        let footer_line =
            "╚═══════════════════════════════════════════════════════════════════════════════════════════════════════╝";

        // Print out the storage
        println!("\n{}", border_line.clone().red());
        println!(
            "{} {:<101} {}",
            "║".red(),
            "Final storage".yellow(),
            "║".red()
        );
        println!("{}", footer_line.clone().red());

        // ... other code ...

        // Create a vector of all addresses
        let addresses: Vec<_> = self.accounts.keys().cloned().collect();

        // Iterate over the vector of addresses
        for address in addresses {
            let account_state = &self.accounts[&address];
            let hex: String = utils::debug::to_hex_address(address.to_owned());
            println!("{}", hex.blue());
            println!("{:?}", account_state);
            let code_hash = account_state.code_hash;
            if code_hash != [0u8; 32] {
                let code = self.get_code_at(address.to_owned()).unwrap();
                let code_hex: String = utils::debug::vec_to_hex_string(code.to_owned());
                println!("  {}: {}", "Code".magenta(), code_hex);
            }
            println!("\n");
        }

        if self.accounts.is_empty() {
            println!("Empty EVM state");
        }
    }
}
