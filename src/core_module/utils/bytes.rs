use ethers::types::U256;

// Colored output
use colored::*;

/// Pad a [u8] with no particular length to 32 bytes to return a [u8; 32]
/// It adds zero padding to the left
///
/// # Arguments
///
/// * `bytes` - The [u8] to pad
///
/// # Returns
///
/// Returns a [u8; 32] with the padded bytes
pub fn pad_left(bytes: &[u8]) -> [u8; 32] {
    let mut padded = [0u8; 32];
    padded[32 - bytes.len()..].copy_from_slice(bytes);
    padded
}

/// Pad a [u8] with no particular length to 32 bytes to return a [u8; 32]
/// It adds zeros to the right of the [u8] instead of the left
///
/// # Arguments
///
/// * `bytes` - The [u8] to pad
///
/// # Returns
///
/// Returns a [u8; 32] with the padded bytes
pub fn _pad_right(bytes: &[u8]) -> [u8; 32] {
    let mut padded = [0u8; 32];
    padded[..bytes.len()].copy_from_slice(bytes);
    padded
}

/// Convert a [u8; 32] to a [u8; 20]
///
/// # Arguments
///
/// * `bytes` - The [u8; 32] to convert
///
/// # Returns
///
/// Returns a [u8; 20] with the address
pub fn bytes32_to_address(bytes: &[u8; 32]) -> [u8; 20] {
    let mut address = [0u8; 20];
    address.copy_from_slice(&bytes[12..]);
    address
}

/// Remove zero padding from a [u8; 32] to return only the relevant bytes
///
/// # Arguments
///
/// * `arr` - The [u8; 32] to remove zero padding from
///
/// # Returns
///
/// Returns a [u8] with the zero padding removed
pub fn strip_zero_padding(arr: &[u8; 32]) -> &[u8] {
    let start = arr.iter().position(|&x| x != 0).unwrap_or(0);
    let end = arr.iter().rposition(|&x| x != 0).unwrap_or(0) + 1;
    &arr[start..end]
}

/// Convert a u64 to a [u8; 32]
///
/// # Arguments
///
/// * `n` - The u64 to convert
///
/// # Returns
///
/// Returns a [u8; 32] with the u64 converted to bytes
pub fn u64_to_u256_array(n: u64) -> [u8; 32] {
    let uint256 = U256::from(n);
    let mut bytes = [0u8; 32];
    uint256.to_big_endian(&mut bytes);
    bytes
}

/// Convert a [u8; 32] to a u64
///
/// # Arguments
///
/// * `arr` - The [u8; 32] to convert
///
/// # Returns
///
/// Returns a u64 with the [u8; 32] converted to a u64
pub fn _hex_string_to_bytes(hex: &str) -> Vec<u8> {
    match hex::decode(hex) {
        Ok(bytes) => bytes,
        Err(e) => {
            panic!("Error: {}", e.to_string().red());
        }
    }
}

/* -------------------------------------------------------------------------- */
/*                               Math operations                              */
/* -------------------------------------------------------------------------- */

/// Add a u64 to a u256 expressed as a [u8; 32]
///
/// # Arguments
///
/// * `arr` - The [u8; 32] to add to
///
/// * `number` - The u64 to add
///
/// # Returns
///
/// Returns a [u8; 32] with the u64 added to the [u8; 32]
pub fn _add(arr: [u8; 32], number: u64) -> [u8; 32] {
    // Convert the [u8; 32] into U256
    let num = U256::from_big_endian(&arr);

    // Add
    let num = num + U256::from(number);

    // Convert back to [u8; 32]
    let mut result = [0u8; 32];
    num.to_big_endian(&mut result);

    result
}
