<img align="right" width="150" height="150" top="100" src="https://avatars.githubusercontent.com/u/5430905?s=200&v=4">

# EVM Rust Emulator

The EVM Rust Emulator is a simple in-memory Ethereum Virtual Machine (EVM) emulator written in Rust. It is designed to be a lightweight and easy-to-use tool for developers who want to test EVM bytecode execution directly in a command line or in a Rust crate, without using a full EVM node with his RPC to interact with a blockchain.

![Github action](https://github.com/Yashiru/evm-rs-emulator/workflows/CI/badge.svg)
![Test coverage](./coverage/badges/flat.svg)

***

## Rust crate## Install crate
```bash
cargo add evm-rs-emulator
```

## Usage
```rust
use evm_rs_emulator::Runner;

fn main() {
  let caller = [
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0xc4, 0x11, 0xe8,
  ];
  let origin: Option<[u8; 20]> = None;
  let address: Option<[u8; 20]> = None
  let value: Option<[u8; 32]> = None;
  let data: Option<Vec<u8>> = None;
  let bytecode: Vec<u8> = vec![0x60, 0xff, 0x60, 0xff];
  
  // Create a new interpreter
  let mut runner =
      Runner::new(caller, origin, address, value, data, None);

  // Run all the bytecode
  let _ = interpreter.interpret(bytecode, Some(255), true);

  // Or run the bytecode OPCODE by OPCODE
  runner.bytecode = bytecode;
  runner.debug_level = Some(255);
  // Run the first 3 opcodes
  let _ = runner.interpret_op_code(runner.bytecode[runner.pc]);
  let _ = runner.interpret_op_code(runner.bytecode[runner.pc]);
  let _ = runner.interpret_op_code(runner.bytecode[runner.pc]);
}
```

***

## 🚧 Warning 🚧
This project is currently experimental and subject to frequent changes as we are still working on stabilizing EVM emulation.
It has not been audited for security purposes and should not be used in production yet.

### Missing features (Contributions welcome ❤️)
- [ ] EVM gas usage (see [this branch](https://github.com/Yashiru/evm-rs-emulator/tree/feat/gas-usage))
- [ ] EVM gas price (display tx price using the fork)
- [ ] EVM gas limit
- [x] Mocked data with RPC
  - [ ] Block data
  - [x] Call to external contract
- [x] External crate features
  - [x] Deploy contract
  - [x] Call contract
  - [ ] Get logs

## Contributions

Contributions are welcome! Feel free to open an issue or submit a pull request if you have a way to improve this project.

[![Github repo](https://img.shields.io/badge/Github_repo-https://github.com/Yashiru/evm--rs--emulator-blue)](https://github.com/Yashiru/evm-rs-emulator)

To contribute to the EVM Rust Emulator, you will need to have Rust and Cargo installed on your system. 
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Once you have these tools installed, you can clone the project.
```bash
git clone https://github.com/Yashiru/evm-rs-emulator.git
```

To run the tests, you can use the following command.
```bash
cargo test
```

To run the coverage task
```bash
cargo make cov
```

## License

The underlying source code is free and unencumbered software released into the public domain. Check LICENSE file for more information.
