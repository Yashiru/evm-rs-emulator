use crate::core_module::runner::Runner;

use super::{bytes::u64_to_u256_array, errors::ExecutionError};

/// Get the balance of an address.
///
/// # Arguments
///
/// * `address` - An array of 20 bytes representing the address to get the balance of.
/// * `runner` - A mutable reference to a `Runner` instance.
///
/// # Returns
///
/// Returns a `Result` containing an array of 32 bytes representing the balance of the address,
/// or an `ExecutionError` if the account is not found.
pub fn get_balance(address: [u8; 20], runner: &mut Runner) -> Result<[u8; 32], ExecutionError> {
    let balance = runner
        .state
        .accounts
        .get(&address)
        .map(|account| account.balance)
        .ok_or(ExecutionError::AccountNotFound);
    Ok(balance?)
}

/// Get the nonce of an Ethereum address.
///
/// # Arguments
///
/// * `address` - An array of 20 bytes representing the Ethereum address.
/// * `runner` - A mutable reference to the `Runner` struct.
///
/// # Returns
///
/// An array of 32 bytes representing the nonce of the address.
///
/// # Errors
///
/// Returns an `ExecutionError` if the account associated with the address is not found.
pub fn get_nonce(address: [u8; 20], runner: &mut Runner) -> Result<[u8; 32], ExecutionError> {
    let nonce = runner
        .state
        .accounts
        .get(&address)
        .map(|account| account.nonce)
        .ok_or(ExecutionError::AccountNotFound);
    Ok(u64_to_u256_array(nonce?))
}

/// Initializes an account with the given address in the EVM state.
///
/// If the account already exists, this function does nothing and returns `Ok(())`.
/// Otherwise, a new account is created with a nonce of 0, a balance of 0, an empty storage,
/// and a code hash of all zeros. The account is then inserted into the EVM state and its nonce is incremented.
///
/// # Arguments
///
/// * `address` - The address of the account to initialize.
/// * `runner` - A mutable reference to the `Runner` struct representing the EVM state.
///
/// # Errors
///
/// Returns an `ExecutionError` if there was an error incrementing the account's nonce.
pub fn init_account(address: [u8; 20], runner: &mut Runner) -> Result<(), ExecutionError> {
    let account = runner.state.accounts.get_mut(&address);
    match account {
        Some(_) => Ok(()),
        None => {
            runner.state.accounts.insert(
                address,
                super::super::state::AccountState {
                    nonce: 0,
                    balance: [0; 32],
                    storage: std::collections::HashMap::new(),
                    code_hash: [0u8; 32],
                },
            );
            increment_nonce(address, runner)
        }
    }
}

/// Deletes the account with the given address from the state.
///
/// # Arguments
///
/// * `address` - The address of the account to be deleted.
/// * `runner` - A mutable reference to the `Runner` instance.
///
/// # Returns
///
/// Returns `Ok(())` if the account was successfully deleted, otherwise returns an `ExecutionError`.
pub fn delete_account(address: [u8; 20], runner: &mut Runner) -> Result<(), ExecutionError> {
    runner.state.accounts.remove(&address);
    Ok(())
}

/// Increments the nonce of the account with the given address in the runner's state.
///
/// # Arguments
///
/// * `address` - A 20-byte array representing the address of the account to increment the nonce of.
/// * `runner` - A mutable reference to the `Runner` instance containing the state to modify.
///
/// # Errors
///
/// Returns an `ExecutionError` if the account with the given address is not found in the state.
pub fn increment_nonce(address: [u8; 20], runner: &mut Runner) -> Result<(), ExecutionError> {
    let result = runner.state.accounts.get_mut(&address);
    let nonce = match result {
        Some(account) => account,
        None => {
            println!("{:?}", ExecutionError::AccountNotFound);
            return Err(ExecutionError::AccountNotFound);
        }
    };
    nonce.nonce += 1;
    Ok(())
}
