use crate::core_module::runner::Runner;

use super::{errors::ExecutionError, bytes::u64_to_u256_array};

// Get address balance 
pub fn get_balance(address: [u8; 20], runner: &mut Runner) -> Result<[u8; 32], ExecutionError> {
    let balance = runner.state.accounts.get(&address).map(|account| account.balance).ok_or(ExecutionError::AccountNotFound);
    Ok(balance?)
}

// Get address nonce 
pub fn get_nonce(address: [u8; 20], runner: &mut Runner) -> Result<[u8; 32], ExecutionError> {
    let nonce = runner.state.accounts.get(&address).map(|account| account.nonce).ok_or(ExecutionError::AccountNotFound);
    Ok(u64_to_u256_array(nonce?))
}

pub fn init_account(address: [u8; 20], runner: &mut Runner) -> Result<(), ExecutionError> {
    let account = runner.state.accounts.get_mut(&address);
    match account {
        Some(_) => Ok(()),
        None => {
            runner.state.accounts.insert(address, super::super::state::AccountState {
                nonce: 0,
                balance: [0; 32],
                storage: std::collections::HashMap::new(),
                code_hash: [0u8; 32],
            });
            increment_nonce(address, runner)
        },
    }
}

pub fn delete_account(address: [u8; 20], runner: &mut Runner) -> Result<(), ExecutionError> {
    runner.state.accounts.remove(&address);
    Ok(())
}

// Increment nonce
pub fn increment_nonce(address: [u8; 20], runner: &mut Runner) -> Result<(), ExecutionError> {
    let result = runner.state.accounts.get_mut(&address);
    let nonce = match result {
        Some(account) => account,
        None => {
            println!("{:?}", ExecutionError::AccountNotFound);
            return Err(ExecutionError::AccountNotFound)
        },
    };
    nonce.nonce += 1;
    Ok(())
}