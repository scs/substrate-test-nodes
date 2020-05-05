# DEPRECATED
This repository is no longer supported. Please consider using [substrate-node-template](https://github.com/scs/substrate-node-template) instead with is based on substrate v2.0

# Substrate Test Node

A substrate test node that is used for testing the [substrate-api-client](https://github.com/scs/substrate-api-client).

The following modifications have been added to the standard node-template:
* Added the contracts-module (`srml-contracts`)
* Added a minimalistic Kitty module (using custom storage structure)

## build from source
Given you have all [substrate prerequisites](https://substrate.dev/docs/en/getting-started/installing-substrate), you can just do
```
cargo build --release
```
then run with
```
./target/release/substrate-test-node --dev
```
