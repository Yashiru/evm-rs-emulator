use crate::core_module::utils::errors::ExecutionError;
use crate::core_module::runner::Runner;

// Colored output
use colored::*;

// Swap first and second element
pub fn swap1(runner: &mut Runner) -> Result<(), ExecutionError> {
    let result = unsafe { runner.stack.swap(1) };

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let words = result.unwrap();

        let hex1: String = words[0]
            .iter()
            .map(|b| match format!("{:02x}", b) {
                s if s == "00" => s.truecolor(80, 80, 80).to_string(),
                s => s.green().to_string(),
            })
            .collect::<Vec<String>>()
            .join(" ");

        let hex2: String = words[1]
            .iter()
            .map(|b| match format!("{:02x}", b) {
                s if s == "00" => s.truecolor(80, 80, 80).to_string(),
                s => s.green().to_string(),
            })
            .collect::<Vec<String>>()
            .join(" ");
        println!("{}{:<10} 🔁 [ {} ↔️  {} ]", "SWAP".bright_blue(), "1".magenta(), hex1, hex2);
    }

    // Increment PC
    runner.increment_pc(1)
}

// Swap first and third element
pub fn swap2(runner: &mut Runner) -> Result<(), ExecutionError> {
    let result = unsafe { runner.stack.swap(2) };

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let words = result.unwrap();

        let hex1: String = words[0]
            .iter()
            .map(|b| match format!("{:02x}", b) {
                s if s == "00" => s.truecolor(80, 80, 80).to_string(),
                s => s.green().to_string(),
            })
            .collect::<Vec<String>>()
            .join(" ");

        let hex2: String = words[1]
            .iter()
            .map(|b| match format!("{:02x}", b) {
                s if s == "00" => s.truecolor(80, 80, 80).to_string(),
                s => s.green().to_string(),
            })
            .collect::<Vec<String>>()
            .join(" ");
        println!("{}{:<10} 🔁 [ {} ↔️  {} ]", "SWAP".bright_blue(), "2".magenta(), hex1, hex2);
    }

    // Increment PC
    runner.increment_pc(1)
}

// Swap first and fourth element
pub fn swap3(runner: &mut Runner) -> Result<(), ExecutionError> {
    let result = unsafe { runner.stack.swap(3) };

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let words = result.unwrap();

        let hex1: String = words[0]
            .iter()
            .map(|b| match format!("{:02x}", b) {
                s if s == "00" => s.truecolor(80, 80, 80).to_string(),
                s => s.green().to_string(),
            })
            .collect::<Vec<String>>()
            .join(" ");

        let hex2: String = words[1]
            .iter()
            .map(|b| match format!("{:02x}", b) {
                s if s == "00" => s.truecolor(80, 80, 80).to_string(),
                s => s.green().to_string(),
            })
            .collect::<Vec<String>>()
            .join(" ");
        println!("{}{:<10} 🔁 [ {} ↔️  {} ]", "SWAP".bright_blue(), "3".magenta(), hex1, hex2);
    }

    // Increment PC
    runner.increment_pc(1)
}

// Swap first and fifth element
pub fn swap4(runner: &mut Runner) -> Result<(), ExecutionError> {
    let result = unsafe { runner.stack.swap(4) };

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let words = result.unwrap();

        let hex1: String = words[0]
            .iter()
            .map(|b| match format!("{:02x}", b) {
                s if s == "00" => s.truecolor(80, 80, 80).to_string(),
                s => s.green().to_string(),
            })
            .collect::<Vec<String>>()
            .join(" ");

        let hex2: String = words[1]
            .iter()
            .map(|b| match format!("{:02x}", b) {
                s if s == "00" => s.truecolor(80, 80, 80).to_string(),
                s => s.green().to_string(),
            })
            .collect::<Vec<String>>()
            .join(" ");
        println!("{}{:<10} 🔁 [ {} ↔️  {} ]", "SWAP".bright_blue(), "4".magenta(), hex1, hex2);
    }

    // Increment PC
    runner.increment_pc(1)
}

