# Stylus ERC-20 token example (with OpenZeppelin)

This is an example ERC-20 token contract written in Rust that uses OpenZeppelin's [Rust contract library](https://github.com/OpenZeppelin/rust-contracts-stylus) and is deployed with a constructor thanks to OpenZeppelin's [koba crate](https://crates.io/crates/koba).

OpenZeppelin's Koba transforms the wasm code generated on a Stylus project into bytecode. It then compiles a basic Solidity constructor and concatenates both bytecodes. The resulting bytecode can be then sent to the blockchain to effectively deploy the contract and automatically call the constructor.

Note that this project doesn't use cargo-stylus. In fact, you might run into issues when using `cargo stylus check` after building the contract with `cargo build`.

## Requirements

- Install [solc](https://docs.soliditylang.org/en/latest/installing-solidity.html)

## Instructions to build and deploy

From the root of the project

```shell
cargo build --release --target wasm32-unknown-unknown
```

Make a copy of the .env file and fill the environemnt variables

```shell
cp .env.example .env
```

Run the deployment script

```shell
cargo run -p stylus-erc20-oz-deployer
```

To test, run the following script

```shell
CONTRACT_ADDRESS=<YOUR_CONTRACT_ADDRESS> ./scripts/test.sh
```
