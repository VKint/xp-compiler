#!/bin/bash

cargo build

# Reads the output of the XP Compiler (the Solidity bytecode)
bytecode=`./target/debug/xp-compiler-test`

# Prints the bytecode to the console
echo "Bytecode of the Solidity Smart Contract:"
echo $bytecode

cd lib/xp-compiler-broadcast-test
npm install
npm run dev -- -b $bytecode -n $1 -k $2