// Swap first and sixth element
pub fn swap5(runner: &mut Runner) -> Result<(), ExecutionError> {
    let result = unsafe { runner.stack.swap(5) };

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let words = result.unwrap();

        let hex1: String = words[0]
            .iter()
            .map(|b| match format!("{:02x}", b) {
                s if s == "00" => s.truecolor(80, 80, 80).to_string(),
                s => s.green().to_string(),
            })
            .collect::<Vec<String>>()
            .join(" ");

        let hex2: String = words[1]
            .iter()
            .map(|b| match format!("{:02x}", b) {
                s if s == "00" => s.truecolor(80, 80, 80).to_string(),
                s => s.green().to_string(),
            })
            .collect::<Vec<String>>()
            .join(" ");
        println!("{}{:<10} 🔁 [ {} ↔️  {} ]", "SWAP".bright_blue(), "5".magenta(), hex1, hex2);
    }

    // Increment PC
    runner.increment_pc(1)
}

// Swap first and seventh element
pub fn swap6(runner: &mut Runner) -> Result<(), ExecutionError> {
    let result = unsafe { runner.stack.swap(6) };

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let words = result.unwrap();

        let hex1: String = words[0]
            .iter()
            .map(|b| match format!("{:02x}", b) {
                s if s == "00" => s.truecolor(80, 80, 80).to_string(),
                s => s.green().to_string(),
            })
            .collect::<Vec<String>>()
            .join(" ");

        let hex2: String = words[1]
            .iter()
            .map(|b| match format!("{:02x}", b) {
                s if s == "00" => s.truecolor(80, 80, 80).to_string(),
                s => s.green().to_string(),
            })
            .collect::<Vec<String>>()
            .join(" ");
        println!("{}{:<10} 🔁 [ {} ↔️  {} ]", "SWAP".bright_blue(), "6".magenta(), hex1, hex2);
    }

    // Increment PC
    runner.increment_pc(1)
}

// Swap first and eighth element
pub fn swap7(runner: &mut Runner) -> Result<(), ExecutionError> {
    let result = unsafe { runner.stack.swap(7) };

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let words = result.unwrap();

        let hex1: String = words[0]
            .iter()
            .map(|b| match format!("{:02x}", b) {
                s if s == "00" => s.truecolor(80, 80, 80).to_string(),
                s => s.green().to_string(),
            })
            .collect::<Vec<String>>()
            .join(" ");

        let hex2: String = words[1]
            .iter()
            .map(|b| match format!("{:02x}", b) {
                s if s == "00" => s.truecolor(80, 80, 80).to_string(),
                s => s.green().to_string(),
            })
            .collect::<Vec<String>>()
            .join(" ");
        println!("{}{:<10} 🔁 [ {} ↔️  {} ]", "SWAP".bright_blue(), "7".magenta(), hex1, hex2);
    }

    // Increment PC
    runner.increment_pc(1)
}

// Swap first and ninth element
pub fn swap8(runner: &mut Runner) -> Result<(), ExecutionError> {
    let result = unsafe { runner.stack.swap(8) };

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let words = result.unwrap();

        let hex1: String = words[0]
            .iter()
            .map(|b| match format!("{:02x}", b) {
                s if s == "00" => s.truecolor(80, 80, 80).to_string(),
                s => s.green().to_string(),
            })
            .collect::<Vec<String>>()
            .join(" ");

        let hex2: String = words[1]
            .iter()
            .map(|b| match format!("{:02x}", b) {
                s if s == "00" => s.truecolor(80, 80, 80).to_string(),
                s => s.green().to_string(),
            })
            .collect::<Vec<String>>()
            .join(" ");
        println!("{}{:<10} 🔁 [ {} ↔️  {} ]", "SWAP".bright_blue(), "8".magenta(), hex1, hex2);
    }

    // Increment PC
    runner.increment_pc(1)
}

