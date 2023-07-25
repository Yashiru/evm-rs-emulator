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

/// Represents the state of an account on the Ethereum Virtual Machine.
pub struct AccountState {
    /// The account's nonce, which is incremented each time a transaction is sent from the account.
    pub nonce: u64,
    /// The account's balance, represented as a 32-byte array.
    pub balance: [u8; 32],
    /// The account's storage, represented as a hashmap where the keys and values are both 32-byte arrays.
    pub storage: HashMap<[u8; 32], [u8; 32]>,
    /// The hash of the account's code, represented as a 32-byte array.
    pub code_hash: [u8; 32],
}

/// Implements the Debug trait for the AccountState struct, which allows for the struct to be printed in a formatted way.
/// The function prints the nonce, balance, code hash, and storage of the account state.
/// If the code hash is empty, it prints "Empty code" instead of the hash.
/// For each storage slot and value, it prints them in a formatted way.
/// If the storage is empty, it prints "Empty storage".
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

/// Represents a log entry in the Ethereum Virtual Machine (EVM) state.
pub struct Log {
    /// The address of the contract that generated the log.
    pub address: [u8; 20],
    /// The topics associated with the log.
    pub topics: Vec<[u8; 32]>,
    /// The data associated with the log.
    pub data: Vec<u8>,
}

/// Implements the Debug trait for the Log struct, which allows for the struct to be printed in a formatted way.
/// The function writes the address, topics, and data of the Log struct to the provided formatter.
/// If the topics vector is not empty, it prints each topic in a formatted way.
/// Otherwise, it prints "No topics".
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

/// Represents the state of the Ethereum Virtual Machine (EVM).
#[derive(Debug)]
pub struct EvmState {
    /// A mapping of account addresses to their respective account states.
    pub accounts: HashMap<[u8; 20], AccountState>,
    /// A mapping of code hashes to their respective code.
    pub codes: HashMap<[u8; 32], Vec<u8>>,
    /// A vector of logs generated during the execution of the EVM.
    pub logs: Vec<Log>,
    /// A flag indicating whether the EVM is in static mode or not.
    pub static_mode: bool,
    /// An optional provider for interacting with the Ethereum network.
    pub provider: Option<Provider<Http>>,
}

/// Implementation of the EVM state.
impl EvmState {
    /// Creates a new instance of the `State` struct.
    ///
    /// # Arguments
    ///
    /// * `fork_url` - An optional `String` representing the URL of the fork to use.
    ///
    /// # Returns
    ///
    /// A new instance of the `State` struct.
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
    /// Transfers a given value from one account to another.
    ///
    /// # Arguments
    ///
    /// * `from` - An array of 20 bytes representing the address of the account to transfer from.
    /// * `to` - An array of 20 bytes representing the address of the account to transfer to.
    /// * `value` - An array of 32 bytes representing the value to transfer.
    ///
    /// # Errors
    ///
    /// Returns an `ExecutionError` if:
    ///
    /// * The static mode is enabled (static call).
    /// * The account to transfer from does not exist.
    /// * The account to transfer to does not exist.
    /// * The balance of the account to transfer from is insufficient.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the transfer was successful.
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

    /// Loads a 256-bit value from the storage of the given account at the given slot.
    /// If the account is not found in the emulator's local state, the storage value is fetched from the provider.
    /// If the provider is not set, or if the storage fetch fails, the function returns a zero-filled 256-bit value.
    ///
    /// # Arguments
    ///
    /// * `account` - An array of 20 bytes representing the address of the account to load from.
    /// * `slot` - An array of 32 bytes representing the slot to load from.
    ///
    /// # Errors
    ///
    /// Never return an error, but returns a 32-byte array of zero instead.
    ///
    /// # Returns
    ///
    /// Returns a 32-byte array representing the value at the given slot.
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
    /// Stores a value in the storage of an account.
    ///
    /// # Arguments
    ///
    /// * `account` - The address of the account to store the value in.
    /// * `slot` - The slot in the storage to store the value in.
    /// * `value` - The value to store in the specified slot.
    ///
    /// # Errors
    ///
    /// Returns an `ExecutionError` if the static mode is enabled or if the account is not found.
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

    /// Returns the code at the given address. If the code is not already in the state, it will be fetched from the blockchain using the provider.
    ///
    /// # Arguments
    ///
    /// * `address` - The address of the contract to get the code for.
    ///
    /// # Errors
    ///
    /// Returns an `ExecutionError` if the code is not found.
    ///
    /// # Returns
    ///
    /// Returns a reference to the Vec<u8> of the code.
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

    /// Stores the code of an account at the given address.
    ///
    /// # Arguments
    ///
    /// * `address` - A 20-byte array representing the address of the account.
    /// * `code` - A vector of bytes representing the code to be stored.
    ///
    /// # Errors
    ///
    /// Returns an `ExecutionError` if the account is not found.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the code is successfully stored.
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

    /// Returns a reference to the code associated with the given code hash.
    ///
    /// # Arguments
    ///
    /// * `code_hash` - A 32-byte array representing the hash of the code to retrieve.
    ///
    /// # Errors
    ///
    /// Returns an `ExecutionError::CodeNotFound` error if the code hash is not found in the state.
    ///
    /// # Returns
    ///
    /// Returns a reference to the code associated with the given code hash.
    fn get_code(&self, code_hash: [u8; 32]) -> Result<&Vec<u8>, ExecutionError> {
        self.codes
            .get(&code_hash)
            .ok_or(ExecutionError::CodeNotFound)
    }

    /// Store contract code and return its hash
    fn put_code(&mut self, code: Vec<u8>) -> Result<[u8; 32], ExecutionError> {
        // Check if static mode is enabled
        if self.static_mode {
            return Err(ExecutionError::StaticCallStateChanged);
        }

        let code_hash = keccak256(&code);
        self.codes.insert(code_hash, code);
        Ok(code_hash)
    }

    /// Print the state of the EVM
    /// This function is used for debugging purposes.
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
