<img align="right" width="150" height="150" top="100" src="https://avatars.githubusercontent.com/u/5430905?s=200&v=4">

# EVM Rust Emulator

The EVM Rust Emulator is a simple in-memory Ethereum Virtual Machine (EVM) emulator written in Rust. It is designed to be a lightweight and easy-to-use tool for developers who want to test EVM bytecode execution directly in a Rust crate, without using a full EVM node with his RPC to interact with a blockchain.

![Github action](https://github.com/Yashiru/evm-rs-emulator/workflows/CI/badge.svg)

## Run some bytecode
Put the bytecode to run in `bytecode.bin` file and run the following command.
```bash
cargo run
```

## Disclaimer
This crate is in development and it is at this point only a bytecode interpreter with a full EVM emulation. It require a lib module with all external accessible features to make this crate a real lib crate that can be used as an external crate in other projects.

### Missing features (Contributions welcome ❤️)
- [ ] Docs 🤫
- [ ] EVM precompiles
- [ ] EVM gas usage
- [ ] EVM gas refund
- [ ] EVM gas price
- [ ] EVM gas limit
- [ ] Mocked data with RPC
  - [ ] Block data
  - [ ] Call to external contract
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

## License

The underlying source code is free and unencumbered software released into the public domain. Check LICENSE file for more information.
