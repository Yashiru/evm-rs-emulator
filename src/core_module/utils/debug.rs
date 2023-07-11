// Colored output
use colored::*;

pub fn to_hex_string(bytes: [u8; 32]) -> String {
    bytes.iter()
    .map(|b| match format!("{:02x}", b) {
        s if s == "00" => s.truecolor(80, 80, 80).to_string(),
        s => s.green().to_string(),
    })
    .collect::<Vec<String>>()
    .join(" ")
}