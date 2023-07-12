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

pub fn to_colored_hex_string(bytes: [u8; 32], r: u8, g: u8, b: u8) -> String {
    bytes.iter()
    .map(|byte| match format!("{:02x}", byte) {
        s if s == "00" => s.truecolor(80, 80, 80).to_string(),
        s => s.truecolor(r, g, b).to_string(),
    })
    .collect::<Vec<String>>()
    .join(" ")
}

pub fn vec_to_hex_string(bytes: Vec<u8>) -> String {
    bytes.iter()
    .map(|b| match format!("{:02x}", b) {
        s if s == "00" => s.truecolor(80, 80, 80).to_string(),
        s => s.green().to_string(),
    })
    .collect::<Vec<String>>()
    .join(" ")
}