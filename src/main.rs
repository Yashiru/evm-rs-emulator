mod core_module;

fn main() {
    // Create a new interpreter
    let mut interpreter = core_module::interpreter::Interpreter::new();

    // Create a new bytecode
    let bytecode = vec![0x01, 0x02, 0x03];

    // Interpret the bytecode
    let result = interpreter.interpret(bytecode);

    match result {
        Ok(_) => {
            println!("Interpretation successful!");
        },
        Err(_) => {
            println!("Interpretation failed!");
        }
    }
}