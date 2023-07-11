mod core_module;

fn main() {
    // Create a new interpreter
    let mut interpreter = core_module::runner::Runner::new();

    // Create a new bytecode
    let bytecode = vec![0x60, 0x02, 0x60, 0xa, 0x05];


    // Interpret the bytecode
    let result = interpreter.interpret(bytecode, Some(true));

    match result {
        Ok(_) => {
            println!("Interpretation successful!");
        },
        Err(_) => { }
    }
}