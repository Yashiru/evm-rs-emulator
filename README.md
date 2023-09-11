<img align="right" width="150" height="150" top="100" src="https://avatars.githubusercontent.com/u/5430905?s=200&v=4">

# EVM Rust Emulator

The EVM Rust Emulator is a simple in-memory Ethereum Virtual Machine (EVM) emulator written in Rust. It is designed to be a lightweight and easy-to-use tool for developers who want to test EVM bytecode execution directly in a Rust crate, without using a full EVM node with his RPC to interact with a blockchain.

![Github action](https://github.com/Yashiru/evm-rs-emulator/workflows/CI/badge.svg)
![Test coverage](./coverage/badges/flat.svg)

## Installation
Clone the sources:
```bash
git clone https://github.com/Yashiru/evm-rs-emulator.git && cd evm-rs-emulator
```

Run the installation script:
```bash
./install
```

### Uninstall
Remove the binary:
```bash 
sudo rm -rf /usr/local/bin/evm-rs
```

## Execute bytecode
Put the bytecode to run in `bytecode.bin` file and run the following command.
```bash
evm-rs path-to-my-bytecode-file
```

### Command Line Arguments
Below is the list of available command line arguments for the application:

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

- **--help**  
Display the help message, listing available arguments and their usage.

- **--version**  
Display the current version of the application.



## 🚧 Warning 🚧
This project is currently experimental and subject to frequent changes as we are still working on stabilizing EVM emulation.
It has not been audited for security purposes and should not be used in production yet.

### Missing features (Contributions welcome ❤️)
- [ ] EVM gas usage
- [ ] EVM gas price (display tx price using the fork)
- [ ] EVM gas limit
- [x] Mocked data with RPC
  - [ ] Block data
  - [x] Call to external contract
- [ ] External crate features
  - [ ] Deploy contract
  - [ ] Call contract
  - [ ] Get logs

## Contributions

To install the EVM Rust Emulator, you will need to have Rust and Cargo installed on your system. 
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
