use crate::core_module::utils;
use crate::core_module::utils::errors::ExecutionError;
use crate::core_module::runner::Runner;

// Colored output
use colored::*;

// Dup first element
pub fn dup1(runner: &mut Runner) -> Result<(), ExecutionError> {
    let result = unsafe { runner.stack.dup(1) };

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug.is_some() && runner.debug.unwrap() {
        let hex: String = utils::debug::to_hex_string(result.unwrap());
        println!("{}{} 👉 [ {} ]", "DUP".magenta(), "1".green(), hex);
    }

    // Increment PC
    runner.increment_pc(1)
}

// Dup second element
pub fn dup2(runner: &mut Runner) -> Result<(), ExecutionError> {
    let result = unsafe { runner.stack.dup(2) };

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug.is_some() && runner.debug.unwrap() {
        let hex: String = utils::debug::to_hex_string(result.unwrap());
        println!("{}{} 👉 [ {} ]", "DUP".magenta(), "2".green(), hex);
    }

    // Increment PC
    runner.increment_pc(1)
}

// Dup third element
pub fn dup3(runner: &mut Runner) -> Result<(), ExecutionError> {
    let result = unsafe { runner.stack.dup(3) };

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug.is_some() && runner.debug.unwrap() {
        let hex: String = utils::debug::to_hex_string(result.unwrap());
        println!("{}{} 👉 [ {} ]", "DUP".magenta(), "3".green(), hex);
    }

    // Increment PC
    runner.increment_pc(1)
}

// Dup fourth element
pub fn dup4(runner: &mut Runner) -> Result<(), ExecutionError> {
    let result = unsafe { runner.stack.dup(4) };

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug.is_some() && runner.debug.unwrap() {
        let hex: String = utils::debug::to_hex_string(result.unwrap());
        println!("{}{} 👉 [ {} ]", "DUP".magenta(), "4".green(), hex);
    }

    // Increment PC
    runner.increment_pc(1)
}

// Dup fifth element
pub fn dup5(runner: &mut Runner) -> Result<(), ExecutionError> {
    let result = unsafe { runner.stack.dup(5) };

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug.is_some() && runner.debug.unwrap() {
        let hex: String = utils::debug::to_hex_string(result.unwrap());
        println!("{}{} 👉 [ {} ]", "DUP".magenta(), "5".green(), hex);
    }

    // Increment PC
    runner.increment_pc(1)
}

// Dup sixth element
pub fn dup6(runner: &mut Runner) -> Result<(), ExecutionError> {
    let result = unsafe { runner.stack.dup(6) };

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug.is_some() && runner.debug.unwrap() {
        let hex: String = utils::debug::to_hex_string(result.unwrap());
        println!("{}{} 👉 [ {} ]", "DUP".magenta(), "6".green(), hex);
    }

    // Increment PC
    runner.increment_pc(1)
}

// Dup seventh element
pub fn dup7(runner: &mut Runner) -> Result<(), ExecutionError> {
    let result = unsafe { runner.stack.dup(7) };

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug.is_some() && runner.debug.unwrap() {
        let hex: String = utils::debug::to_hex_string(result.unwrap());
        println!("{}{} 👉 [ {} ]", "DUP".magenta(), "7".green(), hex);
    }

    // Increment PC
    runner.increment_pc(1)
}

// Dup eighth element
pub fn dup8(runner: &mut Runner) -> Result<(), ExecutionError> {
    let result = unsafe { runner.stack.dup(8) };

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug.is_some() && runner.debug.unwrap() {
        let hex: String = utils::debug::to_hex_string(result.unwrap());
        println!("{}{} 👉 [ {} ]", "DUP".magenta(), "8".green(), hex);
    }

    // Increment PC
    runner.increment_pc(1)
}

// Dup ninth element
pub fn dup9(runner: &mut Runner) -> Result<(), ExecutionError> {
    let result = unsafe { runner.stack.dup(9) };

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug.is_some() && runner.debug.unwrap() {
        let hex: String = utils::debug::to_hex_string(result.unwrap());
        println!("{}{} 👉 [ {} ]", "DUP".magenta(), "9".green(), hex);
    }

    // Increment PC
    runner.increment_pc(1)
}

