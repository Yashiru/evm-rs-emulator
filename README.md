<img align="right" width="150" height="150" top="100" src="https://avatars.githubusercontent.com/u/5430905?s=200&v=4">

# EVM Rust Emulator

The EVM Rust Emulator is a simple in-memory Ethereum Virtual Machine (EVM) emulator written in Rust. It is designed to be a lightweight and easy-to-use tool for developers who want to test EVM bytecode execution directly in a command line or in a Rust crate, without using a full EVM node with his RPC to interact with a blockchain.

![Github action](https://github.com/Yashiru/evm-rs-emulator/workflows/CI/badge.svg)
![Test coverage](./coverage/badges/flat.svg)

***
<details>
  <summary>
    <h2>Rust crate</h2>
    <p>👉 Use the evm-rs-emulator crate in your rust project.</p>
  </summary>

  ## Install crate
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
  let address: Option<[u8; 20]> = None;
  let value: Option<[u8; 32]> = None;
  let data: Option<Vec<u8>> = None;
  let bytecode: Vec<u8> = vec![0x60, 0xff, 0x60, 0xff];
  
  // Create a new interpreter
  let mut runner =
      Runner::new(caller, origin, address, value, data, None);

  // Run all the bytecode
  let _ = runner.interpret(bytecode.clone(), Some(255), true);

  // Or run the bytecode OPCODE by OPCODE
  runner.bytecode = bytecode;
  runner.debug_level = Some(255);
  // Run the first 3 opcodes
  let _ = runner.interpret_op_code(runner.bytecode[runner.pc]);
  let _ = runner.interpret_op_code(runner.bytecode[runner.pc]);
  let _ = runner.interpret_op_code(runner.bytecode[runner.pc]);
}
  ```
</details>

<details>
  <summary>
    <h2>Command line interface</h2>
    <p>👉 Use the evm-rs-emulator as a CLI.</p>
  </summary>

  ## Installation (UNIX-like OS)
  Run this command to install evm-rs:
  ```bash
  git clone https://github.com/Yashiru/evm-rs-emulator.git && \
    cd evm-rs-emulator && \
    ./install && cd .. && \
    rm -rf evm-rs-emulator
  ```
  > [!NOTE]  
  > The install script runs sudo commands.

  ### Uninstall
  Remove the binary:
  ```bash 
  sudo rm -rf /usr/local/bin/evm-rs
  ```

  ### Execute bytecode
  You can run raw bytecode or give a file containing the bytecode in parameter. 
  
  > [!NOTE]  
  > When running some bytecode, the bytecode is deployed at a contract address and the contract is called with the provided data.

  ```bash
  # Run bytecode in a file
  evm-rs ./bytecode.bin

  # Run raw bytecode
  evm-rs 0x60ff60ff
  ```

  #### Command Line Arguments

  - **--address**  
  Set the address of the contract containing the provided bytecode.

  - **--caller**  
  Set the caller address.

  - **--data**  
  Set the data to be passed to the contract.

  - **--origin**  
  Set the origin address.

  - **--value**  
  Set the value to be sent to the contract.

  - **--fork**  
  Set the fork RPC url to be used when local storage reads return nothing.

  - **--help**  
  Display the help message, listing available arguments and their usage.

  - **--version**  
  Display the current version of the CLI.
</details>

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
