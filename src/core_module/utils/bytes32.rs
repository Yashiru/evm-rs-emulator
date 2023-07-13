use ethers::types::U256;

// Pad a [u8] with no particular length to 32 bytes to return a [u8; 32]
pub fn pad_to_32_bytes(bytes: &[u8]) -> [u8; 32] {
    let mut padded = [0u8; 32];
    padded[32 - bytes.len()..].copy_from_slice(bytes);
    padded
}

// Remove zero padding from a [u8; 32]
pub fn strip_zero_padding(arr: &[u8; 32]) -> &[u8] {
    let start = arr.iter().position(|&x| x != 0).unwrap_or(0);
    let end = arr.iter().rposition(|&x| x != 0).unwrap_or(0) + 1;
    &arr[start..end]
}

// Convert a u64 to a [u8; 32]
pub fn u64_to_u256_array(n: u64) -> [u8; 32] {
    let uint256 = U256::from(n);
    let mut bytes = [0u8; 32];
    uint256.to_big_endian(&mut bytes);
    bytes
}

/* -------------------------------------------------------------------------- */
/*                               Math operations                              */
/* -------------------------------------------------------------------------- */

pub fn add(arr: [u8; 32], number: u64) -> [u8; 32] {
    // Convert the [u8; 32] into U256
    let num = U256::from_big_endian(&arr);

    // Add
    let num = num + U256::from(number);

    // Convert back to [u8; 32]
    let mut result = [0u8; 32];
    num.to_big_endian(&mut result);

    result
}