// Dup tenth element
pub fn dup10(runner: &mut Runner) -> Result<(), ExecutionError> {
    let result = unsafe { runner.stack.dup(10) };

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug.is_some() && runner.debug.unwrap() {
        let hex: String = utils::debug::to_hex_string(result.unwrap());
        println!("{}{} 👉 [ {} ]", "DUP".magenta(), "10".green(), hex);
    }

    // Increment PC
    runner.increment_pc(1)
}

// Dup eleventh element
pub fn dup11(runner: &mut Runner) -> Result<(), ExecutionError> {
    let result = unsafe { runner.stack.dup(11) };

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug.is_some() && runner.debug.unwrap() {
        let hex: String = utils::debug::to_hex_string(result.unwrap());
        println!("{}{} 👉 [ {} ]", "DUP".magenta(), "11".green(), hex);
    }

    // Increment PC
    runner.increment_pc(1)
}

// Dup twelfth element
pub fn dup12(runner: &mut Runner) -> Result<(), ExecutionError> {
    let result = unsafe { runner.stack.dup(12) };

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug.is_some() && runner.debug.unwrap() {
        let hex: String = utils::debug::to_hex_string(result.unwrap());
        println!("{}{} 👉 [ {} ]", "DUP".magenta(), "12".green(), hex);
    }

    // Increment PC
    runner.increment_pc(1)
}

// Dup thirteenth element
pub fn dup13(runner: &mut Runner) -> Result<(), ExecutionError> {
    let result = unsafe { runner.stack.dup(13) };

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug.is_some() && runner.debug.unwrap() {
        let hex: String = utils::debug::to_hex_string(result.unwrap());
        println!("{}{} 👉 [ {} ]", "DUP".magenta(), "13".green(), hex);
    }

    // Increment PC
    runner.increment_pc(1)
}

// Dup fourteenth element
pub fn dup14(runner: &mut Runner) -> Result<(), ExecutionError> {
    let result = unsafe { runner.stack.dup(14) };

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug.is_some() && runner.debug.unwrap() {
        let hex: String = utils::debug::to_hex_string(result.unwrap());
        println!("{}{} 👉 [ {} ]", "DUP".magenta(), "14".green(), hex);
    }

    // Increment PC
    runner.increment_pc(1)
}

// Dup fifteenth element
pub fn dup15(runner: &mut Runner) -> Result<(), ExecutionError> {
    let result = unsafe { runner.stack.dup(15) };

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug.is_some() && runner.debug.unwrap() {
        let hex: String = utils::debug::to_hex_string(result.unwrap());
        println!("{}{} 👉 [ {} ]", "DUP".magenta(), "15".green(), hex);
    }

    // Increment PC
    runner.increment_pc(1)
}

// Dup sixteenth element
pub fn dup16(runner: &mut Runner) -> Result<(), ExecutionError> {
    let result = unsafe { runner.stack.dup(16) };

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    if runner.debug.is_some() && runner.debug.unwrap() {
        let hex: String = utils::debug::to_hex_string(result.unwrap());
        println!("{}{} 👉 [ {} ]", "DUP".magenta(), "16".green(), hex);
    }

    // Increment PC
    runner.increment_pc(1)
}