// Swap first and tenth element
pub fn swap9(runner: &mut Runner) -> Result<(), ExecutionError> {
    let result = unsafe { runner.stack.swap(9) };

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let words = result.unwrap();

        let hex1: String = words[0]
            .iter()
            .map(|b| match format!("{:02x}", b) {
                s if s == "00" => s.truecolor(80, 80, 80).to_string(),
                s => s.green().to_string(),
            })
            .collect::<Vec<String>>()
            .join(" ");

        let hex2: String = words[1]
            .iter()
            .map(|b| match format!("{:02x}", b) {
                s if s == "00" => s.truecolor(80, 80, 80).to_string(),
                s => s.green().to_string(),
            })
            .collect::<Vec<String>>()
            .join(" ");
        println!("{}{:<10} 🔁 [ {} ↔️  {} ]", "SWAP".bright_blue(), "9".magenta(), hex1, hex2);
    }

    // Increment PC
    runner.increment_pc(1)
}

// Swap first and eleventh element
pub fn swap10(runner: &mut Runner) -> Result<(), ExecutionError> {
    let result = unsafe { runner.stack.swap(10) };

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let words = result.unwrap();

        let hex1: String = words[0]
            .iter()
            .map(|b| match format!("{:02x}", b) {
                s if s == "00" => s.truecolor(80, 80, 80).to_string(),
                s => s.green().to_string(),
            })
            .collect::<Vec<String>>()
            .join(" ");

        let hex2: String = words[1]
            .iter()
            .map(|b| match format!("{:02x}", b) {
                s if s == "00" => s.truecolor(80, 80, 80).to_string(),
                s => s.green().to_string(),
            })
            .collect::<Vec<String>>()
            .join(" ");
        println!("{}{:<10} 🔁 [ {} ↔️  {} ]", "SWAP".bright_blue(), "10".magenta(), hex1, hex2);
    }

    // Increment PC
    runner.increment_pc(1)
}

// Swap first and twelfth element
pub fn swap11(runner: &mut Runner) -> Result<(), ExecutionError> {
    let result = unsafe { runner.stack.swap(11) };

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let words = result.unwrap();

        let hex1: String = words[0]
            .iter()
            .map(|b| match format!("{:02x}", b) {
                s if s == "00" => s.truecolor(80, 80, 80).to_string(),
                s => s.green().to_string(),
            })
            .collect::<Vec<String>>()
            .join(" ");

        let hex2: String = words[1]
            .iter()
            .map(|b| match format!("{:02x}", b) {
                s if s == "00" => s.truecolor(80, 80, 80).to_string(),
                s => s.green().to_string(),
            })
            .collect::<Vec<String>>()
            .join(" ");
        println!("{}{:<10} 🔁 [ {} ↔️  {} ]", "SWAP".bright_blue(), "11".magenta(), hex1, hex2);
    }

    // Increment PC
    runner.increment_pc(1)
}

// Swap first and thirteenth element
pub fn swap12(runner: &mut Runner) -> Result<(), ExecutionError> {
    let result = unsafe { runner.stack.swap(12) };

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let words = result.unwrap();

        let hex1: String = words[0]
            .iter()
            .map(|b| match format!("{:02x}", b) {
                s if s == "00" => s.truecolor(80, 80, 80).to_string(),
                s => s.green().to_string(),
            })
            .collect::<Vec<String>>()
            .join(" ");

        let hex2: String = words[1]
            .iter()
            .map(|b| match format!("{:02x}", b) {
                s if s == "00" => s.truecolor(80, 80, 80).to_string(),
                s => s.green().to_string(),
            })
            .collect::<Vec<String>>()
            .join(" ");
        println!("{}{:<10} 🔁 [ {} ↔️  {} ]", "SWAP".bright_blue(), "12".magenta(), hex1, hex2);
    }

    // Increment PC
    runner.increment_pc(1)
}

