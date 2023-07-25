// Colored output
use colored::*;

/// Convert a [u8; 32] to a string of hex bytes separated by spaces, with revelent bytes in green.
///
/// # Arguments
///
/// * `bytes` - The [u8; 32] to convert
///
/// # Returns
///
/// Returns a String with the colored hex bytes separated by spaces
///
/// # Example
///
/// ```
/// use core_module::utils::debug::to_hex_string;
///
/// let bytes = [15u8; 32];
/// let hex_string = to_hex_string(bytes);
///
/// assert_eq!(hex_string, "ff ff ff ff ff ff ff ff ff ff ff ff ff ff ff ff ff ff ff ff ff ff ff ff ff ff ff ff ff ff ff ff");
pub fn to_hex_string(bytes: [u8; 32]) -> String {
    bytes
        .iter()
        .map(|b| match format!("{:02x}", b) {
            s if s == "00" => s.truecolor(80, 80, 80).to_string(),
            s => s.green().to_string(),
        })
        .collect::<Vec<String>>()
        .join(" ")
}

/// Convert a [u8; 20] to a string of hex bytes prefixed with a '0x'.
///
/// # Arguments
///
/// * `bytes` - The [u8; 20] to convert
///
/// # Returns
///
/// Returns a String with the hex bytes prefixed with '0x'
///
/// # Example
///
/// ```
/// use core_module::utils::debug::to_hex_address;
///
/// let bytes = [15u8; 20];
/// let hex_address = to_hex_address(bytes);
///
/// assert_eq!(hex_address, "0xffffffffffffffffffffffffffffffffffffffff");
/// ```
pub fn to_hex_address(bytes: [u8; 20]) -> String {
    format!(
        "0x{}",
        bytes
            .iter()
            .map(|b| match format!("{:02x}", b) {
                s => s.to_string(),
            })
            .collect::<Vec<String>>()
            .join("")
    )
}

/// Convert a Vec<u8> to a string of hex bytes separated by spaces, with revelent bytes in green.
///
/// # Arguments
///
/// * `bytes` - The Vec<u8> to convert
///
/// # Returns
///
/// Returns a String with the colored hex bytes separated by spaces
///
/// # Example
///
/// ```
/// use core_module::utils::debug::to_hex_string;
///
/// let bytes = [15u8; 10].to_vec();
/// let hex_string = to_hex_string(bytes);
///
/// assert_eq!(hex_string, "ff ff ff ff ff ff ff ff ff ff");
pub fn vec_to_hex_string(bytes: Vec<u8>) -> String {
    bytes
        .iter()
        .map(|b| match format!("{:02x}", b) {
            s if s == "00" => s.truecolor(80, 80, 80).to_string(),
            s => s.green().to_string(),
        })
        .collect::<Vec<String>>()
        .join(" ")
}
