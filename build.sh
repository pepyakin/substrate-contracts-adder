#!/bin/bash

cargo +nightly build --release --target=wasm32-unknown-unknown --verbose
wasm-build target substrate-contracts-adder --target-runtime=substrate --final=adder --save-raw=./target/adder-deployed.wasm --target wasm32-unknown-unknown