/* -------------------------------------------------------------------------- */
/*                                    TESTS                                   */
/* -------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dup1() {
        let mut runner = Runner::new();

        let _ = runner.interpret(vec![0x60, 0xff, 0x80], Some(true));
        assert_eq!(runner.stack.stack.len(), 2);
        assert_eq!(runner.stack.stack.first(), runner.stack.stack.last());
    }

    #[test]
    fn test_dup2() {
        let mut runner = Runner::new();

        let _ = runner.interpret(vec![0x60, 0xff, 0x60, 0x01, 0x81], Some(true));
        assert_eq!(runner.stack.stack.len(), 3);
        assert_eq!(runner.stack.stack.first(), runner.stack.stack.last());
    }

    #[test]
    fn test_dup3() {
        let mut runner = Runner::new();

        let _ = runner.interpret(vec![0x60, 0xff, 0x60, 0x01, 0x60, 0x01, 0x82], Some(true));
        assert_eq!(runner.stack.stack.len(), 4);
        assert_eq!(runner.stack.stack.first(), runner.stack.stack.last());
    }

    #[test]
    fn test_dup4() {
        let mut runner = Runner::new();

        let _ = runner.interpret(vec![0x60, 0xff, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x83], Some(true));
        assert_eq!(runner.stack.stack.len(), 5);
        assert_eq!(runner.stack.stack.first(), runner.stack.stack.last());
    }

    #[test]
    fn test_dup5() {
        let mut runner = Runner::new();

        let _ = runner.interpret(vec![0x60, 0xff, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x84], Some(true));
        assert_eq!(runner.stack.stack.len(), 6);
        assert_eq!(runner.stack.stack.first(), runner.stack.stack.last());
    }

    #[test]
    fn test_dup6() {
        let mut runner = Runner::new();

        let _ = runner.interpret(vec![0x60, 0xff, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x85], Some(true));
        assert_eq!(runner.stack.stack.len(), 7);
        assert_eq!(runner.stack.stack.first(), runner.stack.stack.last());
    }

    #[test]
    fn test_dup7() {
        let mut runner = Runner::new();

        let _ = runner.interpret(vec![0x60, 0xff, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x86], Some(true));
        assert_eq!(runner.stack.stack.len(), 8);
        assert_eq!(runner.stack.stack.first(), runner.stack.stack.last());
    }

    #[test]
    fn test_dup8() {
        let mut runner = Runner::new();

        let _ = runner.interpret(vec![0x60, 0xff, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x87], Some(true));
        assert_eq!(runner.stack.stack.len(), 9);
        assert_eq!(runner.stack.stack.first(), runner.stack.stack.last());
    }

    #[test]
    fn test_dup9() {
        let mut runner = Runner::new();

        let _ = runner.interpret(vec![0x60, 0xff, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x88], Some(true));
        assert_eq!(runner.stack.stack.len(), 10);
        assert_eq!(runner.stack.stack.first(), runner.stack.stack.last());
    }

    #[test]
    fn test_dup10() {
        let mut runner = Runner::new();

        let _ = runner.interpret(vec![0x60, 0xff, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x89], Some(true));
        assert_eq!(runner.stack.stack.len(), 11);
        assert_eq!(runner.stack.stack.first(), runner.stack.stack.last());
    }

    #[test]
    fn test_dup11() {
        let mut runner = Runner::new();

        let _ = runner.interpret(vec![0x60, 0xff, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x8a], Some(true));
        assert_eq!(runner.stack.stack.len(), 12);
        assert_eq!(runner.stack.stack.first(), runner.stack.stack.last());
    }

    #[test]
    fn test_dup12() {
        let mut runner = Runner::new();

        let _ = runner.interpret(vec![0x60, 0xff, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x8b], Some(true));
        assert_eq!(runner.stack.stack.len(), 13);
        assert_eq!(runner.stack.stack.first(), runner.stack.stack.last());
    }

    #[test]
    fn test_dup13() {
        let mut runner = Runner::new();

        let _ = runner.interpret(vec![0x60, 0xff, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x8c], Some(true));
        assert_eq!(runner.stack.stack.len(), 14);
        assert_eq!(runner.stack.stack.first(), runner.stack.stack.last());
    }

    #[test]
    fn test_dup14() {
        let mut runner = Runner::new();

        let _ = runner.interpret(vec![0x60, 0xff, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x8d], Some(true));
        assert_eq!(runner.stack.stack.len(), 15);
        assert_eq!(runner.stack.stack.first(), runner.stack.stack.last());
    }

    #[test]
    fn test_dup15() {
        let mut runner = Runner::new();

        let _ = runner.interpret(vec![0x60, 0xff, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x8e], Some(true));
        assert_eq!(runner.stack.stack.len(), 16);
        assert_eq!(runner.stack.stack.first(), runner.stack.stack.last());
    }

    #[test]
    fn test_dup16() {
        let mut runner = Runner::new();

        let _ = runner.interpret(vec![0x60, 0xff, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x8f], Some(true));
        assert_eq!(runner.stack.stack.len(), 17);
        assert_eq!(runner.stack.stack.first(), runner.stack.stack.last());
    }
}