// Swap first and fourteenth element
pub fn swap13(runner: &mut Runner) -> Result<(), ExecutionError> {
    let result = unsafe { runner.stack.swap(13) };

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let words = result.unwrap();

        let hex1: String = words[0]
            .iter()
            .map(|b| match format!("{:02x}", b) {
                s if s == "00" => s.truecolor(80, 80, 80).to_string(),
                s => s.green().to_string(),
            })
            .collect::<Vec<String>>()
            .join(" ");

        let hex2: String = words[1]
            .iter()
            .map(|b| match format!("{:02x}", b) {
                s if s == "00" => s.truecolor(80, 80, 80).to_string(),
                s => s.green().to_string(),
            })
            .collect::<Vec<String>>()
            .join(" ");
        println!("{}{:<10} 🔁 [ {} ↔️  {} ]", "SWAP".bright_blue(), "13".magenta(), hex1, hex2);
    }

    // Increment PC
    runner.increment_pc(1)
}

// Swap first and fifteenth element
pub fn swap14(runner: &mut Runner) -> Result<(), ExecutionError> {
    let result = unsafe { runner.stack.swap(14) };

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let words = result.unwrap();

        let hex1: String = words[0]
            .iter()
            .map(|b| match format!("{:02x}", b) {
                s if s == "00" => s.truecolor(80, 80, 80).to_string(),
                s => s.green().to_string(),
            })
            .collect::<Vec<String>>()
            .join(" ");

        let hex2: String = words[1]
            .iter()
            .map(|b| match format!("{:02x}", b) {
                s if s == "00" => s.truecolor(80, 80, 80).to_string(),
                s => s.green().to_string(),
            })
            .collect::<Vec<String>>()
            .join(" ");
        println!("{}{:<10} 🔁 [ {} ↔️  {} ]", "SWAP".bright_blue(), "14".magenta(), hex1, hex2);
    }

    // Increment PC
    runner.increment_pc(1)
}

// Swap first and sixteenth element
pub fn swap15(runner: &mut Runner) -> Result<(), ExecutionError> {
    let result = unsafe { runner.stack.swap(15) };

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let words = result.unwrap();

        let hex1: String = words[0]
            .iter()
            .map(|b| match format!("{:02x}", b) {
                s if s == "00" => s.truecolor(80, 80, 80).to_string(),
                s => s.green().to_string(),
            })
            .collect::<Vec<String>>()
            .join(" ");

        let hex2: String = words[1]
            .iter()
            .map(|b| match format!("{:02x}", b) {
                s if s == "00" => s.truecolor(80, 80, 80).to_string(),
                s => s.green().to_string(),
            })
            .collect::<Vec<String>>()
            .join(" ");
        println!("{}{:<10} 🔁 [ {} ↔️  {} ]", "SWAP".bright_blue(), "15".magenta(), hex1, hex2);
    }

    // Increment PC
    runner.increment_pc(1)
}

// Swap first and seventeenth element
pub fn swap16(runner: &mut Runner) -> Result<(), ExecutionError> {
    let result = unsafe { runner.stack.swap(16) };

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug_level.is_some() && runner.debug_level.unwrap() >= 1 {
        let words = result.unwrap();

        let hex1: String = words[0]
            .iter()
            .map(|b| match format!("{:02x}", b) {
                s if s == "00" => s.truecolor(80, 80, 80).to_string(),
                s => s.green().to_string(),
            })
            .collect::<Vec<String>>()
            .join(" ");

        let hex2: String = words[1]
            .iter()
            .map(|b| match format!("{:02x}", b) {
                s if s == "00" => s.truecolor(80, 80, 80).to_string(),
                s => s.green().to_string(),
            })
            .collect::<Vec<String>>()
            .join(" ");
        println!("{}{:<10} 🔁 [ {} ↔️  {} ]", "SWAP".bright_blue(), "16".magenta(), hex1, hex2);
    }

    // Increment PC
    runner.increment_pc(1)
}
