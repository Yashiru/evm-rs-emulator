#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_interpret() {
        let mut interpreter = Interpreter::new();
        let bytecode = vec![
            Instruction::Push(1),
            Instruction::Push(2),
            Instruction::Add,
            Instruction::Halt,
        ];
        let result = interpreter.interpret(bytecode);
        assert_eq!(result, Ok(3));
    }
    
    #[test]
    fn test_interpret_error() {
        let mut interpreter = Interpreter::new();
        let bytecode = vec![
            Instruction::Push(1),
            Instruction::Add,
            Instruction::Halt,
        ];
        let result = interpreter.interpret(bytecode);
        assert!(result.is_err());
    }
}